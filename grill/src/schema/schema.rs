use crate::{
    error::{
        self, CompileError, CyclicDependencyError, SourceConflictError, SourceError,
        UnknownAnchorError, UnknownKeyError,
    },
    keyword::{BigInts, BigRationals, Compile, Keyword, Values},
    schema::{
        traverse::{
            AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
            TransitiveDependencies,
        },
        Anchor, Dialect, Dialects, Reference,
    },
    source::{Deserializers, Link, Resolvers, Source, Sources},
    AbsoluteUri,
};
use async_recursion::async_recursion;
use jsonptr::Pointer;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use std::{
    borrow::Cow,
    collections::{hash_map::Entry, HashMap},
    ops::Deref,
    str::FromStr,
};

new_key_type! {
    pub struct Key;
}

#[derive(Clone, Debug)]
pub(crate) struct CompiledSchema {
    /// Abs URI of the schema.
    pub(crate) id: Option<AbsoluteUri>,

    /// Path to the schema from the root schema as a JSON Pointer
    pub(crate) path: Pointer,

    /// The [`Key`] of the schema which contains this schema, if any.
    ///
    /// Note that if this schema has an `id`, `parent` will be `None` regardless of
    /// whether or not this schema is embedded.
    pub(crate) parent: Option<Key>,

    /// Directly embedded subschemas, excluding those with `id`s.
    pub(crate) subschemas: Vec<Key>,

    /// Dependents of this `Schema`.
    pub(crate) dependents: Vec<Key>,

    ///  Referenced dependencies of this `Schema`.
    pub(crate) references: Vec<Reference>,

    /// All anchors defined in this schema and embedded schemas which do not
    /// have `id`s.
    pub(crate) anchors: Vec<Anchor>,

    /// All URIs which this schema is referenced by.
    pub(crate) uris: Vec<AbsoluteUri>,

    /// Abs URI of the schema's `Metaschema`.
    pub(crate) metaschema: AbsoluteUri,

    // Compiled keywords.
    pub(crate) keywords: Box<[Keyword]>,

    /// Absolute URI of the source and path to this schema.
    pub(crate) link: Link,
}

impl CompiledSchema {
    pub(crate) fn new(
        id: Option<AbsoluteUri>,
        path: Pointer,
        uris: Vec<AbsoluteUri>,
        link: Link,
        parent: Option<Key>,
        anchors: Vec<Anchor>,
    ) -> Self {
        Self {
            id,
            path,
            uris,
            metaschema: link.uri.clone(),
            parent,
            link,
            subschemas: Vec::new(),
            dependents: Vec::new(),
            references: Vec::new(),
            anchors,
            keywords: Box::default(),
        }
    }
}
impl PartialEq for CompiledSchema {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema && self.link == other.link
    }
}

impl Eq for CompiledSchema {}

#[derive(Clone, Debug)]
pub struct Schema<'i> {
    /// Key of the `Schema`
    pub key: Key,

    /// The `$id` or `id` of the schema, if any
    pub id: Option<Cow<'i, AbsoluteUri>>,

    /// The path to the schema from the root schema as a JSON Pointer
    pub path: Cow<'i, Pointer>,

    /// All URIs which this schema can be referenced by.
    pub uris: Cow<'i, [AbsoluteUri]>,

    /// The URI of the schema's `Metaschema`.
    pub metaschema: Cow<'i, AbsoluteUri>,

    /// The `Key` of the parent `Schema`, if any.
    ///
    /// Note that if this `Schema` has an `id`, then `parent` will be `None`
    /// regardless of whether or not this schema is embedded.
    pub parent: Option<Key>,

    /// `Schema`s that are directly embedded within this `Schema`
    ///
    /// Note that if any embedded `Schema` has an `id`, then it will not be
    /// be present in this list as per the specification, `Schema`s which are
    /// identified are to be treated as root schemas.
    pub subschemas: Cow<'i, [Key]>,

    /// Anchors within this `Schema`
    pub anchors: Cow<'i, [Anchor]>,

    /// Dependents of this `Schema`.
    pub dependents: Cow<'i, [Key]>,

    ///  Dependencies of this `Schema`.
    pub references: Cow<'i, [Reference]>,

    /// Compiled [`Keyword`]s.
    pub keywords: Cow<'i, [Keyword]>,

    /// The schema's source [`Value`], [`AbsoluteUri`], and path as a JSON
    /// [`Pointer`]
    pub source: Source<'i>,
}

impl<'i> Schema<'i> {
    pub(crate) fn new(key: Key, compiled: &'i CompiledSchema, sources: &'i Sources) -> Self {
        Self {
            key,
            id: compiled.id.as_ref().map(Cow::Borrowed),
            path: Cow::Borrowed(&compiled.path),
            uris: Cow::Borrowed(&compiled.uris),
            metaschema: Cow::Borrowed(&compiled.metaschema),
            anchors: Cow::Borrowed(&compiled.anchors),
            source: Source::new(&compiled.link, sources),
            parent: compiled.parent,
            subschemas: Cow::Borrowed(&compiled.subschemas),
            dependents: Cow::Borrowed(&compiled.dependents),
            references: Cow::Borrowed(&compiled.references),
            keywords: Cow::Borrowed(&compiled.keywords),
        }
    }

    #[must_use]
    pub fn into_owned(self) -> Schema<'static> {
        Schema {
            key: self.key,
            parent: self.parent,
            id: self.id.map(|id| Cow::Owned(id.into_owned())),
            path: Cow::Owned(self.path.into_owned()),
            uris: Cow::Owned(self.uris.into_owned()),
            metaschema: Cow::Owned(self.metaschema.into_owned()),
            source: self.source.into_owned(),
            anchors: Cow::Owned(self.anchors.into_owned()),
            dependents: Cow::Owned(self.dependents.into_owned()),
            references: Cow::Owned(self.references.into_owned()),
            keywords: Cow::Owned(self.keywords.into_owned()),
            subschemas: Cow::Owned(self.subschemas.into_owned()),
        }
    }
}
impl std::ops::Index<&str> for Schema<'_> {
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.source[index]
    }
}

impl<'i> Schema<'i> {
    #[must_use]
    pub fn value(&self) -> &Value {
        &self.source
    }
}

impl<'i> Deref for Schema<'i> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<'i> PartialEq for Schema<'i> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema
    }
}
impl<'i> Eq for Schema<'i> {}

#[derive(Debug, Clone, Default)]
struct Store {
    table: SlotMap<Key, CompiledSchema>,
    index: HashMap<AbsoluteUri, Key>,
}

#[allow(clippy::unnecessary_box_returns)]
impl Store {
    fn get_mut(&mut self, key: Key) -> Option<&mut CompiledSchema> {
        self.table.get_mut(key)
    }
    fn iter(&self) -> slotmap::basic::Iter<'_, Key, CompiledSchema> {
        self.table.iter()
    }
    fn get(&self, key: Key) -> Option<&CompiledSchema> {
        self.table.get(key)
    }
    pub(crate) fn get_index(&self, id: &AbsoluteUri) -> Option<Key> {
        self.index.get(id).copied()
    }
    pub(crate) fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
        self.index.entry(id)
    }
    pub(crate) fn contains_key(&self, key: Key) -> bool {
        self.table.contains_key(key)
    }
    pub(crate) fn contains_uri(&self, uri: &AbsoluteUri) -> bool {
        self.index.contains_key(uri)
    }
    /// Inserts the schema unless it already exists. If it does exist, returns
    /// the existing schema's key.
    ///
    /// # Errors
    /// Returns the URI of the existing schema if it is not equal to the new
    /// schema.
    pub(crate) fn insert(&mut self, schema: CompiledSchema) -> Result<Key, SourceConflictError> {
        let id = schema.id.as_ref().unwrap_or(&schema.uris[0]);
        if let Some(key) = self.index.get(id) {
            let existing = self.table.get(*key).unwrap();
            if existing != &schema {
                return Err(SourceConflictError {
                    uri: id.clone(),
                    existing_source: existing.link.source.clone().into(),
                });
            }
            return Ok(*key);
        }
        let uris = schema.uris.clone();
        let key = self.table.insert(schema);
        for uri in uris {
            self.index.insert(uri, key);
        }
        Ok(key)
    }
}
pub(crate) struct Params<'i> {
    path: Option<Pointer>,
    link: Link,
    parent: Option<Key>,
    sources: &'i mut Sources,
    dialects: &'i Dialects<'i>,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
    ints: &'i mut BigInts,
    rationals: &'i mut BigRationals,
    values: &'i mut Values,
}

struct Subparams<'i> {
    key: Key,
    id: Option<&'i AbsoluteUri>,
    base_uri: &'i AbsoluteUri,
    path: &'i Pointer,
    source: &'i Value,
    dialect: &'i Dialect,
    dialects: &'i Dialects<'i>,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
    sources: &'i mut Sources,
    ints: &'i mut BigInts,
    rationals: &'i mut BigRationals,
    values: &'i mut Values,
}

#[derive(Clone, Debug, Default)]
pub struct Schemas {
    store: Store,
    sandbox: Option<Store>,
}

impl Schemas {
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Store::default(),
            sandbox: None,
        }
    }

    #[allow(clippy::too_many_lines, clippy::too_many_arguments)]
    #[async_recursion]
    pub(crate) async fn compile(&mut self, params: Params) -> Result<Key, CompileError> {
        let Params {
            mut path,
            link,
            parent,
            sources,
            dialects,
            deserializers,
            resolvers,
            ints,
            rationals,
            values,
        } = params;
        // check to see if schema is already been compiled
        if self.sandbox().index.contains_key(&link.uri) {
            // if so, return it.
            return Ok(self.sandbox().get_index(&link.uri).unwrap());
        }
        let source = sources.get(link.key).clone();

        // determine the dialect
        let dialect = dialects.pertinent_to_or_default(&source);

        // identify the schema
        let (id, uris) = dialect.identify(link.uri.clone(), &link.path, &source)?;

        // if identify did not find a primary id, use the uri + pointer fragment
        // as the lookup which will be at the first position in the uris list
        let lookup_id = id.as_ref().unwrap_or(&uris[0]);

        // check to see if the schema has already been compiled under the id
        if let Entry::Occupied(key) = self.sandbox().index_entry(lookup_id.clone()) {
            return Ok(*key.get());
        }

        // if the schema is anchored (i.e. has a non json ptr fragment) then
        // compile the root (non-fragmented uri) and attempt to locate the anchor.
        let fragment = link.uri.fragment().unwrap_or_default().trim().to_string();
        if !fragment.is_empty() && !fragment.starts_with('/') {
            return self
                .compile_anchored(
                    link,
                    sources,
                    dialects,
                    deserializers,
                    resolvers,
                    ints,
                    rationals,
                    values,
                )
                .await;
        }

        // if parent is None and this schema is not a document root (does not
        // have an $id) then attempt to locate the parent using the pointer
        // fragment.
        let fragment = lookup_id.fragment().unwrap_or_default().trim();
        if id.is_none() && parent.is_none() && fragment.starts_with('/') {
            parent = self.locate_parent(lookup_id.clone())?;
        }
        if id.is_some() {
            path = Pointer::default()
        }

        // linking all URIs of this schema to the the source location
        sources.link_all(id.as_ref(), &uris, &link.uri, &link.path)?;

        let base_uri = id.clone().unwrap_or(link.uri.clone());
        let link = sources.get_link(&base_uri).cloned().unwrap();
        let anchors = dialect.anchors(&source)?;

        // create a new CompiledSchema and insert it. if compiling fails, the
        // schema store will rollback to its previous state.
        let key = self.insert_placeholder(
            &id,
            path.clone(),
            uris,
            &link,
            &source,
            parent,
            anchors,
            dialect,
        )?;

        let subschemas = self
            .compile_subschemas(Subparams {
                key,
                id: id.as_ref(),
                base_uri: &base_uri,
                path: &path,
                source: &source,
                dialect,
                dialects,
                deserializers,
                resolvers,
                sources,
                ints,
                rationals,
                values,
            })
            .await?;
        let references = self
            .compile_references(Subparams {
                key,
                id: id.as_ref(),
                base_uri: &base_uri,
                path: &link.path,
                source: &source,
                dialect,
                dialects,
                deserializers,
                resolvers,
                sources,
                ints,
                rationals,
                values,
            })
            .await?;

        // check to ensure that there are not cyclic references
        self.ensure_not_cyclic(key, &base_uri, &references, sources)?;
        let keywords = self
            .compile_keywords(key, dialect, sources, ints, rationals, values)
            .await?;
        let schema = self.sandbox().get_mut(key).unwrap();

        schema.references = references;
        schema.subschemas = subschemas;
        schema.keywords = keywords;
        Ok(key)
    }

    async fn compile_keywords(
        &self,
        key: Key,
        dialect: &Dialect,
        sources: &Sources,
        ints: &mut BigInts,
        rationals: &mut BigRationals,
        values: &mut Values,
    ) -> Result<Box<[Keyword]>, CompileError> {
        let schema = self.get(key, sources).unwrap();

        let mut keywords = Vec::new();
        for mut keyword in dialect.keywords().iter().cloned() {
            let mut compile = Compile {
                base_uri: schema.id.as_deref().unwrap_or(&schema.uris[0]),
                schemas: self,
                rationals,
                ints,
                values,
            };
            if keyword.compile(&mut compile, schema.clone()).await? {
                keywords.push(keyword);
            }
        }
        Ok(keywords.into_boxed_slice())
    }
    pub(crate) fn ensure_not_cyclic(
        &mut self,
        key: Key,
        uri: &AbsoluteUri,
        references: &[Reference],
        sources: &Sources,
    ) -> Result<(), CompileError> {
        for reference in references {
            if self
                .transitive_dependencies(reference.key, sources)
                .any(|schema| schema.key == key)
            {
                return Err(CyclicDependencyError {
                    from: uri.clone(),
                    to: reference.uri.clone(),
                }
                .into());
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn insert_placeholder(
        &mut self,
        id: &Option<AbsoluteUri>,
        path: Pointer,
        uris: Vec<AbsoluteUri>,
        link: &Link,
        source: &Value,
        parent: Option<Key>,
        anchors: Vec<Anchor>,
        dialect: &Dialect,
    ) -> Result<Key, CompileError> {
        let id = id.as_ref();
        self.sandbox()
            .insert(CompiledSchema {
                id: id.cloned(),
                path,
                uris,
                metaschema: dialect.primary_metaschema_id().clone(),
                keywords: Box::default(),
                parent,
                link: link.clone(),
                subschemas: Vec::new(),
                dependents: Vec::new(),
                references: Vec::new(),
                anchors: anchors.clone(),
            })
            .map_err(|uri| {
                SourceError::from(SourceConflictError {
                    uri,
                    existing_source: source.clone().into(),
                })
            })
            .map_err(CompileError::from)
    }

    #[allow(clippy::too_many_arguments)]
    async fn compile_anchored(
        &mut self,
        link: Link,
        sources: &mut Sources,
        dialects: &Dialects<'_>,
        deserializers: &Deserializers,
        resolvers: &Resolvers,
        ints: &mut BigInts,
        rationals: &mut BigRationals,
        values: &mut Values,
    ) -> Result<Key, CompileError> {
        let fragment = link.uri.fragment().unwrap_or_default().trim().to_string();
        // need to compile the root first
        let mut base_uri = link.uri.clone();
        base_uri.set_fragment(None).unwrap();
        let (root_link, _) = sources
            .resolve(base_uri.clone(), resolvers, deserializers)
            .await?;
        let root_link = root_link.clone();
        let _ = self.compile().await?;

        // at this stage, all URIs should be indexed.
        match self.get_by_uri(&link.uri, sources) {
            Some(anchored) => Ok(anchored.key),
            None => Err(UnknownAnchorError {
                anchor: fragment,
                uri: link.uri.clone(),
            }
            .into()),
        }
    }

    async fn compile_subschemas(
        &mut self,
        params: Subparams<'_>,
    ) -> Result<Vec<Key>, CompileError> {
        let Subparams {
            id,
            base_uri,
            path,
            source,
            dialect,
            dialects,
            deserializers,
            resolvers,
            sources,
            key,
            ints,
            rationals,
            values,
        } = params;
        // compile nested schemas
        let mut subschemas = Vec::new();
        let path = if id.is_some() {
            Cow::Owned(Pointer::default())
        } else {
            Cow::Borrowed(path)
        };
        for subschema_path in dialect.subschemas(&path, source) {
            let mut uri = base_uri.clone();
            uri.set_fragment(Some(&subschema_path))?;
            let (sub_link, _) = sources.resolve(uri, resolvers, deserializers).await?;
            let sub_link = sub_link.clone();
            let subschema = self
                .compile(
                    sub_link,
                    Some(key),
                    sources,
                    dialects,
                    deserializers,
                    resolvers,
                    ints,
                    rationals,
                    values,
                )
                .await?;
            subschemas.push(subschema);
        }
        Ok(subschemas)
    }

    fn sandbox(&mut self) -> &mut Store {
        self.sandbox
            .as_mut()
            .expect("transaction failed: schema sandbox not found.\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new")
    }

    fn store(&self) -> &Store {
        if let Some(sandbox) = self.sandbox.as_ref() {
            return sandbox;
        }
        &self.store
    }
    pub(crate) fn get_index(&self, id: &AbsoluteUri) -> Option<Key> {
        self.store().get_index(id)
    }
    pub(crate) fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
        self.sandbox().index_entry(id)
    }
    pub(crate) fn insert(&mut self, schema: CompiledSchema) -> Result<Key, SourceConflictError> {
        self.sandbox().insert(schema)
    }
    pub(crate) fn compiled_iter(&self) -> slotmap::basic::Iter<'_, Key, CompiledSchema> {
        self.store().iter()
    }
    pub(crate) fn ancestors<'i>(&'i self, key: Key, sources: &'i Sources) -> Ancestors<'i> {
        Ancestors::new(key, self, sources)
    }
    pub(crate) fn descendants<'i>(&'i self, key: Key, sources: &'i Sources) -> Descendants<'i> {
        Descendants::new(key, self, sources)
    }

    pub(crate) fn ensure_key_exists<T, F>(&self, key: Key, f: F) -> Result<T, UnknownKeyError>
    where
        F: FnOnce() -> T,
    {
        if self.store().contains_key(key) {
            Ok(f())
        } else {
            Err(UnknownKeyError)
        }
    }

    pub(crate) fn direct_dependents<'i>(
        &'i self,
        key: Key,
        sources: &'i Sources,
    ) -> DirectDependents<'i> {
        DirectDependents::new(key, self, sources)
    }
    pub(crate) fn all_dependents<'i>(
        &'i self,
        key: Key,
        sources: &'i Sources,
    ) -> AllDependents<'i> {
        AllDependents::new(key, self, sources)
    }
    pub(crate) fn transitive_dependencies<'i>(
        &'i self,
        key: Key,
        sources: &'i Sources,
    ) -> TransitiveDependencies<'i> {
        TransitiveDependencies::new(key, self, sources)
    }

    pub(crate) fn direct_dependencies<'i>(
        &'i self,
        key: Key,
        sources: &'i Sources,
    ) -> DirectDependencies<'i> {
        DirectDependencies::new(key, self, sources)
    }

    pub(crate) fn get_unchecked<'i>(&'i self, key: Key, sources: &'i Sources) -> Schema<'i> {
        self.get(key, sources).unwrap()
    }
    /// Returns the [`Schema`] with the given `Key` if it exists.
    pub(crate) fn get<'i>(
        &'i self,
        key: Key,
        sources: &'i Sources,
    ) -> Result<Schema<'i>, UnknownKeyError> {
        let schema = self.store().get(key).ok_or(UnknownKeyError)?;

        Ok(Schema {
            key,
            id: schema.id.as_ref().map(Cow::Borrowed),
            path: Cow::Borrowed(&schema.path),
            metaschema: Cow::Borrowed(&schema.metaschema),
            source: Source::new(&schema.link, sources),
            uris: Cow::Borrowed(&schema.uris),
            keywords: Cow::Borrowed(&schema.keywords),
            parent: schema.parent,
            references: Cow::Borrowed(&schema.references),
            dependents: Cow::Borrowed(&schema.dependents),
            subschemas: Cow::Borrowed(&schema.subschemas),
            anchors: Cow::Borrowed(&schema.anchors),
        })
    }

    /// Returns a mutable reference to the [`CompiledSchema`] with the given `Key` if it exists.
    ///
    /// # Panics
    /// Panics if a transaction has not been started.
    pub(crate) fn get_mut(&mut self, key: Key) -> Option<&mut CompiledSchema> {
        self.sandbox().get_mut(key)
    }
    /// Returns a mutable reference to the [`CompiledSchema`] with the given `Key`.
    ///
    /// # Panics
    /// Panics if:
    /// - a transaction has not been started.
    /// - the `Key` does not exist.
    pub(crate) fn get_mut_unchecked(&mut self, key: Key) -> &mut CompiledSchema {
        self.get_mut(key).unwrap()
    }

    #[must_use]
    pub(crate) fn get_by_uri<'i>(
        &'i self,
        uri: &AbsoluteUri,
        sources: &'i Sources,
    ) -> Option<Schema<'i>> {
        let key = self.store().index.get(uri).copied()?;
        Some(self.get_unchecked(key, sources))
    }

    #[must_use]
    pub(crate) fn get_key_by_id(&self, id: &AbsoluteUri) -> Option<Key> {
        self.get_index(id)
    }

    pub(crate) fn has_path_connecting(&self, from: Key, to: Key) -> bool {
        let from = self.store().get(from).unwrap();
        todo!()
    }

    /// Starts a new transaction.
    pub(crate) fn start_txn(&mut self) {
        assert!(self.sandbox.is_none(), "schema sandbox already exists\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new");
        self.sandbox = Some(self.store.clone());
    }

    /// Accepts the current transaction, committing all changes.
    pub(crate) fn commit_txn(&mut self) {
        let sandbox = self.sandbox.take().expect("sandbox should be present");
        self.store = sandbox;
    }

    /// Rejects the current transaction, discarding all changes.
    pub(crate) fn rollback_txn(&mut self) {
        self.sandbox = None;
    }

    pub(crate) fn contains_key(&self, key: Key) -> bool {
        self.store().contains_key(key)
    }

    pub(crate) fn contains_uri(&self, uri: &str) -> bool {
        self.store()
    }
}
#[cfg(test)]
mod tests {
    use crate::{schema::Schemas, Interrogator};
}
