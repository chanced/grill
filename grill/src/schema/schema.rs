use crate::{handler::Handler, source::Sources, AbsoluteUri};
use jsonptr::Pointer;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet, VecDeque},
    iter::Copied,
    slice,
};
use tap::Tap;

use super::Anchor;

new_key_type! {
    pub struct SchemaKey;
}

#[derive(Clone, Debug)]
pub(crate) struct CompiledSchema<Key: slotmap::Key = SchemaKey> {
    /// Abs URI of the schema.
    pub(crate) id: Option<AbsoluteUri>,
    /// The [`Key`] of the schema which contains this schema, if any.
    ///
    /// Note that if this schema has an `id`, `parent` will be `None` regardless of
    /// whether or not this schema is embedded.
    pub(crate) container: Option<Key>,
    /// Directly embedded subschemas, excluding those with `id`s.
    pub(crate) subschemas: Box<[Key]>,
    /// Dependents of this `Schema`.
    pub(crate) dependents: Box<[Key]>,
    ///  Dependencies of this `Schema`.
    pub(crate) dependencies: Box<[Key]>,
    /// All anchors defined in this schema and embedded schemas which do not
    /// have `id`s.
    pub(crate) anchors: Box<[Anchor]>,
    /// All URIs which this schema is referenced by.
    pub(crate) uris: Box<[AbsoluteUri]>,
    /// Abs URI of the schema's `Metaschema`.
    pub(crate) metaschema: AbsoluteUri,
    // Compiled handlers.
    pub(crate) handlers: Box<[Handler]>,
    /// Abs URI of the source.
    pub(crate) source_uri: AbsoluteUri,
    /// Path to the schema within the source as a JSON pointer.
    pub(crate) source_path: Pointer,
}

impl<Key: slotmap::Key> PartialEq for CompiledSchema<Key> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.metaschema == other.metaschema
            && self.source_path == other.source_path
            && self.source_uri == other.source_uri
    }
}

impl<Key: slotmap::Key> Eq for CompiledSchema<Key> {}

#[derive(Clone, Debug)]
pub struct Schema<'i, Key: slotmap::Key> {
    /// Key of the `Schema`
    pub key: Key,
    /// The `$id` or `id` of the schema, if any
    pub id: Option<Cow<'i, AbsoluteUri>>,
    /// All URIs which this schema can be referenced by.
    pub uris: Cow<'i, [AbsoluteUri]>,
    /// The URI of the schema's `Metaschema`.
    pub metaschema: Cow<'i, AbsoluteUri>,
    /// The source of the schema.
    pub source: Cow<'i, Value>,
    /// The [`Key`] of the schema which contains this schema, if any. Note that
    /// if this schema has an `id`, `parent` will be `None` regardless of
    /// whether or not this schema is embedded.
    pub container: Option<Key>,
    /// Dependents of this `Schema`.
    pub dependents: Cow<'i, [Key]>,
    ///  Dependencies of this `Schema`.
    pub dependencies: Cow<'i, [Key]>,
    /// Compiled [`Handler`]s.
    pub handlers: Cow<'i, [Handler]>,
    /// [`AbsoluteUri`] of the source.
    pub source_uri: Cow<'i, AbsoluteUri>,
    /// Path to the schema within the source as a JSON [`Pointer`].
    pub source_path: Cow<'i, Pointer>,
}

impl<'i, Key: slotmap::Key> Schema<'i, Key> {
    pub(crate) fn new(key: Key, compiled: &'i CompiledSchema<Key>, sources: &'i Sources) -> Self {
        let source = sources
            .get(&compiled.source_uri)
            .expect("source_uri not found in Sources");
        let source = compiled
            .source_path
            .resolve(source)
            .expect("sourece_path not found in Source");

        Self {
            key,
            id: compiled.id.as_ref().map(|id| Cow::Borrowed(id)),
            uris: Cow::Borrowed(&compiled.uris),
            metaschema: Cow::Borrowed(&compiled.metaschema),
            source: Cow::Borrowed(source),
            container: compiled.container,
            dependents: Cow::Borrowed(&compiled.dependencies),
            dependencies: Cow::Borrowed(&compiled.dependencies),
            handlers: Cow::Borrowed(&compiled.handlers),
            source_uri: Cow::Borrowed(&compiled.source_uri),
            source_path: Cow::Borrowed(&compiled.source_path),
        }
    }

    pub fn into_owned(self) -> Schema<'static, Key> {
        Schema {
            key: self.key,
            id: self.id.map(|id| Cow::Owned(id.into_owned())),
            uris: Cow::Owned(self.uris.into_owned()),
            metaschema: Cow::Owned(self.metaschema.into_owned()),
            source: Cow::Owned(self.source.into_owned()),
            dependents: Cow::Owned(self.dependents.into_owned()),
            dependencies: Cow::Owned(self.dependencies.into_owned()),
            handlers: Cow::Owned(self.handlers.into_owned()),
            source_uri: Cow::Owned(self.source_uri.into_owned()),
            source_path: Cow::Owned(self.source_path.into_owned()),
            container: self.container,
        }
    }
}

impl<'i, Key: slotmap::Key> Schema<'i, Key> {
    pub fn value(&self) -> &Value {
        self.source_path.resolve(self.source.as_ref()).unwrap()
    }
}

impl<'i, Key: slotmap::Key> PartialEq for Schema<'i, Key> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema
    }
}

impl<'i, Key: slotmap::Key> Eq for Schema<'i, Key> {}

#[derive(Clone, Debug)]
pub(crate) struct Schemas<Key: slotmap::Key = SchemaKey> {
    schemas: SlotMap<Key, CompiledSchema<Key>>,
    keys: HashMap<AbsoluteUri, Key>,
    sandbox: Option<Box<Sandbox<Key>>>,
}

impl<Key: slotmap::Key> Schemas<Key> {
    /// Creates a new [`Schemas<Key>`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            schemas: SlotMap::default(),
            keys: HashMap::default(),
            sandbox: None,
        }
    }

    /// Returns the [`Schema`] with the given `Key` if it exists.
    #[must_use]
    pub(crate) fn get<'i>(&'i self, key: Key, sources: &'i Sources) -> Option<Schema<'i, Key>> {
        let schema = self.schemas.get(key)?;
        let source = sources.get(&schema.source_uri)?;
        Some(Schema {
            key,
            id: schema.id.as_ref().map(Cow::Borrowed),
            metaschema: Cow::Borrowed(&schema.metaschema),
            source: Cow::Borrowed(source),
            uris: Cow::Borrowed(&schema.uris),
            handlers: Cow::Borrowed(&schema.handlers),
            container: schema.container,
            dependencies: Cow::Borrowed(&schema.dependencies),
            dependents: Cow::Borrowed(&schema.dependents),
            source_uri: Cow::Borrowed(&schema.source_uri),
            source_path: Cow::Borrowed(&schema.source_path),
        })
    }

    pub(crate) fn insert(&mut self, schema: CompiledSchema<Key>) -> Result<Key, AbsoluteUri> {
        self.sandbox
            .as_deref_mut()
            .expect("sandbox not present")
            .insert(schema)
    }

    #[must_use]
    pub(crate) fn get_by_uri<'i>(
        &'i self,
        uri: &AbsoluteUri,
        sources: &'i Sources,
    ) -> Option<Schema<'i, Key>> {
        let key = self.keys.get(uri).copied()?;
        self.get(key, sources)
    }

    #[must_use]
    pub(crate) fn get_key_by_id(&self, id: &AbsoluteUri) -> Option<Key> {
        self.keys.get(id).copied()
    }

    pub(crate) fn has_path_connecting(&self, from: Key, to: Key) -> bool {
        let from = self.schemas.get(from).unwrap();
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

    fn schemas(&self) -> &SlotMap<Key, CompiledSchema<Key>> {
        if let Some(sandbox) = self.sandbox.as_deref() {
            return &sandbox.schemas;
        }
        &self.schemas
    }

    fn schemas_mut(&mut self) -> &mut SlotMap<Key, CompiledSchema<Key>> {
        &mut self
            .sandbox
            .as_deref_mut()
            .expect("transaction not started")
            .schemas
    }

    /// Starts a new transaction.
    pub(crate) fn start_txn(&mut self) {
        assert!(self.sandbox.is_none(), "sandbox already exists\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new");
        self.sandbox = Sandbox::new(&self.schemas, &self.keys).into();
    }

    /// Accepts the current transaction, committing all changes.
    pub(crate) fn accept_txn(&mut self) {
        let sandbox = self.sandbox.take().expect("sandbox should be present");
        self.keys = sandbox.keys;
        self.schemas = sandbox.schemas;
    }

    /// Rejects the current transaction, discarding all changes.
    pub(crate) fn rollback_txn(&mut self) {
        self.sandbox = None;
    }
}
impl<Key: slotmap::Key> Default for Schemas<Key> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
struct Sandbox<Key: slotmap::Key> {
    schemas: SlotMap<Key, CompiledSchema<Key>>,
    keys: HashMap<AbsoluteUri, Key>,
}

impl<Key: slotmap::Key> Sandbox<Key> {
    fn new(
        schemas: &SlotMap<Key, CompiledSchema<Key>>,
        keys: &HashMap<AbsoluteUri, Key>,
    ) -> Box<Self> {
        Box::new(Self {
            schemas: schemas.clone(),
            keys: keys.clone(),
        })
    }

    fn insert(&mut self, schema: CompiledSchema<Key>) -> Result<Key, AbsoluteUri> {
        let id = schema.id.as_ref().unwrap_or(&schema.uris[0]);
        if let Some(key) = self.keys.get(id) {
            let existing = self.schemas.get(*key).unwrap();
            if existing != &schema {
                return Err(id.clone());
            }
            return Ok(*key);
        }
        let key = self.schemas.insert(schema);
        // self.dep_graph.insert(key);
        // for uri in schema.uris.iter().cloned() {
        //     self.keys.insert(uri, key);
        // }
        // Ok(key)
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_name() {}
}
