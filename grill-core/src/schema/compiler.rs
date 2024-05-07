use std::collections::{HashMap, HashSet, VecDeque};

use grill_uri::{AbsoluteUri, TryIntoAbsoluteUri};
use jsonptr::{Pointer, Resolve};
use serde_json::Value;
use slotmap::Key;
use snafu::{ensure, Backtrace};

use crate::{
    cache::{Numbers, Values},
    criterion::{Criterion, CriterionReportOutput, Keyword, NewCompile, Output, Ref, Report},
    error::{compile_error::SchemaNotFoundSnafu, CompileError},
    schema::{dialect::Dialects, Schemas},
    source::{Deserializers, Link, Resolvers, SourceKey, Sources},
    Interrogator, Validate,
};

use super::{Anchor, CompiledSchema, Dialect, Evaluate, Reference};

// TODO: insert a link for the uri + an empty fragment if None (http://example/path/ -> http://example/path/#)
// TODO: handle 07 style $ids (hashtagged)

#[derive(Clone, Debug)]
struct RefToResolve<K> {
    referrer_key: K,
    ref_: Ref,
}

#[derive(Debug)]
/// A pending schema to compile
struct SchemaToCompile<K: Key> {
    key: Option<K>,
    uri: AbsoluteUri,
    parent: Option<K>,
    /// Errors are to be disregarded.
    continue_on_err: bool,
    ref_: Option<RefToResolve<K>>,
}

#[derive(Debug)]
struct Location<'v> {
    uri: AbsoluteUri,
    ancestry_uris: Vec<AbsoluteUri>,
    src: &'v Value,
    src_key: SourceKey,
    /// relative from the last schema root
    rel_path: Pointer,
    /// absolute path from the source
    abs_path: Pointer,
    /// path from the parent schema
    sub_path: Pointer,
    default_dialect_idx: usize,
}

pub(crate) struct Compiler<'i, C: Criterion<K>, K: 'static + Key> {
    schemas: &'i mut Schemas<C, K>,
    sources: &'i mut Sources,
    dialects: &'i Dialects<C, K>,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
    numbers: &'i mut Numbers,
    values: &'i mut Values,
    validate: Validate,
    indexed: HashSet<AbsoluteUri>,
    ids: HashMap<AbsoluteUri, Option<AbsoluteUri>>,
    anchors: HashMap<AbsoluteUri, Vec<Anchor>>,
    subschemas: HashMap<AbsoluteUri, HashSet<Pointer>>,
    uris: HashMap<AbsoluteUri, Vec<AbsoluteUri>>,
    dialect_idxs: HashMap<AbsoluteUri, usize>,
    primary_uris: HashMap<AbsoluteUri, AbsoluteUri>,
    paths: HashMap<AbsoluteUri, Pointer>,
    refs: HashMap<AbsoluteUri, Vec<Ref>>,
    keywords: HashMap<AbsoluteUri, &'i [C::Keyword]>,
    criterion: &'i mut C,
}

#[allow(clippy::too_many_arguments)]
impl<'i, C, K> Compiler<'i, C, K>
where
    C: Criterion<K>,
    K: Key,
{
    pub(crate) fn new(interrogator: &'i mut Interrogator<C, K>, validate: Validate) -> Self {
        Self {
            schemas: &mut interrogator.schemas,
            sources: &mut interrogator.sources,
            dialects: &interrogator.dialects,
            deserializers: &interrogator.deserializers,
            resolvers: &interrogator.resolvers,
            numbers: &mut interrogator.numbers,
            values: &mut interrogator.values,
            criterion: &mut interrogator.language,
            validate,
            ids: HashMap::default(),
            indexed: HashSet::default(),
            anchors: HashMap::default(),
            subschemas: HashMap::default(),
            uris: HashMap::default(),
            dialect_idxs: HashMap::default(),
            primary_uris: HashMap::default(),
            paths: HashMap::default(),
            refs: HashMap::default(),
            keywords: HashMap::default(),
        }
    }

    pub(crate) async fn compile(mut self, uri: AbsoluteUri) -> Result<K, CompileError<C, K>> {
        let mut q = VecDeque::with_capacity(32);

        q.push_front(SchemaToCompile {
            key: None,
            uri: uri.clone(),
            parent: None,
            continue_on_err: false,
            ref_: None,
        });
        self.run(q).await?;
        Ok(self.schemas.get_key(&uri).unwrap())
    }

    pub(crate) async fn compile_all<I>(
        mut self,
        uris: I,
    ) -> Result<Vec<(AbsoluteUri, K)>, CompileError<C, K>>
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
            q.push_back(SchemaToCompile {
                key: None,
                uri: uri.clone(),
                parent: None,
                continue_on_err: false,
                ref_: None,
            });
        }
        self.run(q).await?;
        Ok(uris.into_iter().map(|uri| self.compiled(uri)).collect())
    }

    fn compiled(&self, uri: AbsoluteUri) -> (AbsoluteUri, K) {
        let key = self.schemas.get_key(&uri).unwrap();
        (uri, key)
    }

    async fn run(&mut self, mut q: VecDeque<SchemaToCompile<K>>) -> Result<(), CompileError<C, K>> {
        while !q.is_empty() {
            let schema_to_compile = q.pop_front().unwrap();
            let result = self.compile_schema(schema_to_compile, &mut q).await;
            if let Err((continue_on_err, err)) = result {
                if !continue_on_err {
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    async fn precompile(
        &mut self,
        uri: &AbsoluteUri,
    ) -> Result<Option<(Link, Value)>, CompileError<C, K>> {
        let mut indexed = self
            .indexed
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>();
        indexed.sort();
        if self.indexed.contains(uri) {
            let (link, src) = self
                .sources
                .resolve(uri, self.resolvers, self.deserializers)
                .await?;
            return Ok(Some((link.clone(), src.clone())));
        }
        self.index_all(uri.clone()).await
    }

    async fn index_all(
        &mut self,
        uri: AbsoluteUri,
    ) -> Result<Option<(Link, Value)>, CompileError<C, K>> {
        let mut base_uri = uri.clone();
        base_uri.set_fragment(None).unwrap();
        if self.indexed.contains(&base_uri) {
            return Ok(None);
        }
        let (link, src) = self
            .sources
            .resolve(&base_uri, self.resolvers, self.deserializers)
            .await?;

        let src = src.clone();
        let link = link.clone();
        let mut q = Vec::new();

        let default_dialect_idx = self.dialect_idx(&src, self.dialects.default_index());

        q.push(Location {
            uri: base_uri.clone(),
            rel_path: Pointer::default(),
            abs_path: Pointer::default(),
            sub_path: Pointer::default(),
            default_dialect_idx,
            src: &src,
            src_key: link.src_key,
            ancestry_uris: vec![uri.clone()],
        });

        while let Some(loc) = q.pop() {
            self.index(loc, &mut q)?;
        }
        let (link, src) = self
            .sources
            .resolve(&uri, self.resolvers, self.deserializers)
            .await?;

        Ok(Some((link.clone(), src.clone())))
    }

    fn index<'v>(
        &mut self,
        loc: Location<'v>,
        q: &mut Vec<Location<'v>>,
    ) -> Result<(), CompileError<C, K>> {
        let Location {
            uri,
            ancestry_uris,
            src,
            mut rel_path,
            abs_path,
            default_dialect_idx,
            src_key,
            sub_path,
        } = loc;

        let src = src.resolve(&sub_path).unwrap();

        let link = self
            .sources
            .insert_link(uri.clone(), Link::new(src_key, abs_path.clone()))?
            .clone();

        let dialect_idx = self.dialect_idx(src, default_dialect_idx);
        let dialect = &self.dialects[dialect_idx];
        let (id, mut uris) = dialect.identify(uri.clone(), src)?;
        append_ancestry_uris(&mut uris, &sub_path, &ancestry_uris)?;
        if id.is_some() {
            rel_path = Pointer::default();
        }
        let uri = id.clone().unwrap_or(uri);
        let anchors = dialect.anchors(src)?;
        append_anchor_uris(&mut uris, &uri, &anchors)?;

        self.sources.link_all(&uris, &link)?;
        let found_subschemas = dialect.subschemas(src);
        let mut subschemas = HashSet::with_capacity(found_subschemas.len());
        for subschema in found_subschemas {
            let uri = append_uri_path(
                &subschema,
                &uri,
                &uri.fragment_decoded_lossy().unwrap_or_default(),
            )?;
            let mut abs_path = abs_path.clone();
            abs_path.append(&subschema);

            let mut rel_path = rel_path.clone();
            rel_path.append(&subschema);

            subschemas.insert(rel_path.clone());
            q.push(Location {
                uri,
                rel_path,
                abs_path,
                sub_path: subschema.clone(),
                default_dialect_idx: dialect_idx,
                src,
                ancestry_uris: uris.clone(),
                src_key,
            });
        }
        for other in &uris {
            self.primary_uris.insert(other.clone(), uri.clone());
            self.indexed.insert(other.clone());
        }

        self.ids.insert(uri.clone(), id);
        self.paths.insert(uri.clone(), rel_path);
        self.dialect_idxs.insert(uri.clone(), dialect_idx);
        self.anchors.insert(uri.clone(), anchors);
        self.uris.insert(uri.clone(), uris);
        self.subschemas.insert(uri.clone(), subschemas);
        self.keywords.insert(uri.clone(), dialect.keywords());
        self.refs.insert(uri.clone(), dialect.refs(src)?);
        self.indexed.insert(uri);
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    async fn compile_schema(
        &mut self,
        schema_to_compile: SchemaToCompile<K>,
        q: &mut VecDeque<SchemaToCompile<K>>,
    ) -> Result<(), (bool, CompileError<C, K>)> {
        let SchemaToCompile {
            key,
            uri,
            mut parent,
            continue_on_err,
            ref_,
        } = schema_to_compile;
        let Some((link, src)) = self
            .precompile(&uri)
            .await
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?
        else {
            return Err(CompileError::SchemaNotFound {
                uri: uri.clone(),
                backtrace: Backtrace::capture(),
            })
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;
        };

        let uri = self.primary_uris.get(&uri).unwrap().clone();
        let dialect_idx = *self.dialect_idxs.get(&uri).unwrap();

        if let Some(key) = key.or(self.schemas.get_key(&uri)) {
            return self
                .maybe_finalize(key, &uri, parent, ref_, continue_on_err, q)
                .map_err(|err| self.handle_err(err, continue_on_err, &uri));
        }

        self.validate(dialect_idx, &src)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        let uris = self.uris.remove(&uri).unwrap();
        let id = self.ids.get(&uri).unwrap().clone();
        if id.is_some() {
            parent = None;
        }
        if parent.is_none() && has_ptr_fragment(&uri) {
            self.uris.insert(uri.clone(), uris);
            return self.queue_pathed(
                SchemaToCompile {
                    key: None,
                    uri: uri.clone(),
                    parent,
                    continue_on_err,
                    ref_: ref_.clone(),
                },
                q,
            );
        }
        if is_anchored(&uri) {
            return self.queue_anchored(
                SchemaToCompile {
                    key: None,
                    uri: uri.clone(),
                    parent,
                    continue_on_err,
                    ref_: ref_.clone(),
                },
                q,
            );
        }
        let anchors = self.anchors.remove(&uri).unwrap();
        let dialect_uri = self.dialects[dialect_idx].id().clone();
        let path = self.paths.remove(&uri).unwrap();
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

        self.maybe_finalize(key, &uri, parent, ref_, continue_on_err, q)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))
    }

    fn handle_err(
        &mut self,
        err: CompileError<C, K>,
        continue_on_err: bool,
        uri: &AbsoluteUri,
    ) -> (bool, CompileError<C, K>) {
        if !continue_on_err {
            return (false, err);
        }
        match err {
            CompileError::SchemaNotFound { .. }
            | CompileError::FailedToSource { .. }
            | CompileError::FailedToLinkSource { .. }
            | CompileError::Custom { .. } => {
                if let Some(key) = self.schemas.get_key(uri) {
                    self.schemas.remove(key);
                }
                (continue_on_err, err)
            }
            _ => (false, err),
        }
    }

    fn remaining_subschemas(
        &mut self,
        key: K,
        uri: &AbsoluteUri,
    ) -> Result<Vec<SchemaToCompile<K>>, CompileError<C, K>> {
        let existing = self.subschemas.get_mut(uri).unwrap();
        let (remaining, subschemas) = subschemas(key, uri, existing.iter(), self.schemas)?;
        *existing = remaining;
        Ok(subschemas)
    }

    fn remaining_refs(
        &mut self,
        key: K,
        mut base_uri: AbsoluteUri,
    ) -> Result<Vec<SchemaToCompile<K>>, CompileError<C, K>> {
        let refs = self.refs.get(&base_uri).unwrap().clone();
        // references which cannot be resolved yet due to the referenced
        // not being resolved yet.
        let mut queued = Vec::with_capacity(refs.len());
        let mut remaining = Vec::with_capacity(refs.len());
        base_uri.set_fragment(None).unwrap();
        for ref_ in refs.iter().cloned() {
            let referenced_uri = base_uri.resolve(&ref_.uri)?;
            // if the referenced schema has been compiled, resolve it
            if self.schemas.contains_uri(&referenced_uri) {
                let referenced_key = self.schemas.get_key(&referenced_uri).unwrap();
                self.resolve_ref(
                    referenced_key,
                    referenced_uri,
                    RefToResolve {
                        referrer_key: key,
                        ref_,
                    },
                )?;
            } else {
                // otherwise, requeue it
                remaining.push(ref_.clone());
                queued.push(SchemaToCompile {
                    key: None,
                    uri: referenced_uri,
                    parent: None,
                    continue_on_err: false,
                    ref_: Some(RefToResolve {
                        referrer_key: key,
                        ref_,
                    }),
                });
            }
        }
        let refs = self.refs.get_mut(&base_uri).unwrap();
        *refs = remaining;
        Ok(queued)
    }

    fn maybe_finalize(
        &mut self,
        key: K,
        uri: &AbsoluteUri,
        parent: Option<K>,
        ref_: Option<RefToResolve<K>>,
        continue_on_err: bool,
        q: &mut VecDeque<SchemaToCompile<K>>,
    ) -> Result<(), CompileError<C, K>> {
        if self.schemas.is_compiled(key) {
            return Ok(());
        }
        let kickback = |q: &mut VecDeque<SchemaToCompile<K>>| {
            // kicking resolve_ref and setup_keywords down the road until all
            // subschemas are compiled and refs are resolved
            q.push_back(SchemaToCompile {
                key: Some(key),
                uri: uri.clone(),
                parent,
                continue_on_err,
                ref_: ref_.clone(),
            });
            Ok(())
        };

        let subschemas = self.remaining_subschemas(key, uri)?;
        if !subschemas.is_empty() {
            append_all_front(q, subschemas);
            return kickback(q);
        }

        let refs = self.remaining_refs(key, uri.clone())?;
        if !refs.is_empty() {
            append_all_front(q, refs);
            return kickback(q);
        }

        if !self.schemas.has_keywords(key) {
            let keywords = self.keywords_for(key, self.keywords.get(uri).unwrap())?;
            self.schemas.set_keywords(key, keywords);
        }

        if let Some(ref_) = ref_ {
            self.resolve_ref(key, uri.clone(), ref_)?;
        }
        self.schemas.set_compiled(key);
        Ok(())
    }

    fn queue_pathed(
        &mut self,
        s: SchemaToCompile<K>,
        q: &mut VecDeque<SchemaToCompile<K>>,
    ) -> Result<(), (bool, CompileError<C, K>)> {
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
        s: SchemaToCompile<K>,
        q: &mut VecDeque<SchemaToCompile<K>>,
    ) -> Result<(), (bool, CompileError<C, K>)> {
        let mut root_uri = s.uri.clone();
        root_uri.set_fragment(None).unwrap();
        let anchor = s.uri.fragment_decoded_lossy().unwrap_or_default();

        // if the root uri is indexed and this schema was not found by now then
        // the anchor is unknown
        if self.schemas.contains_uri(&root_uri) {
            return Err((
                false,
                CompileError::UnknownAnchor {
                    anchor: anchor.to_string(),
                    uri: s.uri.clone(),
                },
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
            parent: None,
            continue_on_err: false,
            ref_: None,
        });
        Ok(())
    }

    fn keywords_for(
        &mut self,
        key: K,
        possible: &[C::Keyword],
    ) -> Result<Box<[C::Keyword]>, CompileError<C, K>> {
        let schema = self.schemas.get(key, self.sources).unwrap();
        let mut keywords = Vec::new();
        for keyword in possible.iter() {
            // this is used instead of .iter().cloned() because I'm hunting a lifetime error
            let mut keyword = keyword.clone();
            let mut compile = self.criterion.new_compile(NewCompile {
                absolute_uri: schema.absolute_uri(),
                global_numbers: self.numbers,
                schemas: &self.schemas,
                sources: &self.sources,
                dialects: &self.dialects,
                resolvers: &self.resolvers,
                deserializers: &self.deserializers,
                values: self.values,
            });
            let ctrl_flow = keyword.compile(&mut compile, schema.clone())?;
            let is_continue = ctrl_flow.is_continue();

            // will not compile without this explicit drop
            // even tho Criterion::Keyword is bound to 'static
            // not sure why that is.
            // rust 1.75
            drop(compile);

            if is_continue {
                keywords.push(keyword);
            }
        }
        let keywords = keywords.into_boxed_slice();
        Ok(keywords)
    }

    fn resolve_ref(
        &mut self,
        referenced_key: K,
        referenced_uri: AbsoluteUri,
        ref_: RefToResolve<K>,
    ) -> Result<(), CompileError<C, K>> {
        let referrer_key = ref_.referrer_key;
        self.add_reference(ref_.referrer_key, referenced_key, referenced_uri, ref_.ref_)?;

        self.schemas.add_dependent(referenced_key, referrer_key);
        Ok(())
    }

    fn add_reference(
        &mut self,
        referrer_key: K,
        referenced_key: K,
        referenced_uri: AbsoluteUri,
        ref_: Ref,
    ) -> Result<(), CompileError<C, K>> {
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

    fn dialect_idx(&self, src: &Value, default: usize) -> usize {
        self.dialects.pertinent_to_idx(src).unwrap_or(default)
    }

    fn queue_ancestors(
        &mut self,
        target_uri: &AbsoluteUri,
        q: &mut VecDeque<SchemaToCompile<K>>,
    ) -> Result<(), CompileError<C, K>> {
        let mut path = Pointer::parse(&target_uri.fragment_decoded_lossy().unwrap())?;

        q.push_front(SchemaToCompile {
            key: None,
            uri: target_uri.clone(),
            parent: None,
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
                let is_not_compiled = !self.schemas.is_compiled(key);

                ensure!(is_not_compiled, SchemaNotFoundSnafu { uri });
                continue;
            }

            q.push_front(SchemaToCompile {
                key: None,
                uri,
                parent: None,
                continue_on_err: true,
                ref_: None,
            });
        }
        let mut uri = target_uri.clone();
        uri.set_fragment(None).unwrap();

        ensure!(
            !self.schemas.is_compiled_by_uri(&uri),
            SchemaNotFoundSnafu { uri: target_uri }
        );

        q.push_front(SchemaToCompile {
            uri,
            key: None,
            parent: None,
            continue_on_err: true,
            ref_: None,
        });
        Ok(())
    }

    // fn find_anchors(
    //     &mut self,
    //     dialect_idx: usize,
    //     src: &Value,
    // ) -> Result<Vec<Anchor>, CompileError<C, K>> {
    //     Ok(self
    //         .dialects
    //         .get_by_index(dialect_idx)
    //         .unwrap()
    //         .anchors(src)?)
    // }
    fn should_validate(&self) -> bool {
        (self.validate).into()
    }
    fn validate<'v>(
        &mut self,
        dialect_idx: usize,
        value: &'v Value,
    ) -> Result<(), CompileError<C, K>> {
        if !self.should_validate() {
            return Ok(());
        }
        let mut eval_numbers = Numbers::with_capacity(7);
        let key = self.dialects.get_by_index(dialect_idx).unwrap().schema_key;

        let report = self.schemas.evaluate(Evaluate {
            key,
            value,
            criterion: &self.criterion,
            output: <CriterionReportOutput<C, K>>::verbose(),
            instance_location: Pointer::default(),
            keyword_location: Pointer::default(),
            sources: self.sources,
            dialects: self.dialects,
            global_numbers: self.numbers,
            eval_numbers: &mut eval_numbers,
        })?;
        if !report.is_valid() {
            let report: <C::Report<'v> as Report>::Owned = report.into_owned();
            return Err(CompileError::InvalidSchema {
                report,
                backtrace: Backtrace::capture(),
            });
        }
        // TODO: remove the above if statement and replace with the below once fixed
        // ensure!(
        //     report.is_valid(),
        //     SchemaInvalidSnafu {
        //         report: report.into_owned()
        //     }
        // );
        Ok(())
    }
}

fn subschemas<'c, C: Criterion<K>, K: Key>(
    key: K,
    uri: &AbsoluteUri,
    subschemas: impl ExactSizeIterator<Item = &'c Pointer>,
    schemas: &Schemas<C, K>,
) -> Result<(HashSet<Pointer>, Vec<SchemaToCompile<K>>), CompileError<C, K>> {
    let mut q = Vec::with_capacity(subschemas.len());
    let mut r = HashSet::with_capacity(subschemas.len());
    for path in subschemas {
        let mut uri = uri.clone();
        if path.is_empty() {
            uri.set_fragment(None)?;
        } else {
            uri.set_fragment(Some(path))?;
        }
        if !schemas.has_keywords_by_uri(&uri) {
            let subschema = SchemaToCompile {
                key: None,
                uri,
                parent: Some(key),
                continue_on_err: false,
                ref_: None,
            };
            q.push(subschema);
            r.insert(path.clone());
        }
    }
    Ok((r, q))
}

fn append_all_front<K: Key>(
    q: &mut VecDeque<SchemaToCompile<K>>,
    other: Vec<SchemaToCompile<K>>,
) -> bool {
    if other.is_empty() {
        return false;
    }
    q.reserve(other.len());
    for s in other.into_iter().rev() {
        q.push_front(s);
    }
    true
}

fn append_anchor_uris<'i, C: Criterion<K>, K: Key>(
    uris: &mut Vec<AbsoluteUri>,
    base_uri: &'i AbsoluteUri,
    anchors: &'i [Anchor],
) -> Result<(), CompileError<C, K>> {
    for anchor in anchors {
        let mut uri = base_uri.clone();
        uri.set_fragment(Some(&anchor.name))
            .map_err(|_| CompileError::UriFragmentOverflow {
                uri: uri.clone(),
                fragment: anchor.name.clone(),
                backtrace: Backtrace::capture(),
            })?;
        uris.push(uri);
    }
    Ok(())
}

fn append_ancestry_uris<'a, C: Criterion<K>, K: Key>(
    uris: &mut Vec<AbsoluteUri>,
    path: &'a Pointer,
    parent_uris: &'a [AbsoluteUri],
) -> Result<(), CompileError<C, K>> {
    if path.is_empty() {
        return Ok(());
    }
    for parent_uri in parent_uris {
        let fragment = parent_uri.fragment_decoded_lossy().unwrap_or_default();
        if fragment.is_empty() || fragment.starts_with('/') {
            uris.push(append_uri_path(path, parent_uri, &fragment)?);
        }
    }
    Ok(())
}

fn append_uri_path<C: Criterion<K>, K: Key>(
    path: &Pointer,
    uri: &AbsoluteUri,
    fragment: &str,
) -> Result<AbsoluteUri, CompileError<C, K>> {
    let mut uri = uri.clone();
    let mut uri_path = Pointer::parse(fragment)?;
    uri_path.append(path);
    uri.set_fragment(Some(&uri_path))?;
    Ok(uri)
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

// fn identify<C: Criterion<K>, K: Key>(
//     uri: &AbsoluteUri,
//     source: &Value,
//     dialect: &Dialect<C, K>,
// ) -> Result<(AbsoluteUri, Option<AbsoluteUri>, Vec<AbsoluteUri>), CompileError<C, K>> {
//     let (id, uris) = dialect.identify(uri.clone(), source)?;
//     // if identify did not find a primary id, use the uri + pointer fragment
//     // as the lookup which will be at the first position in the uris list
//     let uri = id.as_ref().unwrap_or(&uris[0]);
//     Ok((uri.clone(), id, uris))
// }
