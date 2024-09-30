use core::fmt;
use std::{borrow::Cow, collections::HashMap, fmt::Debug, iter::once, sync::Arc};

use grill_uri::{AbsoluteUri, Uri};
use jsonptr::PointerBuf;
use serde_json::Value;
use slotmap::{new_key_type, Key};

use crate::spec::{
    keyword::{Found, Keyword},
    Specification,
};

new_key_type! {
    pub struct DialectKey;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Dialect                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone)]
pub struct Dialect<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    /// The key of the dialect.
    ///
    /// When initializing, set to `DialectKey::default()`
    pub key: DialectKey,
    /// Primary [`AbsoluteUri`] of the dialect (e.g.
    /// `"https://json-schema.org/draft/2020-12/schema"`)
    pub uri: AbsoluteUri,
    /// Secondary [`AbsoluteUri`]s of the dialect (e.g. http://json-schema.org/draft-07/schema)
    pub additional_uris: Vec<AbsoluteUri>,
    /// All possible keywords of this `Dialect`
    pub keywords: Vec<S::Keyword>,
    /// Metaschema sources of this `Dialect`
    pub sources: HashMap<AbsoluteUri, Arc<Value>>,
    /// Identifies a schema
    pub identifier_field: Cow<'static, str>,
}

impl<S, K> PartialEq for Dialect<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri && self.keywords == other.keywords && self.sources == other.sources
    }
}
impl<S, K> Dialect<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    pub fn identify(&self, value: &Value) -> Result<Option<Uri>, grill_uri::Error> {
        value
            .get(self.identifier_field.as_ref())
            .and_then(Value::as_str)
            .map(|s| Uri::parse(s))
            .transpose()
    }

    pub fn is_relevant_to(&self, value: &Value) -> bool {
        let Some(dolla_dolla_schema_yall) = value.get("$schema") else {
            return false;
        };
        let Value::String(uri) = dolla_dolla_schema_yall else {
            return false;
        };
        self.contains_uri(uri)
    }
    pub fn uri(&self) -> &AbsoluteUri {
        &self.uri
    }
    pub fn contains_uri(&self, uri: &str) -> bool {
        if self.uri == uri {
            return true;
        }
        self.additional_uris.iter().any(|u| u == uri)
    }
    pub fn keywords(&self) -> &[S::Keyword] {
        &self.keywords
    }
    pub fn sources(&self) -> &HashMap<AbsoluteUri, Arc<Value>> {
        &self.sources
    }
    pub fn references(&self, value: &Value) -> Vec<Found<String>> {
        self.keywords
            .iter()
            .filter_map(|keyword| keyword.reference(value))
            .collect()
    }
    pub fn anchors(&self, value: &Value) -> Vec<Found<String>> {
        self.keywords
            .iter()
            .filter_map(|keyword| keyword.anchor(value))
            .collect()
    }

    pub fn embedded_schemas(&self, value: &Value) -> Vec<Found<PointerBuf>>
    where
        K: 'static + Key + Send + Sync,
        S: 'static + Specification<K>,
        K: 'static + Key + Send + Sync,
    {
        self.keywords
            .iter()
            .flat_map(|keyword| keyword.embedded_schemas(value))
            .collect()
    }
    pub fn all_uris(&self) -> impl Iterator<Item = &AbsoluteUri> {
        once(&self.uri).chain(self.additional_uris.iter())
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Dialects                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// An insert-only collection of [`Dialect`]s.
#[derive(Clone, Debug)]
pub struct Dialects<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    table: slotmap::SlotMap<DialectKey, Dialect<S, K>>,
    primary_key: DialectKey,
    uris: HashMap<AbsoluteUri, DialectKey>,
}

impl<S, K> Dialects<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    pub fn new(default: Dialect<S, K>) -> Self {
        let mut table = slotmap::SlotMap::with_key();
        let primary_key = table.insert(default);
        let mut uris = HashMap::new();
        uris.insert(table[primary_key].uri.clone(), primary_key);
        Self {
            table,
            primary_key,
            uris,
        }
    }
    /// Returns the `DialectKey` of the `Dialect` with the given `AbsoluteUri`,
    /// if it exists.
    pub fn get_key(&self, uri: &AbsoluteUri) -> Option<DialectKey> {
        self.uris.get(uri).copied()
    }

    pub fn contains(&self, uri: &AbsoluteUri) -> bool {
        self.uris.contains_key(uri)
    }

    /// Returns the `Dialect` with the given key.
    pub fn get(&self, key: DialectKey) -> &Dialect<S, K> {
        self.table.get(key).unwrap()
    }

    pub fn default_dialect_key(&self) -> DialectKey {
        self.primary_key
    }

    pub fn primary_dialect(&self) -> &Dialect<S, K> {
        self.get(self.primary_key)
    }

    pub fn find_dialect(&self, value: &Value) -> Option<&Dialect<S, K>> {
        self.find_dialect_key(value).map(|key| &self.table[key])
    }
    pub fn find_dialect_key(&self, value: &Value) -> Option<DialectKey> {
        let schema = value.get("$schema")?;
        let uri = schema.as_str()?;
        self.uris.get(uri).copied()
    }

    /// Inserts a new `Dialect` into the `Dialects` and returns its key.
    ///
    /// # Errors
    /// This method returns the `dialect` attempting to be inserted if a
    /// different `Dialect` is associated with the same `AbsoluteUri`.
    pub fn insert(
        &mut self,
        dialect: Dialect<S, K>,
    ) -> Result<DialectKey, DuplicateDialectError<S, K>> {
        // ensuring a unique uris - can't use Dialect::all_uris here as it will
        // borrow `dialect` and force a clone on the err branch
        for uri in once(&dialect.uri).chain(dialect.additional_uris.iter()) {
            if let Some(&existing) = self.uris.get(uri) {
                return if self.table[existing] == dialect {
                    // found, returning existing
                    Ok(existing)
                } else {
                    Err(DuplicateDialectError { dialect })
                };
            }
        }
        // inserting dialect
        let key = self.table.insert(dialect);
        self.table[key].key = key;

        // associating all uris
        for uri in self.table[key].all_uris() {
            self.uris.insert(uri.clone(), key);
        }
        Ok(key)
    }

    /// Sets the primary dialect key.
    pub fn set_primary_key(&mut self, key: DialectKey) {
        self.primary_key = key;
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                            DuplicateDialectError                             ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                            ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug)]
pub struct DuplicateDialectError<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    dialect: Dialect<S, K>,
}

impl<S, K> fmt::Display for DuplicateDialectError<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dialect with uri \"{}\" already exists",
            self.dialect.uri()
        )
    }
}

impl<S, K> DuplicateDialectError<S, K>
where
    S: Specification<K> + Send + Sync,
    K: 'static + Key + Clone + Send + Sync,
{
    pub fn dialect(&self) -> &Dialect<S, K> {
        &self.dialect
    }
    pub fn take_dialect(self) -> Dialect<S, K> {
        self.dialect
    }
}
