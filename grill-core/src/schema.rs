//! Resources for working with schemas.

pub mod iter;

pub mod traverse;

pub mod dialect;

use crate::criterion::{Assessment, Criterion, CriterionReport, NewContext, Report};
use crate::error::{self, CompileError};
use crate::{cache, Key};

pub use dialect::{Dialect, Dialects};
use either::Either;
use serde::{Serialize, Serializer};
use snafu::Backtrace;

use crate::uri::Uri;
pub(crate) mod compiler;

use crate::{
    criterion::Keyword,
    error::{EvaluateError, UnknownKeyError},
    schema::traverse::{
        AllDependents, Ancestors, Descendants, DirectDependencies, DirectDependents,
        TransitiveDependencies,
    },
    source::{Link, Source, Sources},
    uri::AbsoluteUri,
};

use jsonptr::Pointer;
use serde_json::Value;
use slotmap::{new_key_type, SlotMap};
use std::{borrow::Cow, collections::HashMap, hash::Hash, ops::Deref};

pub struct Evaluate<'i, 'v, C, K>
where
    K: 'static + Key,
    C: Criterion<K>,
{
    pub output: <<C as Criterion<K>>::Report<'v> as Report<'v>>::Output,
    pub key: K,
    pub value: &'v Value,
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub sources: &'i Sources,
    pub global_numbers: &'i cache::Numbers,
    pub eval_numbers: &'i mut cache::Numbers,
    pub criterion: &'i C,
}

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
pub(crate) struct CompiledSchema<C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    /// Abs URI of the schema.
    pub(crate) id: Option<AbsoluteUri>,

    /// Path to the schema from the root schema as a JSON Pointer
    pub(crate) path: Pointer,

    /// The [`Key`] of the schema which contains this schema, if any.
    ///
    /// Note that if this schema has an `id`, `parent` will be `None` regardless of
    /// whether or not this schema is embedded.
    pub(crate) parent: Option<K>,

    /// Directly embedded subschemas, excluding those with `id`s.
    pub(crate) subschemas: Vec<K>,

    /// Dependents of this `Schema`.
    pub(crate) dependents: Vec<K>,

    ///  Referenced dependencies of this `Schema`.
    pub(crate) references: Vec<Reference<K>>,

    /// All anchors defined in this schema and embedded schemas which do not
    /// have `id`s.
    pub(crate) anchors: Vec<Anchor>,

    /// All URIs which this schema is referenced by.
    pub(crate) uris: Vec<AbsoluteUri>,

    /// Abs URI of the schema's `Metaschema`.
    pub(crate) metaschema: AbsoluteUri,

    // Compiled keywords.
    pub(crate) keywords: Box<[C::Keyword]>,

    /// Absolute URI of the source and path to this schema.
    pub(crate) link: Link,

    pub(crate) compiled: bool,
}

impl<C, K> CompiledSchema<C, K>
where
    C: Criterion<K>,
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
    ) -> CompiledSchema<C, K> {
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
impl<C, K> CompiledSchema<C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    /// Returns most relevant URI for the schema, either using the `$id` or the
    /// most relevant as determined by the schema's ancestory or source.
    #[must_use]
    pub(crate) fn absolute_uri(&self) -> &AbsoluteUri {
        self.id.as_ref().unwrap_or(&self.uris[0])
    }
}

impl<C, K> PartialEq for CompiledSchema<C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema && self.link == other.link
    }
}

impl<C: Criterion<K>, K: 'static + Key> Eq for CompiledSchema<C, K> {}

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
pub struct Schema<'i, C: Criterion<K>, K: 'static + Key> {
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
    pub keywords: Cow<'i, [C::Keyword]>,

    /// The schema's source [`Value`], [`AbsoluteUri`], and path as a JSON
    /// [`Pointer`]
    pub source: Source<'i>,
}

impl<C, K> PartialEq<Schema<'_, C, K>> for Value
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Schema<'_, C, K>) -> bool {
        self == other.value()
    }
}
impl<C, K> PartialEq<Value> for Schema<'_, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Value) -> bool {
        self.value() == other
    }
}

impl<C, K> Serialize for Schema<'_, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.source.serialize(serializer)
    }
}

impl<'i, C: Criterion<K>, K: 'static + Key> Schema<'i, C, K> {
    /// Clones the `Schema`
    #[must_use]
    pub fn into_owned(self) -> Schema<'static, C, K> {
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
impl<C, K> std::ops::Index<&str> for Schema<'_, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    type Output = Value;

    fn index(&self, index: &str) -> &Self::Output {
        &self.source[index]
    }
}

impl<'i, C: Criterion<K>, K: 'static + Key> Schema<'i, C, K> {
    /// [`Value`] of the schema
    #[must_use]
    pub fn value(&self) -> &Value {
        &self.source
    }
}

impl<'i, C: Criterion<K>, K: 'static + Key> Deref for Schema<'i, C, K> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

impl<'i, C: Criterion<K>, K: 'static + Key> PartialEq for Schema<'i, C, K> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.metaschema == other.metaschema
    }
}
impl<'i, C: Criterion<K>, K: 'static + Key> Eq for Schema<'i, C, K> {}

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
struct Store<C: Criterion<K>, K: 'static + Key> {
    table: SlotMap<K, CompiledSchema<C, K>>,
    index: HashMap<AbsoluteUri, K>,
}
impl<C, K> Default for Store<C, K>
where
    C: Criterion<K>,
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
impl<C, K> Store<C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn get_mut(&mut self, key: K) -> Option<&mut CompiledSchema<C, K>> {
        self.table.get_mut(key)
    }
    fn get(&self, key: K) -> Option<&CompiledSchema<C, K>> {
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
    pub(crate) fn insert(&mut self, schema: CompiledSchema<C, K>) -> Result<K, error::SourceError> {
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
pub struct Schemas<C: Criterion<K>, K: 'static + Key> {
    store: Store<C, K>,
    sandbox: Option<Store<C, K>>,
}

impl<C, K> Schemas<C, K>
where
    C: Criterion<K>,
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
        eval: Evaluate<'i, 'v, C, K>,
    ) -> Result<CriterionReport<'v, C, K>, EvaluateError<K>> {
        let Evaluate {
            key,
            sources,
            keyword_location,
            instance_location,
            output,
            global_numbers,
            eval_numbers,
            value,
            criterion,
        } = eval;

        let schema = self.get(key, sources)?;

        let mut report = <<C as Criterion<K>>::Report<'v> as Report<'v>>::new(
            output,
            schema.absolute_uri(),
            keyword_location,
            instance_location,
            Assessment::Annotation(None),
        );

        for keyword in &*schema.keywords {
            let mut ctx = criterion.new_context(NewContext {
                global_numbers,
                eval_numbers,
                sources,
                report: &mut report,
                schemas: self,
            });
            if let Some(op) = keyword.evaluate(&mut ctx, value)? {
                report.push(op);
            }
        }
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
    ) -> Result<(), error::CompileError<C, K>> {
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
    pub(crate) fn set_keywords(&mut self, key: K, keywords: Box<[C::Keyword]>) {
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

    fn sandbox(&mut self) -> &mut Store<C, K> {
        self.sandbox
            .as_mut()
            .expect("transaction failed: schema sandbox not found.\n\nthis is a bug, please report it: https://github.com/chanced/grill/issues/new")
    }

    fn store(&self) -> &Store<C, K> {
        if let Some(sandbox) = self.sandbox.as_ref() {
            return sandbox;
        }
        &self.store
    }
    pub(crate) fn get_key(&self, uri: &AbsoluteUri) -> Option<K> {
        self.store().get_index(uri)
    }
    // pub(crate) fn index_entry(&mut self, id: AbsoluteUri) -> Entry<'_, AbsoluteUri, Key> {
    //     self.sandbox().index_entry(id)
    // }
    pub(crate) fn insert(&mut self, schema: CompiledSchema<C, K>) -> Result<K, error::SourceError> {
        self.sandbox().insert(schema)
    }

    // pub(crate) fn compiled_iter(&self) -> slotmap::basic::Iter<'_, Key, CompiledSchema> {
    //     self.store().iter()
    // }

    pub(crate) fn ancestors<'i>(&'i self, key: K, sources: &'i Sources) -> Ancestors<'i, C, K> {
        Ancestors::new(key, self, sources)
    }

    pub(crate) fn descendants<'i>(&'i self, key: K, sources: &'i Sources) -> Descendants<'i, C, K> {
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
    ) -> DirectDependents<'i, C, K> {
        DirectDependents::new(key, self, sources)
    }

    pub(crate) fn all_dependents<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> AllDependents<'i, C, K> {
        AllDependents::new(key, self, sources)
    }

    pub(crate) fn transitive_dependencies<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> TransitiveDependencies<'i, C, K> {
        TransitiveDependencies::new(key, self, sources)
    }

    pub(crate) fn direct_dependencies<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> DirectDependencies<'i, C, K> {
        DirectDependencies::new(key, self, sources)
    }

    pub(crate) fn get_unchecked<'i>(&'i self, key: K, sources: &'i Sources) -> Schema<'i, C, K> {
        self.get(key, sources).unwrap()
    }

    pub(crate) fn get_compiled(&self, key: K) -> Option<CompiledSchema<C, K>> {
        self.store().get(key).cloned()
    }

    /// Returns the [`Schema`] with the given `Key` if it exists.
    pub(crate) fn get<'i>(
        &'i self,
        key: K,
        sources: &'i Sources,
    ) -> Result<Schema<'i, C, K>, UnknownKeyError<K>> {
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
    pub(crate) fn get_mut(&mut self, key: K) -> Option<&mut CompiledSchema<C, K>> {
        self.sandbox().get_mut(key)
    }

    pub(crate) fn add_reference(
        &mut self,
        key: K,
        ref_: Reference<K>,
        sources: &Sources,
    ) -> Result<(), CompileError<C, K>> {
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
    ) -> Option<Schema<'i, C, K>> {
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
#[derive(Debug, Clone)]
pub struct Ref {
    /// the parsed [`Uri`] value.
    pub uri: Uri,
    /// the keyword of the reference (i.e. $ref)
    pub keyword: &'static str,
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
pub struct Iter<'i, C: Criterion<K>, K: 'static + Key> {
    sources: &'i Sources,
    schemas: &'i Schemas<C, K>,
    inner: Either<std::slice::Iter<'i, K>, std::vec::IntoIter<K>>,
}

impl<'i, C: Criterion<K>, K: 'static + Key> Iter<'i, C, K> {
    pub(crate) fn new(keys: &'i [K], schemas: &'i Schemas<C, K>, sources: &'i Sources) -> Self {
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
    pub fn unchecked(self) -> IterUnchecked<'i, C, K> {
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
impl<'i, C: Criterion<K>, K: 'static + Key> Iterator for Iter<'i, C, K> {
    type Item = Result<Schema<'i, C, K>, UnknownKeyError<K>>;

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
pub struct IterUnchecked<'i, C: Criterion<K>, K: 'static + Key> {
    inner: Iter<'i, C, K>,
}

impl<'i, C: Criterion<K>, K: 'static + Key> Iterator for IterUnchecked<'i, C, K> {
    type Item = Schema<'i, C, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(std::result::Result::unwrap)
    }
}

#[cfg(test)]
mod tests {}
