use grill_uri::AbsoluteUri;
use slotmap::{Key, SlotMap};
use snafu::{ensure, Snafu};
use std::collections::HashMap;

use crate::source::Sources;

/// A trait which indicates that a schema is capable of being embedded in
/// another schema.
pub trait EmbeddedIn<K> {
    /// Returns the key of the schema that this schema is embedded in, if any.
    fn embedded_in_key(&self) -> Option<K>;
}

/// A trait which indicates that a schema is capable of having subschemas.
pub trait Embedded<K> {
    /// Returns a slice of subschema keys for this schema.
    fn embedded_keys(&self) -> &[K];
}

/// A trait satisfied by a type that represents a borrowed (but ownable) schema.
///
/// This trait is satisfied by [`Language`](crate::lang::Language) implementations. See your desired language's documentation for more information.
pub trait Schema<'i, K> {
    /// Returns the key of the schema.
    fn key(&self) -> K;
}

/// A trait satisfied by a type that represents a compiled schema.
///
/// This trait is satisfied by [`Language`](crate::lang::Language) implementations. See
/// your desired language's documentation for more information.
pub trait CompiledSchema<K>: PartialEq {
    /// The borrowed schema representation.
    type Schema<'i>: Schema<'i, K>;

    /// Sets the key of the schema.
    fn set_key(&mut self, key: K);

    /// Returns the borrowed [`Self::Schema`] representation.
    fn as_schema<'i>(&self, sources: &Sources) -> Self::Schema<'i>;
}

/// A collection of schemas indexed by [`AbsoluteUri`]s.
pub struct Schemas<C, K: Key> {
    schemas: SlotMap<K, C>,
    refs: HashMap<AbsoluteUri, K>,
}

impl<C, K> Schemas<C, K>
where
    C: CompiledSchema<K>,
    K: Key,
{
    /// Inserts `schema` into the collection and returns its key.
    pub fn insert(&mut self, schema: C) -> K {
        self.schemas.insert(schema)
    }

    /// Assigns a schema to a URI
    ///
    /// # Errors
    /// Returns [`DuplicateLinkError`] if a schema is already linked to the given `uri`.
    pub fn assign(&mut self, uri: AbsoluteUri, key: K) -> Result<K, DuplicateLinkError<K>> {
        match self.refs.get(&uri).copied() {
            Some(existing) => ensure!(existing == key, DuplicateLinkSnafu { existing, uri }),
            None => self.insert_ref(uri, key),
        }
        Ok(key)
    }
    fn insert_ref(&mut self, uri: AbsoluteUri, key: K) {
        self.refs.insert(uri, key);
    }
    /// Returns [`C::Schema`](CompiledSchema::Schema) for the supplied
    /// [`AbsoluteUri`], if it exists.
    pub fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&C> {
        self.refs.get(uri).copied().and_then(|k| self.get_by_key(k))
    }

    pub fn get_by_key(&self, key: K) -> Option<&C> {
        self.schemas.get(key)
    }

    /// Returns a mutable reference to the schema ([`C`](`CompiledSchema`)) with
    /// the given key.
    pub fn get_mut(&mut self, key: K) -> Option<&mut C> {
        self.schemas.get_mut(key)
    }
}

/// A duplicate [`CompiledSchema`] already exists at the given `uri`.
#[derive(Debug, Snafu)]
pub struct DuplicateLinkError<K> {
    /// The URI that the schema is already linked to.
    pub uri: AbsoluteUri,
    /// The key of the existing schema.
    pub existing: K,
}
