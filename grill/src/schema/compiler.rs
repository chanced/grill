use std::collections::{hash_map::Entry, HashSet, VecDeque};

use jsonptr::Pointer;
use serde_json::Value;

use crate::{
    anymap::AnyMap,
    error::{CompileError, SourceError, UnknownAnchorError},
    keyword::{Compile, Keyword, Numbers, Values},
    schema::{dialect::Dialects, Schemas},
    source::{Deserializers, Link, Resolvers, Sources},
    uri::TryIntoAbsoluteUri,
    AbsoluteUri, Interrogator, Key,
};

use super::{CompiledSchema, Dialect, Ref};

/// A pending schema to compile
struct PendingSchema {
    uri: AbsoluteUri,
    uris: Vec<AbsoluteUri>,
    id: Option<AbsoluteUri>,
    path: Option<Pointer>,
    parent: Option<Key>,
    default_dialect_idx: usize,
    skip_if_err: bool,
}

pub(crate) struct Compiler<'i> {
    schemas: &'i mut Schemas,
    sources: &'i mut Sources,
    global_state: &'i mut AnyMap,
    dialects: Dialects<'i>,
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
            dialects: interrogator.dialects.as_borrowed(),
            deserializers: &interrogator.deserializers,
            resolvers: &interrogator.resolvers,
            numbers: &mut interrogator.numbers,
            values: &mut interrogator.values,
        }
    }
    pub(crate) async fn compile(mut self, uri: AbsoluteUri) -> Result<Key, CompileError> {
        let mut q = VecDeque::new();
        q.push_front(PendingSchema {
            uri: uri.clone(),
            uris: Vec::new(),
            id: None,
            parent: None,
            path: None,
            default_dialect_idx: self.dialects.default_index(),
            skip_if_err: false,
        });
        self.run(q).await?;
        Ok(self.compiled_key(&uri))
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
            q.push_back(PendingSchema {
                uri: uri.clone(),
                path: None,
                parent: None,
                default_dialect_idx: self.dialects.default_index(),
                skip_if_err: false,
                uris: Vec::new(),
                id: None,
            });
        }
        self.run(q).await?;
        Ok(uris.into_iter().map(|uri| self.compiled(uri)).collect())
    }
    fn compiled_key(&self, uri: &AbsoluteUri) -> Key {
        self.schemas.get_key(uri).unwrap()
    }
    fn compiled(&self, uri: AbsoluteUri) -> (AbsoluteUri, Key) {
        let key = self.compiled_key(&uri);
        (uri, key)
    }

    async fn run(&mut self, mut q: VecDeque<PendingSchema>) -> Result<(), CompileError> {
        while let Some(c) = q.front_mut() {
            // check to see if schema is already been compiled
            if let Some(key) = self.schemas.get_key(&c.uri) {
                q.pop_front();
                continue;
            }
            let (link, src) = self.source(&c.uri).await?;
            let (dialect_idx, dialect) = self.dialect(&src, c.default_dialect_idx);
            let (uri, id, uris) = identify(c, &link, &src, dialect)?;
            if id.is_some() {
                c.path = Some(Pointer::default());
                c.parent = None;
            } else if c.parent.is_none() && c.path.is_none() && has_ptr_fragment(&uri) {
                // if parent is None and this schema is not a document root (does
                // not have an $id/id) then find the most relevant ancestor and
                // compile it. Doing so will also compile this schema.
                self.queue_ancestors(&uri, &mut q)?;
                continue;
            } else if is_anchored(&uri) {
                self.queue_root(&uri, &mut q)?;
                continue;
            } else if c.parent.is_none() && c.path.is_none() {
                // if the uri does not have a pointer fragment, then it should be
                // compiled as document root
                c.path = Some(Pointer::default());
            }
            // path should now have a value
            self.add_parent_uris_with_path(
                &mut c.uris,
                &c.path.as_ref().expect("path should be set"),
                c.parent,
            )?;
            self.sources.link_all(id.as_ref(), &c.uris, &link)?;

            let key = self.insert(c, link)?;

            self.queue_subschemas(key, &src, &mut q)?;

            self.queue_refs(&mut q)?;
        }
        todo!()
    }

    fn insert(&mut self, schema: &PendingSchema, link: Link) -> Result<Key, CompileError> {
        let key = self.schemas.insert(CompiledSchema::new(
            schema.id.clone(),
            schema.path.clone().unwrap(),
            schema.uris.clone(),
            link,
            schema.parent.clone(),
            Vec::new(),
        ))?;
        Ok(key)
    }

    fn dialect(&mut self, src: &Value, default: usize) -> (usize, &Dialect) {
        let dialect_idx = self.dialects.pertinent_to_idx(src).unwrap_or(default);
        let dialect = &self.dialects[dialect_idx];
        (dialect_idx, dialect)
    }

    fn queue_root(
        &mut self,
        uri: &AbsoluteUri,
        q: &mut VecDeque<PendingSchema>,
    ) -> Result<(), CompileError> {
        let mut uri = uri.clone();
        uri.set_fragment(None).unwrap();
        q.push_front(PendingSchema {
            uri,
            path: Some(Pointer::default()),
            parent: None,
            default_dialect_idx: self.dialects.default_index(),
            skip_if_err: false,
            id: None,
            uris: Vec::new(),
        });
        Ok(())
    }

    fn queue_subschemas(
        &mut self,
        key: Key,
        path: &Pointer,
        dialect: &Dialect,
        src: &Value,
        q: &mut VecDeque<PendingSchema>,
    ) -> Result<(), CompileError> {
        let subschemas = dialect.subschemas(path, src).aa?;
        for subschema_path in subschemas {}
        todo!()
    }

    fn queue_ancestors(
        &mut self,
        target_uri: &AbsoluteUri,
        q: &mut VecDeque<PendingSchema>,
    ) -> Result<(), CompileError> {
        let path = Pointer::parse(target_uri.fragment().unwrap())
            .map_err(|err| CompileError::FailedToParsePointer(err.into()))?;
        while !path.is_empty() {
            let mut path = path.clone();
            path.pop_back();
            let mut uri = target_uri.clone();
            uri.set_fragment(Some(&path));
            if self.schemas.contains_uri(&uri) {
                return Err(CompileError::SchemaNotFound(target_uri.clone()));
            }
            q.push_front(PendingSchema {
                uri,
                path: Some(path),
                parent: None,
                default_dialect_idx: self.dialects.default_index(),
                skip_if_err: true,
                uris: Vec::new(),
                id: None,
            })
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
                uri.set_fragment(Some(uri_path.append(path)))?;
                if !uris.contains(&uri) {
                    uris.push(uri);
                }
            }
        }
        Ok(())
    }
}

//     async fn compile_schema(
//         mut self,
//         mut path: Option<Pointer>,
//         link: Link,
//         mut parent: Option<Key>,
//         mut dialects: Dialects<'i>,
//     ) -> Result<Key, CompileError> {
//         // check to see if schema is already been compiled
//         if self.schemas.contains_uri(&link.uri) {
//             // if so, return it.
//             return Ok(self.schemas.get_index(&link.uri).unwrap());
//         }
//         let source = self.sources.get(link.key).clone();

//         // determine the dialect
//         let dialect_idx = dialects.pertinent_to_or_default_idx(&source);
//         dialects.set_default_dialect_index(dialect_idx);
//         let dialect = dialects.get_by_index(dialect_idx).unwrap();

//         let (uri, id, uris) = identify(&link, &source, dialect)?;

//         // check to see if the schema has already been compiled under the id
//         if let Entry::Occupied(key) = self.schemas.index_entry(uri.clone()) {
//             return Ok(*key.get());
//         }

//         if id.is_some() {
//             path = Some(Pointer::default());
//             parent = None;
//         } else if parent.is_none() && path.is_none() && has_ptr_fragment(&uri) {
//             // if parent is None and this schema is not a document root (does
//             // not have an $id/id) then find the most relevant ancestor and
//             // compile it. Doing so will also compile this schema.
//             return self.compile_ancestors(uri, link).await;
//         } else if is_anchored(&uri) {
//             return self.compile_anchored(link).await;
//         } else if parent.is_none() && path.is_none() {
//             // if the uri does not have a pointer fragment, then it should be
//             // compiled as document root
//             path = Some(Pointer::default());
//         }
//         // path should now have a value
//         let path = path.unwrap();

//         let uris = self.add_parent_uris_with_path(uris, &path, parent)?;

//         // linking all URIs of this schema to the the source location
//         self.sources.link_all(id.as_ref(), &uris, &link)?;

//         let anchors = dialect.anchors(&source)?;

//         let key = self
//             .schemas
//             .insert(CompiledSchema::new(
//                 id.clone(),
//                 path.clone(),
//                 uris,
//                 link,
//                 parent,
//                 anchors,
//             ))
//             .map_err(SourceError::SourceConflict)?;

//         let subschemas = self
//             .compile_subschemas(key, &uri, &path, dialect, &source, dialects.clone())
//             .await?;

//         let references = self.references(key, &source, dialect).await?;

//         // check to ensure that there are no cyclic references
//         self.schemas
//             .ensure_not_cyclic(key, &uri, &references, self.sources)?;

//         let keywords = self.compile_keywords(key, &uri, dialect).await?;

//         let schema = self.schemas.get_mut(key).unwrap();

//         schema.references = references;
//         schema.subschemas = subschemas;
//         schema.keywords = keywords;
//         Ok(key)
//     }

//     async fn references(
//         &mut self,
//         base_uri: &AbsoluteUri,
//         source: &Value,
//         dialect: &Dialect,
//     ) -> Result<bool, CompileError> {
//         let refs = dialect.refs(source)?;
//         let has_refs = refs.len() > 0;
//         for ref_ in refs {
//             let ref_uri = base_uri.resolve(&ref_.uri)?;
//             self.q.schemas.push_front(ref_uri);
//             self.q.refs.push_front(ref_);
//         }
//         Ok(has_refs)
//     }

//     async fn anchored(&mut self, uri: AbsoluteUri) -> Result<Key, CompileError> {
//         let fragment = uri.fragment().unwrap_or_default().trim().to_string();
//         let mut base_uri = uri.clone();
//         base_uri.set_fragment(None).unwrap();
//         let (root_link, _) = self
//             .sources
//             .resolve(base_uri.clone(), self.resolvers, self.deserializers)
//             .await?;
//         let root_link = root_link.clone();
//         let _ = self
//             .compile_schema(
//                 Some(Pointer::default()),
//                 root_link,
//                 None,
//                 self.dialects.clone(),
//             )
//             .await?;

//         // at this stage, all URIs should be indexed.
//         match self.schemas.get_by_uri(&link.uri, self.sources) {
//             Some(anchored) => Ok(anchored.key),
//             None => Err(UnknownAnchorError {
//                 anchor: fragment,
//                 uri: link.uri.clone(),
//             }
//             .into()),
//         }
//     }
//     async fn compile_ancestors(
//         &mut self,
//         uri: AbsoluteUri,
//         link: Link,
//     ) -> Result<Key, CompileError> {
//         let link = self.sources.get_link(&link.uri).unwrap().clone();
//         let full_path = Pointer::parse(uri.fragment().unwrap())
//             .map_err(|err| CompileError::FailedToParsePointer(err.into()))?;
//         let mut path = Pointer::default();
//         // TODO: remove once i update jsonptr to include the root token in the `Tokens` iter
//         // https://github.com/chanced/jsonptr/issues/17
//         let uri = link.uri.clone();
//         if let Some(key) = self
//             .compile_ancestor_then_find_descendant(uri.clone(), path.clone())
//             .await
//         {
//             return Ok(key);
//         }
//         for tok in full_path.tokens() {
//             path.push_back(tok);
//             if let Some(key) = self
//                 .compile_ancestor_then_find_descendant(uri.clone(), path.clone())
//                 .await
//             {
//                 return Ok(key);
//             }
//         }
//         self.compile_schema(Some(full_path), link, None, self.dialects.clone())
//             .await
//     }

//     async fn compile_ancestor_then_find_descendant(
//         &mut self,
//         mut uri: AbsoluteUri,
//         path: Pointer,
//     ) -> Option<Key> {
//         uri.set_fragment(Some(&path)).unwrap();
//         let link = self.sources.get_link(&uri).unwrap().clone();
//         self.compile_schema(
//             Some(path.clone()),
//             link.clone(),
//             None,
//             self.dialects.clone(),
//         )
//         .await
//         .ok()
//         .and_then(|ancestor| {
//             self.schemas
//                 .descendants(ancestor, self.sources)
//                 .find_by_uri(&uri)
//                 .map(|s| s.key)
//         })
//     }

//     async fn compile_subschemas(
//         &mut self,
//         key: Key,
//         base_uri: &AbsoluteUri,
//         path: &Pointer,
//         dialect: &Dialect,
//         source: &Value,
//         dialects: Dialects<'i>,
//     ) -> Result<Vec<Key>, CompileError> {
//         let mut subschemas = Vec::new();
//         for subschema_path in dialect.subschemas(path, source) {
//             let mut uri = base_uri.clone();
//             uri.set_fragment(Some(&subschema_path))?;
//             let (sub_link, _) = self
//                 .sources
//                 .resolve(uri, self.resolvers, self.deserializers)
//                 .await?;
//             let sub_link = sub_link.clone();
//             let subschema = self
//                 .compile_schema(Some(subschema_path), sub_link, Some(key), dialects.clone())
//                 .await?;
//             subschemas.push(subschema);
//         }
//         Ok(subschemas)
//     }

//     async fn compile_keywords(
//         &mut self,
//         key: Key,
//         base_uri: &AbsoluteUri,
//         dialect: &Dialect,
//     ) -> Result<Box<[Box<dyn Keyword>]>, CompileError> {
//         let schema = self.schemas.get(key, self.sources).unwrap();

//         let mut keywords = Vec::new();
//         for mut keyword in dialect.keywords().iter().cloned() {
//             let mut compile = Compile {
//                 base_uri,
//                 schemas: self.schemas,
//                 numbers: self.numbers,
//                 value_cache: self.values,
//                 state: self.global_state,
//             };
//             if keyword.compile(&mut compile, schema.clone()).await? {
//                 keywords.push(keyword);
//             }
//         }
//         Ok(keywords.into_boxed_slice())
//     }
// }

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
    c: &mut PendingSchema,
    link: &Link,
    source: &Value,
    dialect: &Dialect,
) -> Result<(AbsoluteUri, Option<AbsoluteUri>, Vec<AbsoluteUri>), CompileError> {
    if let Some(id) = c.id {
        return Ok((c.uri.clone(), Some(id), c.uris));
    }
    let (id, uris) = dialect.identify(link.uri.clone(), &link.path, source)?;
    // if identify did not find a primary id, use the uri + pointer fragment
    // as the lookup which will be at the first position in the uris list
    let uri = id.as_ref().unwrap_or(&uris[0]);
    c.id = id.clone();
    c.uris = uris;
    c.uri = uri.clone();
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
