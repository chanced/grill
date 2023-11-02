use std::collections::{HashSet, VecDeque};

use jsonptr::{Pointer, Resolve};
use serde_json::Value;

use crate::{
    anymap::AnyMap,
    error::{CompileError, UnknownAnchorError},
    keyword::{
        cache::{Numbers, Values},
        Compile,
    },
    schema::{dialect::Dialects, Schemas},
    source::{Deserializers, Link, Resolvers, Sources},
    uri::TryIntoAbsoluteUri,
    AbsoluteUri, Interrogator, Key, Structure,
};

use super::{Anchor, CompiledSchema, Dialect, Ref, Reference};

#[derive(Clone, Debug)]
struct RefToResolve {
    referrer_key: Key,
    ref_: Ref,
}

#[derive(Debug)]
/// A pending schema to compile
struct SchemaToCompile {
    key: Option<Key>,
    uri: AbsoluteUri,
    path: Option<Pointer>,
    parent: Option<Key>,
    default_dialect_idx: usize,
    /// Errors are to be disregarded.
    continue_on_err: bool,
    /// (dependent_key, ref)
    ref_: Option<RefToResolve>,
}

pub(crate) struct Compiler<'i> {
    schemas: &'i mut Schemas,
    sources: &'i mut Sources,
    global_state: &'i mut AnyMap,
    dialects: &'i Dialects,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
    numbers: &'i mut Numbers,
    values: &'i mut Values,
    validate: bool,
}
#[allow(clippy::too_many_arguments)]
impl<'i> Compiler<'i> {
    pub(crate) fn new(interrogator: &'i mut Interrogator, validate: bool) -> Self {
        Self {
            schemas: &mut interrogator.schemas,
            sources: &mut interrogator.sources,
            global_state: &mut interrogator.state,
            dialects: &interrogator.dialects,
            deserializers: &interrogator.deserializers,
            resolvers: &interrogator.resolvers,
            numbers: &mut interrogator.numbers,
            values: &mut interrogator.values,
            validate,
        }
    }
    pub(crate) async fn compile(mut self, uri: AbsoluteUri) -> Result<Key, CompileError> {
        let mut q = VecDeque::new();

        q.push_front(SchemaToCompile {
            key: None,
            uri: uri.clone(),
            parent: None,
            path: None,
            default_dialect_idx: self.dialects.default_index(),
            continue_on_err: false,
            ref_: None,
        });
        self.run(q).await?;
        Ok(self.schemas.get_key(&uri).unwrap())
    }

    pub(crate) async fn compile_all<I>(
        mut self,
        uris: I,
    ) -> Result<Vec<(AbsoluteUri, Key)>, CompileError>
    where
        I: IntoIterator,
        I::Item: TryIntoAbsoluteUri,
    {
        let uris = uris
            .into_iter()
            .map(TryIntoAbsoluteUri::try_into_absolute_uri)
            .collect::<Result<Vec<_>, _>>()?;
        let mut q = VecDeque::default();
        for uri in &uris {
            if uri == "http://localhost:1234/draft2020-12/root#/$defs/B" {
                println!("!!!!!!!!FOUND IT");
                dbg!(&uri);
            }

            q.push_back(SchemaToCompile {
                key: None,
                uri: uri.clone(),
                path: None,
                parent: None,
                default_dialect_idx: self.dialects.default_index(),
                continue_on_err: false,
                ref_: None,
            });
        }
        self.run(q).await?;
        Ok(uris.into_iter().map(|uri| self.compiled(uri)).collect())
    }

    fn compiled(&self, uri: AbsoluteUri) -> (AbsoluteUri, Key) {
        let key = self.schemas.get_key(&uri).unwrap();
        (uri, key)
    }

    async fn run(&mut self, mut q: VecDeque<SchemaToCompile>) -> Result<(), CompileError> {
        while !q.is_empty() {
            let schema_to_compile = q.pop_front().unwrap();
            let result = self.compile_schema(schema_to_compile, &mut q).await;
            if let Err((continue_on_err, err)) = result {
                if continue_on_err {
                    continue;
                }
                return Err(err);
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    async fn compile_schema(
        &mut self,
        schema_to_compile: SchemaToCompile,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), (bool, CompileError)> {
        println!("##############################################################################################################");
        println!("COMPILE SCHEMA");
        dbg!(&schema_to_compile.uri);
        println!(
            "{}",
            self.sources
                .sandbox
                .as_ref()
                .unwrap()
                .index
                .iter()
                .filter(|(k, _)| !k.starts_with("https://json"))
                .map(|(k, v)| format!("\n{k}: {v:#?}"))
                .collect::<Vec<_>>()
                .join("\n")
        );
        // println!("##############################################################################################################");
        // println!("##############################################################################################################");
        // println!(
        //     "{}",
        //     q.iter()
        //         .map(|v| v.uri.to_string())
        //         .collect::<Vec<_>>()
        //         .join(", ")
        // );
        // println!("##############################################################################################################");
        // println!("##############################################################################################################");
        // println!("##############################################################################################################");

        let SchemaToCompile {
            key,
            uri,
            mut path,
            mut parent,
            default_dialect_idx,
            continue_on_err,
            ref_,
        } = schema_to_compile;

        let (link, src) = self
            .source(&uri)
            .await
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        let dialect_idx = self.dialect_idx(&src, default_dialect_idx);

        if let Some(key) = key.or(self.schemas.get_key(&uri)) {
            let schema = self.schemas.get(key, self.sources).unwrap();
            let path = schema.path.clone().into_owned();
            let uri = schema.absolute_uri().clone();
            return self
                .maybe_finalize(
                    key,
                    &uri,
                    path,
                    &src,
                    dialect_idx,
                    parent,
                    ref_,
                    continue_on_err,
                    q,
                )
                .map_err(|err| self.handle_err(err, continue_on_err, &uri));
        }

        self.validate(dialect_idx, &src)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        let (uri, id, mut uris) = identify(&link, &src, &self.dialects[dialect_idx])
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        let schema_to_compile = || {
            if uri == "http://localhost:1234/draft2020-12/root#/$defs/B" {
                println!("!!!!!!!!FOUND IT");
                dbg!(&uri);
            }

            SchemaToCompile {
                key: None,
                uri: uri.clone(),
                path: path.clone(),
                parent,
                default_dialect_idx,
                continue_on_err,
                ref_: ref_.clone(),
            }
        };
        if id.is_some() {
            path = Some(Pointer::default());
            parent = None;
        } else if parent.is_none() && path.is_none() && has_ptr_fragment(&uri) {
            return self.queue_pathed(schema_to_compile(), q);
        } else if is_anchored(&uri) {
            return self.queue_anchored(schema_to_compile(), q);
        } else if parent.is_none() && path.is_none() {
            // if the uri does not have a pointer fragment, then it should be
            // compiled as document root
            path = Some(Pointer::default());
        }
        // path should now have a value
        let path = path.expect("path should be set");
        self.add_parent_uris_with_path(&mut uris, &path, parent)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        let anchors = self
            .find_anchors(dialect_idx, &src)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        add_uris_from_anchors(&uri, &mut uris, &anchors)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        if uris
            .iter()
            .any(|uri| uri == "http://localhost:1234/draft2020-12/root#/$defs/B")
        {
            println!("!!!!!!!!!!!!!!!!!!!!!!!!\n{uris:#?}");
        }

        self.sources
            .link_all(id.as_ref(), &uris, &link)
            .map_err(|err| self.handle_err(err.into(), continue_on_err, &uri))?;

        let dialect_uri = self.dialects[dialect_idx].id().clone();

        let key = self
            .schemas
            .insert(CompiledSchema::new(
                id,
                path.clone(),
                uris,
                link,
                anchors,
                parent,
                dialect_uri,
            ))
            .map_err(|err| self.handle_err(err.into(), continue_on_err, &uri))?;

        self.maybe_finalize(
            key,
            &uri,
            path,
            &src,
            dialect_idx,
            parent,
            ref_,
            continue_on_err,
            q,
        )
        .map_err(|err| self.handle_err(err, continue_on_err, &uri))
    }

    fn handle_err(
        &mut self,
        err: CompileError,
        continue_on_err: bool,
        uri: &AbsoluteUri,
    ) -> (bool, CompileError) {
        if !continue_on_err {
            return (false, err);
        }
        match err {
            CompileError::SchemaNotFound(_)
            | CompileError::FailedToSource(_)
            | CompileError::FailedToLinkSource(_)
            | CompileError::Custom(_) => {
                if let Some(key) = self.schemas.get_key(uri) {
                    self.schemas.remove(key);
                }
                (false, err)
            }
            _ => (false, err),
        }
    }

    fn maybe_finalize(
        &mut self,
        key: Key,
        uri: &AbsoluteUri,
        path: Pointer,
        src: &Value,
        dialect_idx: usize,
        parent: Option<Key>,
        ref_: Option<RefToResolve>,
        continue_on_err: bool,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), CompileError> {
        let src_str = serde_json::to_string_pretty(src).unwrap();
        if !uri.starts_with("https://json-schema") {
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
            println!("MAYBE FINALIZE");
            println!("uri: {uri}");
            println!("path: \"{path}\"");
            println!("{src_str}");
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        }

        if self.schemas.is_compiled(key) {
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
            println!("ALREADY FINALIZED");
            println!("uri: {uri}");
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
            return Ok(());
        }
        let kick_back = |q: &mut VecDeque<SchemaToCompile>| {
            // kicking resolve_ref and setup_keywords down the road until all
            // subschemas are compiled and refs are resolved
            if !uri.starts_with("https://json") {
                println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
                println!("KICK BACK");
                dbg!(&uri);
                dbg!(&path);
                println!("{src_str}");
                println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
            }
            q.push_back(SchemaToCompile {
                key: Some(key),
                uri: uri.clone(),
                path: Some(path.clone()),
                parent,
                default_dialect_idx: dialect_idx,
                continue_on_err,
                ref_: ref_.clone(),
            });
            Ok(())
        };

        // if !uri.starts_with("https://json") {
        //     println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        //     println!("QUEUING SUBSCHEMAS");
        //     println!("uri:\t{uri}");
        //     println!("path:\t{path}");
        //     println!("src:\t{src_str}");
        //     println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        // }

        if self.queue_subschemas(key, uri, &path, dialect_idx, src, q)? {
            println!("++++ SUBSCHEMAS KICKBACK");
            return kick_back(q);
        }
        if !uri.starts_with("https://json") {
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
            println!("QUEUING REFS");
            println!("uri:\t{uri}");
            println!("path:\t{path}");
            println!("src:\t{src_str}");
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        }

        if self.queue_refs(key, dialect_idx, src, q)? {
            println!("++++ REFS KICKBACK");
            return kick_back(q);
        }

        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        println!("CHECKING KEYWORDS");

        if !self.schemas.has_keywords(key) {
            println!("SETUP KEYWORDS");
            self.setup_keywords(key, &self.dialects[dialect_idx])?;
        }
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        println!("CHECKING REF");

        if let Some(ref_) = ref_ {
            self.resolve_ref(ref_.referrer_key, key, uri.clone(), ref_.ref_)?;
        }
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        println!("SETTING COMPILED");
        dbg!(uri);
        self.schemas.set_compiled(key);
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        Ok(())
    }

    fn queue_pathed(
        &mut self,
        s: SchemaToCompile,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), (bool, CompileError)> {
        // if parent is None and this schema is not a document root (does
        // not have an $id/id) then find the most relevant ancestor and
        // compile it. Doing so will also compile this schema.
        self.queue_ancestors(&s.uri, q)
            .map_err(|err| self.handle_err(err, s.continue_on_err, &s.uri))?;
        q.push_back(s);
        Ok(())
    }

    fn queue_anchored(
        &mut self,
        s: SchemaToCompile,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), (bool, CompileError)> {
        let mut root_uri = s.uri.clone();
        root_uri.set_fragment(None).unwrap();
        let anchor = s.uri.fragment_decoded_lossy().unwrap_or_default();

        // if the root uri is indexed and this schema was not found by now then
        // the anchor is unknown
        if self.schemas.contains_uri(&root_uri) {
            return Err((
                false,
                UnknownAnchorError {
                    anchor: anchor.to_string(),
                    uri: s.uri.clone(),
                }
                .into(),
            ));
        }
        // need to compile the root schema first in order to locate the anchor
        //
        // adding this schema to the front of the queue
        q.push_front(s);
        // putting the root ahead of it
        //
        // if the anchor is not found then an error should be raised.
        q.push_front(SchemaToCompile {
            key: None,
            uri: root_uri,
            path: Some(Pointer::default()),
            parent: None,
            default_dialect_idx: self.dialects.default_index(),
            continue_on_err: false,
            ref_: None,
        });
        Ok(())
    }

    fn setup_keywords(&mut self, key: Key, dialect: &Dialect) -> Result<(), CompileError> {
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        println!("SETUP KEYWORDS");
        println!("key:\t{key:?}");
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");

        let keywords = {
            let schema = self.schemas.get(key, self.sources).unwrap();
            let mut keywords = Vec::new();
            for mut keyword in dialect.keywords().iter().cloned() {
                let mut compile = Compile {
                    absolute_uri: schema.absolute_uri(),
                    schemas: self.schemas,
                    numbers: self.numbers,
                    value_cache: self.values,
                    state: self.global_state,
                };
                if keyword.compile(&mut compile, schema.clone())? {
                    keywords.push(keyword);
                }
            }
            keywords.into_boxed_slice()
        };
        self.schemas.get_mut(key).unwrap().keywords = keywords;
        Ok(())
    }

    fn resolve_ref(
        &mut self,
        referrer_key: Key,
        referenced_key: Key,
        referenced_uri: AbsoluteUri,
        ref_: Ref,
    ) -> Result<(), CompileError> {
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        println!("RESOLVE REF");
        dbg!(&ref_);
        dbg!(referrer_key);
        dbg!(referenced_key);
        dbg!(&referenced_uri);
        println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        self.add_reference(referrer_key, referenced_key, referenced_uri, ref_)?;
        self.schemas.add_dependent(referenced_key, referrer_key);
        Ok(())
    }

    fn add_reference(
        &mut self,
        referrer_key: Key,
        referenced_key: Key,
        referenced_uri: AbsoluteUri,
        ref_: Ref,
    ) -> Result<(), CompileError> {
        self.schemas.add_reference(
            referrer_key,
            Reference {
                key: referenced_key,
                uri: ref_.uri,
                absolute_uri: referenced_uri,
                keyword: ref_.keyword,
            },
            self.sources,
        )?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn queue_refs(
        &mut self,
        key: Key,
        default_dialect_idx: usize,
        src: &Value,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<bool, CompileError> {
        let dialect = &self.dialects[default_dialect_idx];
        let refs = dialect.refs(src)?;
        let dependent_uri = self
            .schemas
            .get(key, self.sources)
            .unwrap()
            .absolute_uri()
            .clone();

        let mut base_uri = dependent_uri.clone();
        base_uri.set_fragment(None).unwrap();

        let mut has_unresolved_refs = false;
        for ref_ in refs {
            let ref_uri = base_uri.resolve(&ref_.uri)?;
            if self.schemas.contains_uri(&ref_uri) {
                let ref_key = self.schemas.get_key(&ref_uri).unwrap();
                self.resolve_ref(key, ref_key, ref_uri, ref_)?;
            } else {
                has_unresolved_refs = true;
                if ref_uri == "http://localhost:1234/draft2020-12/root#/$defs/B" {
                    println!("!!!!!!!!FOUND IT");
                    dbg!(&ref_uri);
                }
                q.push_front(SchemaToCompile {
                    key: None,
                    uri: ref_uri,
                    path: None,
                    parent: None,
                    default_dialect_idx: self.dialects.default_index(),
                    continue_on_err: false,
                    ref_: Some(RefToResolve {
                        referrer_key: key,
                        ref_,
                    }),
                });
            }
        }

        Ok(has_unresolved_refs)
    }
    fn dialect_idx(&self, src: &Value, default: usize) -> usize {
        self.dialects.pertinent_to_idx(src).unwrap_or(default)
    }

    fn queue_subschemas(
        &mut self,
        key: Key,
        uri: &AbsoluteUri,
        path: &Pointer,
        dialect_idx: usize,
        src: &Value,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<bool, CompileError> {
        let src_str = serde_json::to_string_pretty(src).unwrap();
        if !uri.starts_with("https://json") {
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
            println!("QUEUE SUBSCHEMAS");
            dbg!(&uri);
            dbg!(path);
            println!("{src_str}");
            println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
        }
        let fragment = uri.fragment_decoded_lossy().unwrap_or_default();
        let mut path = path.clone();
        if path.is_empty() && fragment.starts_with('/') {
            path = Pointer::parse(&fragment)?;
        }

        let dialect = &self.dialects[dialect_idx];
        let subschemas = dialect.subschemas(&path, src);
        let mut has_subschemas = false;
        for subschema_path in subschemas {
            let mut uri = uri.clone();
            if subschema_path.is_empty() {
                uri.set_fragment(None)?;
            } else {
                uri.set_fragment(Some(&subschema_path))?;
            }
            if !uri.starts_with("https://json") {
                println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
                println!("SUBSCHEMA");
                dbg!(&uri);
                dbg!(&path);
                dbg!(&subschema_path);
            }
            if !self.schemas.has_keywords_by_uri(&uri) {
                has_subschemas = true;
                // if q.iter()
                //     .any(|s| s.uri == uri && s.path.as_ref() == Some(&subschema_path))
                // {
                //     continue;
                // }

                let subschema = SchemaToCompile {
                    key: None,
                    uri,
                    path: Some(subschema_path.clone()),
                    parent: Some(key),
                    default_dialect_idx: dialect_idx,
                    continue_on_err: false,
                    ref_: None,
                };
                println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
                println!("QUEUING SUBSCEHMA");
                dbg!(&subschema);
                println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
                q.push_front(subschema);
            } else if !uri.starts_with("https://json") {
                println!("\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n");
                println!("HAS KEYWORDS");
                dbg!(&uri);
            }
        }
        Ok(has_subschemas)
    }

    fn queue_ancestors(
        &mut self,
        target_uri: &AbsoluteUri,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), CompileError> {
        let mut path = Pointer::parse(&target_uri.fragment_decoded_lossy().unwrap())
            .map_err(|err| CompileError::FailedToParsePointer(err.into()))?;

        if target_uri == "http://localhost:1234/draft2020-12/root#/$defs/B" {
            println!("!!!!!!!!FOUND IT");
            dbg!(&target_uri);
        }

        q.push_front(SchemaToCompile {
            key: None,
            uri: target_uri.clone(),
            path: Some(path.clone()),
            parent: None,
            default_dialect_idx: self.dialects.default_index(),
            continue_on_err: true,
            ref_: None,
        });
        while !path.is_root() {
            path.pop_back();
            let mut uri = target_uri.clone();
            if path.is_empty() {
                uri.set_fragment(None)?;
            } else {
                uri.set_fragment(Some(&path))?;
            }
            if let Some(key) = self.schemas.get_key(&uri) {
                if self.schemas.is_compiled(key) {
                    return Err(CompileError::SchemaNotFound(target_uri.clone()));
                }
                continue;
            }
            q.push_front(SchemaToCompile {
                key: None,
                uri,
                path: Some(path.clone()),
                parent: None,
                default_dialect_idx: self.dialects.default_index(),
                continue_on_err: true,
                ref_: None,
            });
        }
        let mut uri = target_uri.clone();
        uri.set_fragment(None).unwrap();

        if self.schemas.is_compiled_by_uri(&uri) {
            return Err(CompileError::SchemaNotFound(target_uri.clone()));
        }

        q.push_front(SchemaToCompile {
            uri,
            key: None,
            path: None,
            parent: None,
            default_dialect_idx: self.dialects.default_index(),
            continue_on_err: true,
            ref_: None,
        });
        Ok(())
    }
    async fn source(&mut self, uri: &AbsoluteUri) -> Result<(Link, Value), CompileError> {
        let link = self
            .sources
            .resolve_link(uri.clone(), self.resolvers, self.deserializers)
            .await?;
        let mut source = self.sources.get(link.src_key);
        if !link.src_path.is_empty() {
            source = source.resolve(&link.src_path).unwrap();
        }
        let source = source.clone();
        Ok((link, source))
    }

    fn find_anchors(
        &mut self,
        dialect_idx: usize,
        src: &Value,
    ) -> Result<Vec<Anchor>, CompileError> {
        Ok(self
            .dialects
            .get_by_index(dialect_idx)
            .unwrap()
            .anchors(src)?)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn add_parent_uris_with_path(
        &mut self,
        uris: &mut Vec<AbsoluteUri>,
        path: &Pointer,
        parent: Option<Key>,
    ) -> Result<(), CompileError> {
        let Some(parent) = parent else {
            return Ok(());
        };
        let parent = self.schemas.get_compiled(parent).unwrap();
        #[allow(clippy::explicit_iter_loop)]
        for uri in parent.uris.iter() {
            let fragment = uri.fragment_decoded_lossy().unwrap_or_default();
            if fragment.is_empty() || fragment.starts_with('/') {
                let mut uri = uri.clone();
                let mut uri_path = Pointer::parse(&fragment)
                    .map_err(|e| CompileError::FailedToParsePointer(e.into()))?;
                uri_path.append(path);
                uri.set_fragment(Some(&uri_path))?;
                if !uris.contains(&uri) {
                    uris.push(uri);
                }
            }
        }
        Ok(())
    }

    fn validate(&mut self, dialect_idx: usize, src: &Value) -> Result<(), CompileError> {
        if !self.validate {
            return Ok(());
        }
        let mut eval_state = AnyMap::new();
        let mut evaluated = HashSet::default();
        let mut eval_numbers = Numbers::with_capacity(7);
        let key = self.dialects.get_by_index(dialect_idx).unwrap().schema_key;

        let output = self.schemas.evaluate(
            Structure::Verbose,
            key,
            src,
            Pointer::default(),
            Pointer::default(),
            self.sources,
            &mut evaluated,
            self.global_state,
            &mut eval_state,
            self.numbers,
            &mut eval_numbers,
        )?;
        if output.is_invalid() {
            return Err(CompileError::SchemaInvalid(output.into_owned()));
        }
        Ok(())
    }
}

fn add_uris_from_anchors(
    base_uri: &AbsoluteUri,
    uris: &mut Vec<AbsoluteUri>,
    anchors: &[Anchor],
) -> Result<(), CompileError> {
    for anchor in anchors {
        let mut base_uri = base_uri.clone();
        base_uri.set_fragment(Some(&anchor.name))?;
        if !uris.contains(&base_uri) {
            uris.push(base_uri);
        }
    }
    Ok(())
}

fn is_anchored(uri: &AbsoluteUri) -> bool {
    // if the schema is anchored (i.e. has a non json ptr fragment) then
    // compile the root (non-fragmented uri) and attempt to locate the anchor.
    let fragment = uri.fragment_decoded_lossy().unwrap_or_default();
    !fragment.is_empty() && !fragment.starts_with('/')
}

fn has_ptr_fragment(uri: &AbsoluteUri) -> bool {
    uri.fragment().unwrap_or_default().starts_with('/')
}

fn identify(
    link: &Link,
    source: &Value,
    dialect: &Dialect,
) -> Result<(AbsoluteUri, Option<AbsoluteUri>, Vec<AbsoluteUri>), CompileError> {
    let (id, uris) = dialect.identify(link.uri.clone(), &link.src_path, source)?;
    // if identify did not find a primary id, use the uri + pointer fragment
    // as the lookup which will be at the first position in the uris list
    let uri = id.as_ref().unwrap_or(&uris[0]);
    Ok((uri.clone(), id, uris))
}
