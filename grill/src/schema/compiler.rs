use std::{borrow::Cow, collections::hash_map::Entry};

use async_recursion::async_recursion;
use jsonptr::Pointer;
use serde_json::Value;

use crate::{
    error::{CompileError, SourceError, UnknownAnchorError},
    interrogator::state::State,
    keyword::{BigInts, BigRationals, Compile, Keyword, Values},
    schema::{Dialects, Schemas},
    source::{Deserializers, Link, Resolvers, Sources},
    AbsoluteUri, Interrogator, Key,
};

use super::{CompiledSchema, Dialect, Reference};

pub(crate) struct Compiler<'i> {
    schemas: &'i mut Schemas,
    sources: &'i mut Sources,
    local_state: State,
    global_state: &'i mut State,
    dialects: &'i Dialects<'i>,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
    ints: &'i mut BigInts,
    rationals: &'i mut BigRationals,
    values: &'i mut Values,
}

impl<'i> Compiler<'i> {
    pub(crate) fn new(interrogator: &mut Interrogator) -> Self {
        Self {
            schemas: &mut interrogator.schemas,
            sources: &mut interrogator.sources,
            local_state: State::default(),
            global_state: &mut interrogator.state,
            dialects: &interrogator.dialects,
            deserializers: &interrogator.deserializers,
            resolvers: &interrogator.resolvers,
            ints: &mut interrogator.ints,
            rationals: &mut interrogator.rationals,
            values: &mut interrogator.values,
        }
    }

    #[async_recursion]
    pub(crate) async fn compile_schema(
        &mut self,
        mut path: Option<Pointer>,
        link: Link,
        mut parent: Option<Key>,
    ) -> Result<Key, CompileError> {
        // check to see if schema is already been compiled
        if self.schemas.contains_uri(&link.uri) {
            // if so, return it.
            return Ok(self.schemas.get_index(&link.uri).unwrap());
        }
        let source = self.sources.get(link.key).clone();

        // determine the dialect
        let dialect = self.dialects.pertinent_to_or_default(&source);

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
            return self.compile_via_ancestors(uri, link).await;
        } else if is_anchored(&uri) {
            return self.compile_anchored(link).await;
        } else if parent.is_none() && path.is_none() && !has_ptr_fragment(&uri) {
            // if the uri does not have a pointer fragment, then it should be
            // compiled as document root
            path = Some(Pointer::default());
        }

        // path should now have a value
        let path = path.unwrap();

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
            .compile_subschemas(key, &uri, &path, dialect, &source)
            .await?;

        let references = self.compile_references(key, &source, dialect).await?;

        // check to ensure that there are no cyclic references
        self.schemas
            .ensure_not_cyclic(key, &uri, &references, self.sources)?;

        let keywords = self.compile_keywords(key, &uri, dialect).await?;

        let schema = self.schemas.get_mut(key).unwrap();

        schema.references = references;
        schema.subschemas = subschemas;
        schema.keywords = keywords;

        Ok(key)
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
            let ref_key = self.compile_schema(None, ref_link, None).await?;
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
            .compile_schema(Some(Pointer::default()), root_link, None)
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

    async fn compile_via_ancestors(
        &mut self,
        uri: AbsoluteUri,
        link: Link,
    ) -> Result<Key, CompileError> {
        let mut path = Pointer::parse(uri.fragment().unwrap())
            .map_err(|err| CompileError::PointerFailedToParse(err.into()))?;

        while let Some(tok) = path.pop_back() {
            todo!()
        }
    }

    async fn compile_subschemas(
        &mut self,
        key: Key,
        base_uri: &AbsoluteUri,
        path: &Pointer,
        dialect: &Dialect,
        source: &Value,
    ) -> Result<Vec<Key>, CompileError> {
        let mut subschemas = Vec::new();
        for subschema_path in dialect.subschemas(&path, source) {
            let mut uri = base_uri.clone();
            uri.set_fragment(Some(&subschema_path))?;
            let (sub_link, _) = self
                .sources
                .resolve(uri, self.resolvers, self.deserializers)
                .await?;
            let sub_link = sub_link.clone();
            let subschema = self
                .compile_schema(Some(subschema_path), sub_link, Some(key))
                .await?;
            subschemas.push(subschema);
        }
        Ok(subschemas)
    }

    async fn compile_keywords(
        &mut self,
        key: Key,
        base_uri: &AbsoluteUri,
        dialect: &Dialect,
    ) -> Result<Box<[Keyword]>, CompileError> {
        let schema = self.schemas.get(key, self.sources).unwrap();

        let mut keywords = Vec::new();
        for mut keyword in dialect.keywords().iter().cloned() {
            let mut compile = Compile {
                base_uri,
                schemas: &self.schemas,
                rationals: self.rationals,
                ints: self.ints,
                values: self.values,
            };
            if keyword.compile(&mut compile, schema.clone()).await? {
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
    let (id, uris) = dialect.identify(link.uri.clone(), &link.path, &source)?;
    // if identify did not find a primary id, use the uri + pointer fragment
    // as the lookup which will be at the first position in the uris list
    let lookup_id = id.as_ref().unwrap_or(&uris[0]);
    Ok((lookup_id.clone(), id, uris))
}
