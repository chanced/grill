use crate::{
    error::{CompileError, CyclicDependencyError, SourceConflictError, UnknownKeyError},
    keyword::{BigInts, BigRationals, Keyword, Values},
    schema::{
        traverse::{
            AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
            TransitiveDependencies,
        },
        Anchor, Dialect, Reference,
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
};

use super::dialect::Dialects;

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
    pub(crate) keywords: Box<[Box<dyn Keyword>]>,

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
    pub keywords: Cow<'i, [Box<dyn Keyword>]>,

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
                return Err(SourceConflictError { uri: id.clone() });
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

    pub(crate) fn contains_uri(&self, uri: &AbsoluteUri) -> bool {
        self.store().contains_uri(uri)
    }
}
#[cfg(test)]
mod tests {
    use crate::{schema::Schemas, Interrogator};
}
