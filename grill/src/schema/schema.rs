use crate::{
    error::{CompileError, SourceConflictError, SourceError, UnknownAnchorError, UnknownKeyError},
    handler::Handler,
    schema::{
        traverse::{
            AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
            TransitiveDependencies,
        },
        Anchor, Dialects, Reference,
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

    // Compiled handlers.
    pub(crate) handlers: Box<[Handler]>,

    /// Abs URI of the source.
    pub(crate) src: Link,
}

impl PartialEq for CompiledSchema {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema && self.src == other.src
    }
}

impl Eq for CompiledSchema {}

#[derive(Clone, Debug)]
pub struct Schema<'i> {
    /// Key of the `Schema`
    pub key: Key,
    /// The `$id` or `id` of the schema, if any
    pub id: Option<Cow<'i, AbsoluteUri>>,
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
    /// Compiled [`Handler`]s.
    pub handlers: Cow<'i, [Handler]>,
    /// The schema's source [`Value`], [`AbsoluteUri`], and path as a JSON
    /// [`Pointer`]
    pub source: Source<'i>,
}

impl<'i> Schema<'i> {
    pub(crate) fn new(key: Key, compiled: &'i CompiledSchema, sources: &'i Sources) -> Self {
        Self {
            key,
            id: compiled.id.as_ref().map(Cow::Borrowed),
            uris: Cow::Borrowed(&compiled.uris),
            metaschema: Cow::Borrowed(&compiled.metaschema),
            anchors: Cow::Borrowed(&compiled.anchors),
            source: Source::new(&compiled.src, sources),
            parent: compiled.parent,
            subschemas: Cow::Borrowed(&compiled.subschemas),
            dependents: Cow::Borrowed(&compiled.dependents),
            references: Cow::Borrowed(&compiled.references),
            handlers: Cow::Borrowed(&compiled.handlers),
        }
    }

    #[must_use]
    pub fn into_owned(self) -> Schema<'static> {
        Schema {
            key: self.key,
            parent: self.parent,
            id: self.id.map(|id| Cow::Owned(id.into_owned())),
            uris: Cow::Owned(self.uris.into_owned()),
            metaschema: Cow::Owned(self.metaschema.into_owned()),
            source: self.source.into_owned(),
            anchors: Cow::Owned(self.anchors.into_owned()),
            dependents: Cow::Owned(self.dependents.into_owned()),
            references: Cow::Owned(self.references.into_owned()),
            handlers: Cow::Owned(self.handlers.into_owned()),
            subschemas: Cow::Owned(self.subschemas.into_owned()),
        }
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
    fn new(store: SlotMap<Key, CompiledSchema>, keys: HashMap<AbsoluteUri, Key>) -> Box<Self> {
        Box::new(Self {
            table: store,
            index: keys,
        })
    }
    fn get_mut(&mut self, key: Key) -> Option<&mut CompiledSchema> {
        self.table.get_mut(key)
    }
    fn iter(&self) -> slotmap::basic::Iter<'_, Key, CompiledSchema> {
        self.table.iter()
    }
    fn get(&self, key: Key) -> Option<&CompiledSchema> {
        self.table.get(key)
    }

    fn get_index(&self, id: &AbsoluteUri) -> Option<Key> {
        self.index.get(id).copied()
    }

    fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
        self.index.entry(id)
    }

    fn insert(&mut self, schema: CompiledSchema) -> Result<Key, AbsoluteUri> {
        let id = schema.id.as_ref().unwrap_or(&schema.uris[0]);
        if let Some(key) = self.index.get(id) {
            let existing = self.table.get(*key).unwrap();
            if existing != &schema {
                return Err(id.clone());
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

    fn contains_key(&self, key: Key) -> bool {
        self.table.contains_key(key)
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Schemas {
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

    #[allow(clippy::too_many_lines)]
    #[async_recursion]
    pub(crate) async fn compile(
        &mut self,
        link: Link,
        mut parent: Option<Key>,
        sources: &mut Sources,
        dialects: &Dialects<'_>,
        deserializers: &Deserializers,
        resolvers: &Resolvers,
    ) -> Result<Key, CompileError> {
        let store = self.store_mut();

        // checking to ensure that the schema has not already been compiled
        if store.index.contains_key(&link.uri) {
            // if so, return it.
            return Ok(store.get_index(&link.uri).unwrap());
        }

        let fragment = link.uri.fragment().unwrap_or_default().trim().to_string();
        if !fragment.is_empty() && !fragment.starts_with('/') {
            // this schema is anchored.. we need to compile the root first if it doesn't already exist.
            let mut base_uri = link.uri.clone();
            base_uri.set_fragment(None).unwrap();
            let (root_link, _) = sources
                .resolve(base_uri.clone(), resolvers, deserializers)
                .await?;
            let root_link = root_link.clone();
            let _ = self
                .compile(root_link, None, sources, dialects, deserializers, resolvers)
                .await?;

            // at this stage, all URIs should be indexed.
            match self.get_by_uri(&link.uri, sources) {
                Some(anchored) => return Ok(anchored.key),
                None => {
                    return Err(UnknownAnchorError {
                        anchor: fragment,
                        uri: link.uri.clone(),
                    }
                    .into())
                }
            }
        }
        let source = sources.get(link.key).clone();

        // determine the dialect
        let dialect = dialects.pertinent_to_or_default(&source);

        // identify the schema
        let (id, uris) = dialect.identify(link.uri.clone(), &link.path, &source)?;

        // if identify did not find a primary id, use the uri + pointer fragment
        // as the lookup which will be at the first position in the uris list
        let lookup_id = id.as_ref().unwrap_or(&uris[0]);

        // checking to see if the schema has already been compiled under the id
        if let Entry::Occupied(key) = self.index_entry(lookup_id.clone()) {
            return Ok(*key.get());
        }

        // if parent is None and this schema is not a document root (does not
        // have an $id) then attempt to locate the parent using the pointer
        // fragment.
        if id.is_none()
            && parent.is_none()
            && lookup_id.has_fragment()
            && lookup_id.fragment().unwrap().starts_with('/')
        {
            parent = self.locate_parent(lookup_id.clone())?;
        }

        // linking all URIs of this schema to the the source location
        for uri in &uris {
            sources.link(uri.clone(), link.uri.clone(), link.path.clone())?;
        }

        let base_uri = id.clone().unwrap_or(link.uri.clone());
        let link = sources.get_link(&base_uri).cloned().unwrap();

        let anchors = dialect.anchors(&source)?;
        // create a new CompiledSchema and insert it. if compiling fails, the
        // schema store will rollback to its previous state.

        let key = self
            .insert(CompiledSchema {
                id: id.clone(),
                uris,
                metaschema: dialect.primary_metaschema_id().clone(),
                handlers: dialect.handlers().to_vec().into_boxed_slice(),
                parent,
                src: link.clone(),
                subschemas: Vec::default(), // set below
                dependents: Vec::default(), // set below
                references: Vec::default(), // set below
                anchors: anchors.clone(),
            })
            .map_err(|uri| {
                SourceError::from(SourceConflictError {
                    uri,
                    existing_source: source.clone().into(),
                })
            })?;

        // resolving & compiling references

        let mut references = dialect.references(&source)?;
        for reference in &mut references {
            let (ref_link, _) = sources
                .resolve(reference.uri.clone(), resolvers, deserializers)
                .await?;
            let ref_link = ref_link.clone();
            let ref_key = self
                .compile(
                    ref_link,
                    Some(key),
                    sources,
                    dialects,
                    deserializers,
                    resolvers,
                )
                .await?;
            reference.key = ref_key;
        }

        // compiling nested schemas

        let mut subschemas = Vec::new();

        let path = if id.is_some() {
            Cow::Owned(Pointer::default())
        } else {
            Cow::Borrowed(&link.path)
        };
        for subschema_path in dialect.subschemas(&path, &source) {
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
                )
                .await?;
            subschemas.push(subschema);
        }

        let schema = self.get_mut_unchecked(key);
        schema.subschemas = subschemas;

        todo!()
    }
    fn sandbox(&mut self) -> &mut Store {
        self.sandbox
            .as_mut()
            .expect("transaction failed: schema sandbox not found.\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new")
    }

    fn store_mut(&mut self) -> &mut Store {
        self.sandbox()
    }

    fn store(&self) -> &Store {
        if let Some(sandbox) = self.sandbox.as_ref() {
            return sandbox;
        }
        &self.store
    }
    fn get_index(&self, id: &AbsoluteUri) -> Option<Key> {
        self.store().get_index(id)
    }
    fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
        self.store_mut().index_entry(id)
    }
    pub(crate) fn insert(&mut self, schema: CompiledSchema) -> Result<Key, AbsoluteUri> {
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
            metaschema: Cow::Borrowed(&schema.metaschema),
            source: Source::new(&schema.src, sources),
            uris: Cow::Borrowed(&schema.uris),
            handlers: Cow::Borrowed(&schema.handlers),
            parent: schema.parent,
            references: Cow::Borrowed(&schema.references),
            dependents: Cow::Borrowed(&schema.dependents),
            subschemas: Cow::Borrowed(&schema.subschemas),
            anchors: Cow::Borrowed(&schema.anchors),
        })
    }

    pub(crate) fn locate_parent(
        &mut self,
        mut base: AbsoluteUri,
    ) -> Result<Option<Key>, CompileError> {
        let ptr = Pointer::from_str(base.fragment().unwrap()).map_err(|e| {
            crate::error::LocatedSchemaUriPointerError {
                source: e,
                uri: base.clone(),
            }
        })?;
        let mut path = Pointer::default();
        base.set_fragment(None).unwrap();
        for idx in 0..ptr.count() {
            path.push_front(ptr.get(idx).unwrap());
            base.set_fragment(Some(&path))?;
            if let Some(key) = self.get_key_by_id(&base) {
                return Ok(Some(key));
            }
        }
        Ok(None)
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
}

#[cfg(test)]
mod tests {
    use crate::{schema::Schemas, Interrogator};
}
