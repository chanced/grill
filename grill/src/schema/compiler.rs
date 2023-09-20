use std::collections::hash_map::Entry;

use async_recursion::async_recursion;
use jsonptr::Pointer;
use serde_json::Value;

use crate::{
    error::{CompileError, SourceError, UnknownAnchorError},
    interrogator::state::State,
    keyword::{BigInts, BigRationals, Compile, Keyword, Values},
    schema::{dialect::Dialects, Schemas},
    source::{Deserializers, Link, Resolvers, Sources},
    AbsoluteUri, Interrogator, Key,
};

use super::{traverse::Traverse, CompiledSchema, Dialect, Reference};

pub(crate) struct Compiler<'i> {
    schemas: &'i mut Schemas,
    sources: &'i mut Sources,
    global_state: &'i mut State,
    dialects: &'i Dialects<'i>,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
    ints: &'i mut BigInts,
    rationals: &'i mut BigRationals,
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
            ints: &mut interrogator.ints,
            rationals: &mut interrogator.rationals,
            values: &mut interrogator.values,
        }
    }
    pub(crate) async fn compile(mut self, uri: AbsoluteUri) -> Result<Key, CompileError> {
        let link = self
            .sources
            .resolve_link(uri, self.resolvers, self.deserializers)
            .await?;
        self.compile_schema(None, link, None, self.dialects.clone())
            .await
    }

    pub(crate) async fn compile_all(
        mut self,
        uris: impl IntoIterator<Item = AbsoluteUri>,
    ) -> Result<Vec<(AbsoluteUri, Key)>, CompileError> {
        let mut keys = Vec::new();
        for uri in uris {
            let (link, _) = self
                .sources
                .resolve(uri.clone(), self.resolvers, self.deserializers)
                .await?;
            let link = link.clone();
            keys.push((
                uri,
                self.compile_schema(None, link, None, self.dialects.as_borrowed())
                    .await?,
            ));
        }
        Ok(keys)
    }

    #[async_recursion]
    async fn compile_schema(
        &mut self,
        mut path: Option<Pointer>,
        link: Link,
        mut parent: Option<Key>,
        mut dialects: Dialects<'i>,
    ) -> Result<Key, CompileError> {
        // check to see if schema is already been compiled
        if self.schemas.contains_uri(&link.uri) {
            // if so, return it.
            return Ok(self.schemas.get_index(&link.uri).unwrap());
        }
        let source = self.sources.get(link.key).clone();

        // determine the dialect
        let dialect_idx = dialects.pertinent_to_or_default_idx(&source);
        dialects.set_default_dialect_index(dialect_idx);
        let dialect = dialects.get_by_index(dialect_idx).unwrap();

        let (uri, id, uris) = identify(&link, &source, dialect)?;

        // check to see if the schema has already been compiled under the id
        if let Entry::Occupied(key) = self.schemas.index_entry(uri.clone()) {
            return Ok(*key.get());
        }

        if id.is_some() {
            path = Some(Pointer::default());
            parent = None;
        } else if parent.is_none() && path.is_none() && has_ptr_fragment(&uri) {
            // if parent is None and this schema is not a document root (does
            // not have an $id/id) then find the most relevant ancestor and
            // compile it. Doing so will also compile this schema.
            return self.compile_ancestors(uri, link).await;
        } else if is_anchored(&uri) {
            return self.compile_anchored(link).await;
        } else if parent.is_none() && path.is_none() && !has_ptr_fragment(&uri) {
            // if the uri does not have a pointer fragment, then it should be
            // compiled as document root
            path = Some(Pointer::default());
        }
        // path should now have a value
        let path = path.unwrap();

        let uris = self.add_parent_uris_with_path(uris, &path, parent)?;

        // linking all URIs of this schema to the the source location
        self.sources.link_all(id.as_ref(), &uris, &link)?;

        let anchors = dialect.anchors(&source)?;

        let key = self
            .schemas
            .insert(CompiledSchema::new(
                id.clone(),
                path.clone(),
                uris,
                link,
                parent,
                anchors,
            ))
            .map_err(SourceError::SourceConflict)?;

        let subschemas = self
            .compile_subschemas(key, &uri, &path, dialect, &source, dialects.clone())
            .await?;

        let references = self.compile_references(key, &source, dialect).await?;

        // check to ensure that there are no cyclic references
        self.schemas
            .ensure_not_cyclic(key, &uri, &references, self.sources)?;

        let keywords = self.compile_keywords(key, &uri, dialect)?;

        let schema = self.schemas.get_mut(key).unwrap();

        schema.references = references;
        schema.subschemas = subschemas;
        schema.keywords = keywords;
        Ok(key)
    }

    #[allow(clippy::unnecessary_wraps)]
    fn add_parent_uris_with_path(
        &mut self,
        mut uris: Vec<AbsoluteUri>,
        path: &Pointer,
        parent: Option<Key>,
    ) -> Result<Vec<AbsoluteUri>, CompileError> {
        let Some(parent) = parent else { return Ok(uris) };
        let parent = self.schemas.get(parent, self.sources).unwrap();
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
        Ok(uris)
    }

    async fn compile_references(
        &mut self,
        key: Key,
        source: &Value,
        dialect: &Dialect,
    ) -> Result<Vec<Reference>, CompileError> {
        let mut references = dialect.references(source)?;
        for reference in &mut references {
            let (ref_link, _) = self
                .sources
                .resolve(reference.uri.clone(), self.resolvers, self.deserializers)
                .await?;
            let ref_link = ref_link.clone();
            let ref_key = self
                .compile_schema(None, ref_link, None, self.dialects.clone())
                .await?;
            reference.key = ref_key;
            let ref_schema = self.schemas.get_mut(ref_key).unwrap();
            ref_schema.dependents.push(key);
        }
        Ok(references)
    }

    async fn compile_anchored(&mut self, link: Link) -> Result<Key, CompileError> {
        let fragment = link.uri.fragment().unwrap_or_default().trim().to_string();
        // need to compile the root first
        let mut base_uri = link.uri.clone();
        base_uri.set_fragment(None).unwrap();
        let (root_link, _) = self
            .sources
            .resolve(base_uri.clone(), self.resolvers, self.deserializers)
            .await?;
        let root_link = root_link.clone();
        let _ = self
            .compile_schema(
                Some(Pointer::default()),
                root_link,
                None,
                self.dialects.clone(),
            )
            .await?;

        // at this stage, all URIs should be indexed.
        match self.schemas.get_by_uri(&link.uri, self.sources) {
            Some(anchored) => Ok(anchored.key),
            None => Err(UnknownAnchorError {
                anchor: fragment,
                uri: link.uri.clone(),
            }
            .into()),
        }
    }
    async fn compile_ancestors(
        &mut self,
        uri: AbsoluteUri,
        link: Link,
    ) -> Result<Key, CompileError> {
        let link = self.sources.get_link(&link.uri).unwrap().clone();
        let full_path = Pointer::parse(uri.fragment().unwrap())
            .map_err(|err| CompileError::FailedToParsePointer(err.into()))?;
        let mut path = Pointer::default();
        // TODO: remove once i update jsonptr to include the root token in the `Tokens` iter
        // https://github.com/chanced/jsonptr/issues/17
        let uri = link.uri.clone();
        if let Some(key) = self
            .compile_ancestor_then_find_descendant(uri.clone(), path.clone())
            .await
        {
            return Ok(key);
        }
        for tok in full_path.tokens() {
            path.push_back(tok);
            if let Some(key) = self
                .compile_ancestor_then_find_descendant(uri.clone(), path.clone())
                .await
            {
                return Ok(key);
            }
        }
        self.compile_schema(Some(full_path), link, None, self.dialects.clone())
            .await
    }

    async fn compile_ancestor_then_find_descendant(
        &mut self,
        mut uri: AbsoluteUri,
        path: Pointer,
    ) -> Option<Key> {
        uri.set_fragment(Some(&path)).unwrap();
        let link = self.sources.get_link(&uri).unwrap().clone();
        self.compile_schema(
            Some(path.clone()),
            link.clone(),
            None,
            self.dialects.clone(),
        )
        .await
        .ok()
        .and_then(|ancestor| {
            self.schemas
                .descendants(ancestor, self.sources)
                .find_by_uri(&uri)
                .map(|s| s.key)
        })
    }

    async fn compile_subschemas(
        &mut self,
        key: Key,
        base_uri: &AbsoluteUri,
        path: &Pointer,
        dialect: &Dialect,
        source: &Value,
        dialects: Dialects<'i>,
    ) -> Result<Vec<Key>, CompileError> {
        let mut subschemas = Vec::new();
        for subschema_path in dialect.subschemas(path, source) {
            let mut uri = base_uri.clone();
            uri.set_fragment(Some(&subschema_path))?;
            let (sub_link, _) = self
                .sources
                .resolve(uri, self.resolvers, self.deserializers)
                .await?;
            let sub_link = sub_link.clone();
            let subschema = self
                .compile_schema(Some(subschema_path), sub_link, Some(key), dialects.clone())
                .await?;
            subschemas.push(subschema);
        }
        Ok(subschemas)
    }

    fn compile_keywords(
        &mut self,
        key: Key,
        base_uri: &AbsoluteUri,
        dialect: &Dialect,
    ) -> Result<Box<[Box<dyn Keyword>]>, CompileError> {
        let schema = self.schemas.get(key, self.sources).unwrap();

        let mut keywords = Vec::new();
        for mut keyword in dialect.keywords().iter().cloned() {
            let mut compile = Compile {
                base_uri,
                schemas: self.schemas,
                rationals: self.rationals,
                ints: self.ints,
                values: self.values,
                global_state: self.global_state,
            };
            if keyword.compile(&mut compile, schema.clone())? {
                keywords.push(keyword);
            }
        }
        Ok(keywords.into_boxed_slice())
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
    let lookup_id = id.as_ref().unwrap_or(&uris[0]);
    Ok((lookup_id.clone(), id, uris))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_spike() {
        let interrogator = Interrogator::with_json_schema_2020_12()
            .with_json_support()
            .build()
            .await
            .unwrap();
    }
}
