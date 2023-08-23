use crate::{
    error::{CompileError, SourceConflictError, SourceError, UnknownKeyError},
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

#[derive(Clone, Debug)]
pub(crate) struct Schemas {
    pub(crate) store: SlotMap<Key, CompiledSchema>,
    keys: HashMap<AbsoluteUri, Key>,
    sandbox: Option<Box<Sandbox>>,
}

pub(crate) struct Params<'i> {
    base_uri: AbsoluteUri,
    path: &'i Pointer,
    src: Link,
    parent: Option<Key>,
    sources: &'i mut Sources,
    dialects: &'i Dialects<'i>,
    deserializers: &'i Deserializers,
    resolvers: &'i Resolvers,
}

impl Schemas {
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: SlotMap::default(),
            keys: HashMap::default(),
            sandbox: None,
        }
    }

    pub(crate) async fn compile(&mut self, params: Params<'_>) -> Result<Key, CompileError> {
        let Params {
            base_uri,
            path,
            src,
            mut parent,
            sources,
            dialects,
            deserializers,
            resolvers,
        } = params;
        let source = sources.store.get(src.key).unwrap().clone();
        // determining the dialect
        let dialect = dialects.pertinent_to_or_default(&source);

        // identifying the schema
        let (id, uris) = dialect.identify(base_uri.clone(), path, &source)?;

        // if identify did not find a primary id, use the uri + pointer fragment
        // as the lookup which will be at the first position in the uris list
        let lookup_id = id.as_ref().unwrap_or(&uris[0]);

        // checking to see if the schema has already been compiled under the id

        if let Entry::Occupied(key) = self.keys.entry(lookup_id.clone()) {
            return Ok(*key.get());
        }

        // if parent is None and this schema is not a document root (that is,
        // has an $id) then attempt to locate the parent using the pointer
        // fragment.
        // this shouldn't be used much, if at all, but its here for safety
        if id.is_none()
            && parent.is_none()
            && lookup_id.has_fragment()
            && lookup_id.fragment().unwrap().starts_with('/')
        {
            parent = self.locate_parent(lookup_id.clone())?;
        }
        // create a new CompiledSchema and insert it. if compiling fails, the
        // schema store will rollback to its previous state.
        let key = self
            .insert(CompiledSchema {
                id: id.clone(),
                uris: uris.clone(),
                metaschema: dialect.primary_metaschema_id().clone(),
                handlers: dialect.handlers.clone().into_boxed_slice(),
                parent,
                src: src.clone(),
                subschemas: Vec::default(),
                dependents: Vec::default(),
                references: Vec::default(),
                anchors: Vec::default(),
            })
            .map_err(|uri| {
                SourceError::from(SourceConflictError {
                    uri,
                    value: source.clone().into(),
                })
            })?;

        // linking all URIs of this schema to the the source location
        let base_uri = id.unwrap_or(base_uri);
        for uri in uris {
            sources.link(uri, src.uri.clone(), src.path.clone())?;
        }

        // gather references
        for Reference {
            keyword,
            ref_path,
            uri,
            key,
            src_key,
        } in dialect.references(&source)?
        {
            let mut base_uri = uri.clone();
            let fragment = base_uri.set_fragment(None).unwrap().unwrap_or_default();

            let (_, src) = sources.resolve(&uri, resolvers, deserializers).await?;
            // let ref_key = self.compile(base_uri).await?;
        }

        let mut subschemas = Vec::new();

        // gathering nested schemas
        // for subschema_path in dialect.subschemas(src_path, source) {
        // let subschema = self
        //     .compile(
        //         base_uri.clone(),
        //         &subschema_path,
        //         src,
        //         src_uri.clone(),
        //         src_path,
        //         Some(key),
        //         sources,
        //         dialects,
        //         deserializers,
        //         resolvers,
        //     )
        //     .await?;
        // subschemas.push(subschema);
        // }

        let schema = self.get_mut_unchecked(key);
        schema.subschemas = subschemas;

        todo!()
    }

    pub(crate) fn insert(&mut self, schema: CompiledSchema) -> Result<Key, AbsoluteUri> {
        self.sandbox
            .as_deref_mut()
            .expect("sandbox not present")
            .insert(schema)
    }
    pub(crate) fn compiled_iter(&self) -> slotmap::basic::Iter<'_, Key, CompiledSchema> {
        if let Some(sandbox) = self.sandbox.as_ref() {
            sandbox.store.iter()
        } else {
            self.store.iter()
        }
    }
    pub(crate) fn ancestors<'i>(&'i self, key: Key, sources: &'i Sources) -> Ancestors<'i> {
        Ancestors::new(key, self, sources)
    }
    pub(crate) fn descendants<'i>(&'i self, key: Key, sources: &'i Sources) -> Descendants<'i> {
        Descendants::new(key, self, sources)
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
        let schema = if let Some(sandbox) = self.sandbox.as_ref() {
            sandbox.get(key)
        } else {
            self.store.get(key)
        }
        .ok_or(UnknownKeyError)?;

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

    pub(crate) fn get_mut(&mut self, key: Key) -> Option<&mut CompiledSchema> {
        if let Some(sandbox) = self.sandbox.as_mut() {
            sandbox.get_mut(key)
        } else {
            self.store.get_mut(key)
        }
    }
    pub(crate) fn get_mut_unchecked(&mut self, key: Key) -> &mut CompiledSchema {
        self.get_mut(key).unwrap()
    }

    #[must_use]
    pub(crate) fn get_by_uri<'i>(
        &'i self,
        uri: &AbsoluteUri,
        sources: &'i Sources,
    ) -> Option<Schema<'i>> {
        let key = self.keys.get(uri).copied()?;
        Some(self.get_unchecked(key, sources))
    }

    #[must_use]
    pub(crate) fn get_key_by_id(&self, id: &AbsoluteUri) -> Option<Key> {
        self.keys.get(id).copied()
    }

    pub(crate) fn has_path_connecting(&self, from: Key, to: Key) -> bool {
        let from = self.store.get(from).unwrap();
        todo!()
    }

    // pub(crate) fn transitive_dependencies(of: Key) -> Iter

    fn keys(&self) -> &HashMap<AbsoluteUri, Key> {
        if let Some(sandbox) = self.sandbox.as_deref() {
            return &sandbox.keys;
        }
        &self.keys
    }

    fn keys_mut(&mut self) -> &mut HashMap<AbsoluteUri, Key> {
        &mut self
            .sandbox
            .as_deref_mut()
            .expect("transaction not started")
            .keys
    }

    fn schemas(&self) -> &SlotMap<Key, CompiledSchema> {
        if let Some(sandbox) = self.sandbox.as_deref() {
            return &sandbox.store;
        }
        &self.store
    }

    fn schemas_mut(&mut self) -> &mut SlotMap<Key, CompiledSchema> {
        &mut self
            .sandbox
            .as_deref_mut()
            .expect("transaction not started")
            .store
    }

    /// Starts a new transaction.
    pub(crate) fn start_txn(&mut self) {
        assert!(self.sandbox.is_none(), "sandbox already exists\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new");
        self.sandbox = Sandbox::new(&self.store, &self.keys).into();
    }

    /// Accepts the current transaction, committing all changes.
    pub(crate) fn accept_txn(&mut self) {
        let sandbox = self.sandbox.take().expect("sandbox should be present");
        self.keys = sandbox.keys;
        self.store = sandbox.store;
    }

    /// Rejects the current transaction, discarding all changes.
    pub(crate) fn rollback_txn(&mut self) {
        self.sandbox = None;
    }

    pub(crate) fn contains_key(&self, key: Key) -> bool {
        self.store.contains_key(key)
    }
}
impl Default for Schemas {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
struct Sandbox {
    store: SlotMap<Key, CompiledSchema>,
    keys: HashMap<AbsoluteUri, Key>,
}

#[allow(clippy::unnecessary_box_returns)]
impl Sandbox {
    fn new(schemas: &SlotMap<Key, CompiledSchema>, keys: &HashMap<AbsoluteUri, Key>) -> Box<Self> {
        Box::new(Self {
            store: schemas.clone(),
            keys: keys.clone(),
        })
    }
    fn get_mut(&mut self, key: Key) -> Option<&mut CompiledSchema> {
        self.store.get_mut(key)
    }

    fn get(&self, key: Key) -> Option<&CompiledSchema> {
        self.store.get(key)
    }

    fn insert(&mut self, schema: CompiledSchema) -> Result<Key, AbsoluteUri> {
        let id = schema.id.as_ref().unwrap_or(&schema.uris[0]);
        if let Some(key) = self.keys.get(id) {
            let existing = self.store.get(*key).unwrap();
            if existing != &schema {
                return Err(id.clone());
            }
            return Ok(*key);
        }
        let uris = schema.uris.clone();
        let key = self.store.insert(schema);
        for uri in uris {
            self.keys.insert(uri, key);
        }
        Ok(key)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {}
}
