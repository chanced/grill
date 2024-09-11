use core::fmt;
use std::{borrow::Cow, collections::HashMap, fmt::Debug, sync::Arc};

use grill_uri::AbsoluteUri;
use serde_json::Value;
use slotmap::{new_key_type, Key};

use crate::spec::{found, Keyword, Specification};

new_key_type! {
    pub struct DialectKey;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   identify                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Modern schema identification (draft 06 and later), identifies a schema by
/// the `$id` keyword.
pub fn identify(value: &Value) -> Option<Cow<str>> {
    value.get("$id").and_then(Value::as_str).map(Cow::Borrowed)
}

/// Legacy schema identification (draft 04 and earlier), identifies a schema by
/// the `id` keyword.
pub fn identify_legacy(value: &Value) -> Option<Cow<str>> {
    value.get("id").and_then(Value::as_str).map(Cow::Borrowed)
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
    /// Primary [`AbsoluteUri`] of the dialect (e.g.
    /// `"https://json-schema.org/draft/2020-12/schema"`)
    pub uri: AbsoluteUri,
    /// All possible keywords of this `Dialect`
    pub keywords: Vec<S::Keyword>,
    /// Metaschema sources of this `Dialect`
    pub sources: HashMap<AbsoluteUri, Arc<Value>>,
    /// Identifies a schema
    pub identify: fn(&Value) -> Option<Cow<str>>,
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
    pub fn identify<'v>(&self, value: &'v Value) -> Option<Cow<'v, str>> {
        (self.identify)(value)
    }

    pub fn uri(&self) -> &AbsoluteUri {
        &self.uri
    }
    pub fn keywords(&self) -> &[S::Keyword] {
        &self.keywords
    }
    pub fn sources(&self) -> &HashMap<AbsoluteUri, Arc<Value>> {
        &self.sources
    }

    pub fn references(&self, value: &Value) -> Vec<found::Reference> {
        self.keywords
            .iter()
            .filter_map(|keyword| keyword.reference(value))
            .collect()
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
    pub fn get(&self, key: DialectKey) -> Option<&Dialect<S, K>> {
        self.table.get(key)
    }

    /// Inserts a new `Dialect` into the `Dialects` and returns its key.
    ///
    /// # Errors
    /// This method returns the `dialect` attempted if a different `Dialect` is associated
    /// with the same `AbsoluteUri`.
    #[allow(clippy::missing_panics_doc)]
    pub fn insert(
        &mut self,
        dialect: Dialect<S, K>,
    ) -> Result<DialectKey, DuplicateDialectError<S, K>> {
        if let Some(&existing) = self.uris.get(&dialect.uri) {
            return if self.table[existing] == dialect {
                Ok(existing)
            } else {
                Err(DuplicateDialectError { dialect })
            };
        }
        let without_frag = dialect
            .uri
            .fragment()
            .is_some_and(str::is_empty)
            .then(|| dialect.uri.with_fragment(None).unwrap());
        let with_empty_frag = dialect
            .uri
            .fragment()
            .is_none()
            .then(|| dialect.uri.clone());
        let key = self.table.insert(dialect);
        self.uris.insert(self.table[key].uri.clone(), key);
        if let Some(uri) = without_frag {
            self.uris.insert(uri, key);
        } else if let Some(uri) = with_empty_frag {
            self.uris.insert(uri, key);
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
