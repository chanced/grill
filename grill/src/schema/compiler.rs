use std::collections::VecDeque;

use jsonptr::Pointer;
use serde_json::Value;

use crate::{
    anymap::AnyMap,
    error::{CompileError, UnknownAnchorError},
    keyword::{Compile, Numbers, Values},
    schema::{dialect::Dialects, Schemas},
    source::{Deserializers, Link, Resolvers, Sources},
    uri::TryIntoAbsoluteUri,
    AbsoluteUri, Interrogator, Key,
};

use super::{CompiledSchema, Dialect, Ref, Reference};

struct RefToResolve {
    dependent_key: Key,
    ref_: Ref,
}

/// A pending schema to compile
struct SchemaToCompile {
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
}

impl<'i> Compiler<'i> {
    pub(crate) fn new(interrogator: &'i mut Interrogator) -> Self {
        Self {
            schemas: &mut interrogator.schemas,
            sources: &mut interrogator.sources,
            global_state: &mut interrogator.state,
            dialects: &interrogator.dialects,
            deserializers: &interrogator.deserializers,
            resolvers: &interrogator.resolvers,
            numbers: &mut interrogator.numbers,
            values: &mut interrogator.values,
        }
    }
    pub(crate) async fn compile(mut self, uri: AbsoluteUri) -> Result<Key, CompileError> {
        let mut q = VecDeque::new();
        q.push_front(SchemaToCompile {
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
            q.push_back(SchemaToCompile {
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

    fn handle_err(
        &mut self,
        err: CompileError,
        continue_on_err: bool,
        uri: &AbsoluteUri,
    ) -> (bool, CompileError) {
        let key = self.schemas.get_key(uri);
        match &err {
            CompileError::FailedToResolve(_)
            | CompileError::FailedToSource(_)
            | CompileError::CyclicGraph(_)
            | CompileError::FailedToLinkSource(_)
            | CompileError::Custom(_) => {
                if continue_on_err {
                    if let Some(key) = key {
                        self.schemas.remove(key);
                    }
                }
                (continue_on_err, err)
            }
            _ => (false, err),
        }
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
        let anchor = s.uri.fragment().unwrap_or_default();
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
        q.push_front(s);
        q.push_front(SchemaToCompile {
            uri: root_uri,
            path: Some(Pointer::default()),
            parent: None,
            default_dialect_idx: self.dialects.default_index(),
            continue_on_err: false,
            ref_: None,
        });
        Ok(())
    }
    async fn compile_schema(
        &mut self,
        schema_to_compile: SchemaToCompile,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), (bool, CompileError)> {
        let SchemaToCompile {
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
        let (uri, id, mut uris) = identify(&link, &src, &self.dialects[dialect_idx])
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;
        if id.is_some() {
            path = Some(Pointer::default());
            parent = None;
        } else if parent.is_none() && path.is_none() && has_ptr_fragment(&uri) {
            return self.queue_pathed(
                SchemaToCompile {
                    uri,
                    path,
                    parent,
                    default_dialect_idx,
                    continue_on_err,
                    ref_,
                },
                q,
            );
        } else if is_anchored(&uri) {
            return self.queue_anchored(
                SchemaToCompile {
                    uri,
                    path,
                    parent,
                    default_dialect_idx,
                    continue_on_err,
                    ref_,
                },
                q,
            );
        } else if parent.is_none() && path.is_none() {
            // if the uri does not have a pointer fragment, then it should be
            // compiled as document root
            path = Some(Pointer::default());
        }

        // // path should now have a value
        let path = path.expect("path should be set");

        self.add_parent_uris_with_path(&mut uris, &path, parent)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        self.sources
            .link_all(id.as_ref(), &uris, &link)
            .map_err(|err| self.handle_err(err.into(), continue_on_err, &uri))?;

        let key = self
            .insert(id.clone(), path.clone(), uris.clone(), parent, link)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        self.queue_subschemas(key, &uri, &path, dialect_idx, &src, q)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        self.queue_refs(key, &src, &self.dialects[dialect_idx], q)
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;

        if let Some(ref_) = ref_ {
            self.resolve_ref(uri.clone(), ref_)
                .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;
        }
        self.setup_keywords(key, &self.dialects[dialect_idx])
            .map_err(|err| self.handle_err(err, continue_on_err, &uri))?;
        Ok(())
    }

    fn setup_keywords(&mut self, key: Key, dialect: &Dialect) -> Result<(), CompileError> {
        let keywords = {
            let schema = self.schemas.get(key, self.sources).unwrap();
            let mut keywords = Vec::new();
            for mut keyword in dialect.keywords().iter().cloned() {
                let mut compile = Compile {
                    base_uri: schema.absolute_uri(),
                    schemas: self.schemas,
                    numbers: self.numbers,
                    value_cache: self.values,
                    state: self.global_state,
                };
                if keyword.setup(&mut compile, schema.clone())? {
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
        absolute_uri: AbsoluteUri,
        ref_: RefToResolve,
    ) -> Result<(), CompileError> {
        let RefToResolve {
            dependent_key,
            ref_,
        } = ref_;
        let referenced_key = self.schemas.get_key(&absolute_uri).unwrap();
        self.schemas.add_reference(
            dependent_key,
            Reference {
                key: referenced_key,
                uri: ref_.uri,
                absolute_uri,
                keyword: ref_.keyword,
            },
        );
        self.schemas.add_dependent(referenced_key, dependent_key);
        todo!()
    }

    fn queue_refs(
        &mut self,
        key: Key,
        src: &Value,
        dialect: &Dialect,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), CompileError> {
        let refs = dialect.refs(src)?;
        let dependent_uri = self
            .schemas
            .get(key, self.sources)
            .unwrap()
            .absolute_uri()
            .clone();
        let mut base_uri = dependent_uri.clone();
        base_uri.set_fragment(None).unwrap();

        for ref_ in refs {
            let absolute_uri = base_uri.resolve(&ref_.uri)?;
            q.push_front(SchemaToCompile {
                uri: absolute_uri,
                path: None,
                parent: None,
                default_dialect_idx: self.dialects.default_index(),
                continue_on_err: false,
                ref_: Some(RefToResolve {
                    dependent_key: key,
                    ref_,
                }),
            });
        }
        Ok(())
    }
    fn dialect_idx(&self, src: &Value, default: usize) -> usize {
        self.dialects.pertinent_to_idx(src).unwrap_or(default)
    }
    fn insert(
        &mut self,
        id: Option<AbsoluteUri>,
        path: Pointer,
        uris: Vec<AbsoluteUri>,
        parent: Option<Key>,
        link: Link,
    ) -> Result<Key, CompileError> {
        let key = self
            .schemas
            .insert(CompiledSchema::new(id, path, uris, link, parent))?;
        Ok(key)
    }

    fn queue_subschemas(
        &mut self,
        key: Key,
        uri: &AbsoluteUri,
        path: &Pointer,
        dialect_idx: usize,
        src: &Value,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), CompileError> {
        let dialect = &self.dialects[dialect_idx];
        let subschemas = dialect.subschemas(path, src);
        for subschema_path in subschemas {
            let mut uri = uri.clone();
            if subschema_path.is_empty() {
                uri.set_fragment(None)?;
            } else {
                uri.set_fragment(Some(&subschema_path))?;
            }
            q.push_front(SchemaToCompile {
                uri,
                path: Some(subschema_path),
                parent: Some(key),
                default_dialect_idx: dialect_idx,
                continue_on_err: false,
                ref_: None,
            });
        }
        Ok(())
    }

    fn queue_ancestors(
        &mut self,
        target_uri: &AbsoluteUri,
        q: &mut VecDeque<SchemaToCompile>,
    ) -> Result<(), CompileError> {
        let path = Pointer::parse(target_uri.fragment().unwrap())
            .map_err(|err| CompileError::FailedToParsePointer(err.into()))?;
        while !path.is_empty() {
            let mut path = path.clone();
            path.pop_back();
            let mut uri = target_uri.clone();
            if path.is_empty() {
                uri.set_fragment(None)?;
            } else {
                uri.set_fragment(Some(&path))?;
            }
            if self.schemas.contains_uri(&uri) {
                return Err(CompileError::SchemaNotFound(target_uri.clone()));
            }
            q.push_front(SchemaToCompile {
                uri,
                path: Some(path),
                parent: None,
                default_dialect_idx: self.dialects.default_index(),
                continue_on_err: true,
                ref_: None,
            });
        }
        Ok(())
    }
    async fn source(&mut self, uri: &AbsoluteUri) -> Result<(Link, Value), CompileError> {
        let link = self
            .sources
            .resolve_link(uri.clone(), self.resolvers, self.deserializers)
            .await?;
        let source = self.sources.get(link.key).clone();
        Ok((link, source))
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
        let parent = self.schemas.get(parent, self.sources).unwrap();
        #[allow(clippy::explicit_iter_loop)]
        for uri in parent.uris.iter() {
            let fragment = uri.fragment().unwrap_or_default();
            if fragment.is_empty() || fragment.starts_with('/') {
                let mut uri = uri.clone();
                let mut uri_path = Pointer::parse(fragment)
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
}

fn is_anchored(uri: &AbsoluteUri) -> bool {
    // if the schema is anchored (i.e. has a non json ptr fragment) then
    // compile the root (non-fragmented uri) and attempt to locate the anchor.
    let fragment = uri.fragment().unwrap_or_default().trim();
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
    let (id, uris) = dialect.identify(link.uri.clone(), &link.path, source)?;
    // if identify did not find a primary id, use the uri + pointer fragment
    // as the lookup which will be at the first position in the uris list
    let uri = id.as_ref().unwrap_or(&uris[0]);
    Ok((uri.clone(), id, uris))
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[tokio::test]
//     async fn test_spike() {
//         let _interrogator = Interrogator::json_schema_2020_12()
//             .deserialize_json()
//             .build()
//             .await
//             .unwrap();
//     }
// }
