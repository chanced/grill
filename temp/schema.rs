//! Resources for working with schemas.

pub mod iter;

pub mod traverse;

pub mod dialect;

use crate::error::{self};
use crate::language::Language;
use crate::{cache, Key};

pub use dialect::{Dialect, Dialects};
use either::Either;
use serde::{Serialize, Serializer};
use snafu::Backtrace;

use grill_uri::{AbsoluteUri, Uri};

use crate::{
    language::Keyword,
    schema::traverse::{
        AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
        TransitiveDependencies,
    },
    source::{Link, Source, Sources},
};

use jsonptr::Pointer;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use std::{borrow::Cow, collections::HashMap, hash::Hash, ops::Deref};


/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                  Key                                  ║
║                                  ¯¯¯                                  ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

new_key_type! {
    /// A unique identifier for a schema.
    pub struct DefaultKey;
}

/// An anchored location within a schema document
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor {
    /// Value of the anchor.  
    pub name: String,
    /// Path to the anchor
    pub path: Cow<'static, Pointer>,
    /// The keyword of the anchor, e.g. `"$anchor"`, `"$dynamicAnchor"`, `"$recursiveAnchor"`
    pub keyword: &'static str,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             CompiledSchema                            ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Clone, Debug)]
pub(crate) struct CompiledSchema<W, K: 'static + Key> {
    /// Abs URI of the schema.
    pub id: Option<AbsoluteUri>,

    /// Path to the schema from the root schema as a JSON Pointer
    pub path: Pointer,

    /// The [`Key`] of the schema which contains this schema, if any.
    ///
    /// Note that if this schema has an `id`, `parent` will be `None` regardless of
    /// whether or not this schema is embedded.
    pub parent: Option<K>,

    /// Directly embedded subschemas, excluding those with `id`s.
    pub subschemas: Vec<K>,

    /// Dependents of this `Schema`.
    pub dependents: Vec<K>,

    ///  Referenced dependencies of this `Schema`.
    pub references: Vec<Reference<K>>,

    /// All anchors defined in this schema and embedded schemas which do not
    /// have `id`s.
    pub anchors: Vec<Anchor>,

    /// All URIs which this schema is referenced by.
    pub uris: Vec<AbsoluteUri>,

    /// Abs URI of the schema's `Metaschema`.
    pub metaschema: AbsoluteUri,

    // Compiled keywords.
    pub keywords: Box<[W]>,

    /// Absolute URI of the source and path to this schema.
    pub link: Link,

    pub compiled: bool,
}

impl<L, K> CompiledSchema<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub(crate) fn new(
        id: Option<AbsoluteUri>,
        path: Pointer,
        uris: Vec<AbsoluteUri>,
        link: Link,
        anchors: Vec<Anchor>,
        parent: Option<K>,
        metaschema: AbsoluteUri,
    ) -> CompiledSchema<L, K> {
        Self {
            id,
            path,
            uris,
            metaschema,
            parent,
            link,
            anchors,
            subschemas: Vec::default(),
            dependents: Vec::default(),
            references: Vec::default(),
            keywords: Box::default(),
            compiled: false,
        }
    }
}
impl<L, K> CompiledSchema<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Returns most relevant URI for the schema, either using the `$id` or the
    /// most relevant as determined by the schema's ancestory or source.
    #[must_use]
    pub(crate) fn absolute_uri(&self) -> &AbsoluteUri {
        self.id.as_ref().unwrap_or(&self.uris[0])
    }
}

impl<L, K> PartialEq for CompiledSchema<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema && self.link == other.link
    }
}

impl<L, K> Eq for CompiledSchema<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Schema                                ║
║                                 ¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A compiled schema.
#[derive(Clone, Debug)]
pub struct Schema<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Key of the `Schema`
    pub key: K,

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
    pub parent: Option<K>,

    /// `Schema`s that are directly embedded within this `Schema`
    ///
    /// Note that if any embedded `Schema` has an `id`, then it will not be
    /// be present in this list as per the specification, `Schema`s which are
    /// identified are to be treated as root schemas.
    pub subschemas: Cow<'i, [K]>,

    /// Anchors within this `Schema`
    pub anchors: Cow<'i, [Anchor]>,

    /// Dependents of this `Schema`.
    pub dependents: Cow<'i, [K]>,

    ///  Dependencies of this `Schema`.
    pub references: Cow<'i, [Reference<K>]>,

    /// Compiled [`Keyword`]s.
    pub keywords: Cow<'i, [L::Keyword]>,

    /// The schema's source [`Value`], [`AbsoluteUri`], and path as a JSON
    /// [`Pointer`]
    pub source: Source<'i>,
}

impl<L, K> PartialEq<Schema<'_, L, K>> for Value
where
    L: Language<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Schema<'_, L, K>) -> bool {
        self == other.value()
    }
}
impl<L, K> PartialEq<Value> for Schema<'_, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Value) -> bool {
        self.value() == other
    }
}

impl<L, K> Serialize for Schema<'_, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.source.serialize(serializer)
    }
}

impl<'i, L, K> Schema<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Clones the `Schema`
    #[must_use]
    pub fn into_owned(self) -> Schema<'static, L, K> {
        Schema {
            key: self.key,
            parent: self.parent,
            id: self.id.map(|id| Cow::Owned(id.into_owned())),
            path: Cow::Owned(self.path.into_owned()),
            uris: Cow::Owned(self.uris.into_owned()),
            metaschema: Cow::Owned(self.metaschema.into_owned()),
            source: self.source.to_owned(),
            anchors: Cow::Owned(self.anchors.into_owned()),
            dependents: Cow::Owned(self.dependents.into_owned()),
            references: Cow::Owned(self.references.into_owned()),
            keywords: Cow::Owned(self.keywords.into_owned()),
            subschemas: Cow::Owned(self.subschemas.into_owned()),
        }
    }

    /// Returns most relevant URI for the schema, either using the `$id` or the
    /// most relevant as determined by the schema's ancestory or source.
    #[must_use]
    pub fn absolute_uri(&self) -> &AbsoluteUri {
        self.id.as_deref().unwrap_or(&self.uris[0])
    }
}
impl<L, K> std::ops::Index<&str> for Schema<'_, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.source[index]
    }
}

impl<'i, L, K> Schema<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// [`Value`] of the schema
    #[must_use]
    pub fn value(&self) -> &Value {
        &self.source
    }
}

impl<'i, L, K> Deref for Schema<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<'i, L, K> PartialEq for Schema<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema
    }
}
impl<'i, L, K> Eq for Schema<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Store                                 ║
║                                 ¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone)]
struct Store<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    table: SlotMap<K, CompiledSchema<L, K>>,
    index: HashMap<AbsoluteUri, K>,
}
impl<L, K> Default for Store<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn default() -> Self {
        Self {
            table: Default::default(),
            index: Default::default(),
        }
    }
}
#[allow(clippy::unnecessary_box_returns)]
impl<L, K> Store<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn get_mut(&mut self, key: K) -> Option<&mut CompiledSchema<L, K>> {
        self.table.get_mut(key)
    }
    fn get(&self, key: K) -> Option<&CompiledSchema<L, K>> {
        self.table.get(key)
    }
    pub(crate) fn get_index(&self, id: &AbsoluteUri) -> Option<K> {
        self.index.get(id).copied()
    }
    // pub(crate) fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
    //     self.index.entry(id)
    // }
    pub(crate) fn contains_key(&self, key: K) -> bool {
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
    pub(crate) fn insert(&mut self, schema: CompiledSchema<L, K>) -> Result<K, error::SourceError> {
        let id = schema.id.as_ref().unwrap_or(&schema.uris[0]);
        if let Some(key) = self.index.get(id) {
            let existing = self.table.get(*key).unwrap();
            if existing != &schema {
                return Err(error::SourceError::SchemaConflict {
                    uri: id.clone(),
                    existing_path: existing.path.clone(),
                    new_path: schema.path.clone(),
                    backtrace: Backtrace::capture(),
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Schemas                               ║
║                                 ¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Clone, Debug, Default)]
pub struct Schemas<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    store: Store<L, K>,
    sandbox: Option<Store<L, K>>,
}

impl<L, K> Schemas<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Store::default(),
            sandbox: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn evaluate<'i, 'v>(
        &self,
        eval: Evaluate<'i, 'v, L, K>,
    ) -> Result<<L as LanguageReport<'v, L, K>, EvaluateError<K>> {
        let Evaluate {
            key,
            sources,
            dialects,
            output,
            global_numbers,
            eval_numbers,
            value,
            language,
            instance_location,
            keyword_location,
        } = eval;

        let schema = self.get(key, sources)?;

        let mut report = <<L as Language<K>>::Report<'v> as Report<'v>>::new(output, &schema);

        let mut ctx = language.new_context(NewContext {
            global_numbers,
            eval_numbers,
            sources,
            report: &mut report,
            schemas: self,
            dialects,
            instance_location,
            keyword_location,
        });

        for keyword in &*schema.keywords {
            keyword.evaluate(&mut ctx, value)?;
        }
        // required otherwise borrowck sees `ctx` as still being potentially
        // borrowed
        drop(ctx);
        Ok(report)
    }
    pub(crate) fn is_compiled_by_uri(&self, uri: &AbsoluteUri) -> bool {
        let Some(key) = self.store().get_index(uri) else {
            return false;
        };
        self.is_compiled(key)
    }
    pub(crate) fn is_compiled(&self, key: K) -> bool {
        let Some(s) = self.store().get(key) else {
            return false;
        };
        s.compiled
    }
    pub(crate) fn set_compiled(&mut self, key: K) {
        self.sandbox().table.get_mut(key).unwrap().compiled = true;
    }

    pub(crate) fn ensure_not_cyclic(
        &mut self,
        key: K,
        uri: &AbsoluteUri,
        references: &[Reference<K>],
        sources: &Sources,
    ) -> Result<(), error::CompileError<L, K>> {
        for reference in references {
            if key == reference.key
                || self
                    .transitive_dependencies(reference.key, sources)
                    .any(|schema| schema.key == key)
            {
                return Err(CompileError::CyclicGraph {
                    from: uri.clone(),
                    to: reference.absolute_uri.clone(),
                    backtrace: Backtrace::capture(),
                }
                .into());
            }
        }
        Ok(())
    }

    pub(crate) fn remove(&mut self, key: K) {
        let uri = self.get_uri(key).cloned();
        self.sandbox().table.remove(key);
        if let Some(uri) = uri {
            self.sandbox().index.remove(&uri);
        }
    }

    pub(crate) fn has_keywords(&self, key: K) -> bool {
        !self.store().get(key).unwrap().keywords.is_empty()
    }
    pub(crate) fn set_keywords(&mut self, key: K, keywords: Box<[L::Keyword]>) {
        self.sandbox().table.get_mut(key).unwrap().keywords = keywords;
    }

    pub(crate) fn has_keywords_by_uri(&self, uri: &AbsoluteUri) -> bool {
        self.get_key(uri)
            .map_or(false, |key| self.has_keywords(key))
    }
    pub(crate) fn get_uri(&mut self, key: K) -> Option<&AbsoluteUri> {
        self.store()
            .index
            .iter()
            .find(|(_, v)| **v == key)
            .map(|(k, _)| k)
    }

    fn sandbox(&mut self) -> &mut Store<L, K> {
        self.sandbox
            .as_mut()
            .expect("transaction failed: schema sandbox not found.\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new")
    }

    fn store(&self) -> &Store<L, K> {
        if let Some(sandbox) = self.sandbox.as_ref() {
            return sandbox;
        }
        &self.store
    }
    pub fn get_key(&self, uri: &AbsoluteUri) -> Option<K> {
        self.store().get_index(uri)
    }
    // pub(crate) fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
    //     self.sandbox().index_entry(id)
    // }
    pub(crate) fn insert(&mut self, schema: CompiledSchema<L, K>) -> Result<K, error::SourceError> {
        self.sandbox().insert(schema)
    }

    // pub(crate) fn compiled_iter(&self) -> slotmap::basic::Iter<'_, Key, CompiledSchema> {
    //     self.store().iter()
    // }

    pub(crate) fn ancestors<'i>(&'i self, key: K, sources: &'i Sources) -> Ancestors<'i, L, K> {
        Ancestors::new(key, self, sources)
    }

    pub(crate) fn descendants<'i>(&'i self, key: K, sources: &'i Sources) -> Descendants<'i, L, K> {
        Descendants::new(key, self, sources)
    }

    pub(crate) fn ensure_key_exists<T, F>(&self, key: K, f: F) -> Result<T, UnknownKeyError<K>>
    where
        F: FnOnce() -> T,
    {
        if self.store().contains_key(key) {
            Ok(f())
        } else {
            Err(UnknownKeyError {
                key,
                backtrace: Backtrace::capture(),
            })
        }
    }

    pub(crate) fn direct_dependents<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> DirectDependents<'i, L, K> {
        DirectDependents::new(key, self, sources)
    }

    pub(crate) fn all_dependents<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> AllDependents<'i, L, K> {
        AllDependents::new(key, self, sources)
    }

    pub(crate) fn transitive_dependencies<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> TransitiveDependencies<'i, L, K> {
        TransitiveDependencies::new(key, self, sources)
    }

    pub(crate) fn direct_dependencies<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> DirectDependencies<'i, L, K> {
        DirectDependencies::new(key, self, sources)
    }

    pub(crate) fn get_unchecked<'i>(&'i self, key: K, sources: &'i Sources) -> Schema<'i, L, K> {
        self.get(key, sources).unwrap()
    }

    pub(crate) fn get_compiled(&self, key: K) -> Option<CompiledSchema<L, K>> {
        self.store().get(key).cloned()
    }

    /// Returns the [`Schema`] with the given `Key` if it exists.
    pub(crate) fn get<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> Result<Schema<'i, L, K>, UnknownKeyError<K>> {
        let schema = self.store().get(key).ok_or(UnknownKeyError {
            key,
            backtrace: Backtrace::capture(),
        })?;

        Ok(Schema {
            key,
            id: schema.id.as_ref().map(Cow::Borrowed),
            path: Cow::Borrowed(&schema.path),
            metaschema: Cow::Borrowed(&schema.metaschema),
            source: Source::new(schema.absolute_uri(), &schema.link, sources),
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
    pub(crate) fn get_mut(&mut self, key: K) -> Option<&mut CompiledSchema<L, K>> {
        self.sandbox().get_mut(key)
    }

    pub(crate) fn add_reference(
        &mut self,
        key: K,
        ref_: Reference<K>,
        sources: &Sources,
    ) -> Result<(), CompileError<L, K>> {
        let references = self.get_compiled(ref_.key).unwrap().references.clone();
        self.ensure_not_cyclic(key, &ref_.absolute_uri, &references, sources)?;
        self.get_mut(key).unwrap().references.push(ref_);
        Ok(())
    }
    ///
    pub(crate) fn add_dependent(&mut self, referenced: K, referrer: K) {
        self.get_mut(referenced).unwrap().dependents.push(referrer);
    }

    #[must_use]
    pub(crate) fn get_by_uri<'i>(
        &'i self,
        uri: &AbsoluteUri,
        sources: &'i Sources,
    ) -> Option<Schema<'i, L, K>> {
        let key = self.store().index.get(uri).copied()?;
        Some(self.get_unchecked(key, sources))
    }
    // #[must_use]
    // pub(crate) fn get_compiled_by_uri(&self, uri: &AbsoluteUri) -> Option<CompiledSchema> {
    //     let key = self.store().index.get(uri).copied()?;
    //     Some(self.get_compiled_unchecked(key))
    // }

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

    pub(crate) fn contains_key(&self, key: K) -> bool {
        self.store().contains_key(key)
    }

    pub(crate) fn contains_uri(&self, uri: &AbsoluteUri) -> bool {
        self.store().contains_uri(uri)
    }
}

/// A reference to a schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference<Key> {
    /// Key to the referenced [`Schema`]
    pub key: Key,
    /// The referenced URI
    pub uri: Uri,
    /// The resolved Absolute URI
    pub absolute_uri: AbsoluteUri,
    /// The keyword of the reference (e.g. $ref, $dynamicRef, $recursiveRef, etc)
    pub keyword: &'static str,
}

/// An [`Iterator`] over [`Schema`]s from an `Iterator` of [`Key`]s.
///
/// Each [`Item`](Iterator::Item) is a `Result<Schema, UnknownKeyError>`, to
/// safeguard against the circumstance of a [`Key`] belonging to a different
/// [`Interrogator`](`crate::Interrogator`). If this is not a concern, use
/// [`unchecked`](`Iter::unchecked`) which unwraps all `Result`s.
///
pub struct Iter<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    sources: &'i Sources,
    schemas: &'i Schemas<L, K>,
    inner: Either<std::slice::Iter<'i, K>, std::vec::IntoIter<K>>,
}

impl<'i, L, K> Iter<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub(crate) fn new(keys: &'i [K], schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            sources,
            schemas,
            inner: Either::Left(keys.iter()),
        }
    }
    /// Converts this `Iter` into an `IterUnchecked`, thus unwrapping all
    /// `Result`s.
    ///
    /// # Safety
    /// Do not use this unless you are certain all `Key`s are associated with
    /// the [`Interrogator`] from which this is originated.
    #[must_use]
    pub fn unchecked(self) -> IterUnchecked<'i, L, K> {
        IterUnchecked { inner: self }
    }

    // pub(crate) fn from_vec(keys: Vec<Key>, schemas: &'i Schemas, sources: &'i Sources) -> Self {
    //     Self {
    //         sources,
    //         schemas,
    //         inner: Either::Right(keys.into_iter()),
    //     }
    // }
}
impl<'i, L, K> Iterator for Iter<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    type Item = Result<Schema<'i, L, K>, UnknownKeyError<K>>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = match self.inner.as_mut() {
            Either::Left(iter) => *iter.next()?,
            Either::Right(iter) => iter.next()?,
        };
        Some(self.schemas.get(key, self.sources))
    }
}
/// An unchecked [`Iterator`] over [`Schema`]s from an `Iterator` of [`Key`]s.
///
/// # Panics
/// This will panic if any of the [`Key`]s are not associated with the same
/// [`Interrogator`](`crate::Interrogator`).
pub struct IterUnchecked<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    inner: Iter<'i, L, K>,
}

impl<'i, L, K> Iterator for IterUnchecked<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    type Item = Schema<'i, L, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(std::result::Result::unwrap)
    }
}

#[cfg(test)]
mod tests {}