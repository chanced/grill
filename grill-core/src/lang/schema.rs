//! Schema definitions and data structures.

use grill_uri::AbsoluteUri;
use slotmap::{new_key_type, Key, SlotMap};
use snafu::{ensure, Snafu};
use std::collections::HashMap;

use super::source::Sources;

new_key_type! {
    /// Default key type used as a unique identifier for a schema.
    pub struct DefaultKey;
}

/// A trait which indicates that a schema is capable of being embedded in
/// another schema.
pub trait EmbeddedIn<K> {
    /// Returns the key of the schema that this schema is embedded in, if any.
    fn embedded_in(&self) -> Option<K>;
}

/// A trait which indicates that a schema is capable of having subschemas.
pub trait Embedded<K> {
    /// Returns a slice of subschema keys for this schema.
    fn embedded(&self) -> &[K];
}

/// A trait satisfied by a type that represents a borrowed (but ownable) schema.
///
/// This trait is satisfied by [`Language`](crate::lang::Language)
/// implementations. See your desired language's documentation for more
/// information.
pub trait Schema<'i, K> {
    /// Returns the key of the schema.
    fn key(&self) -> K;
}

/// A trait satisfied by a type that represents a compiled schema.
///
/// This trait is satisfied by [`Language`](crate::lang::Language)
/// implementations. See your desired language's documentation for more
/// information.
pub trait CompiledSchema<K>: Clone + PartialEq {
    /// The borrowed schema representation.
    type Schema<'i>: Schema<'i, K>;

    /// Sets the key of the schema.
    fn set_key(&mut self, key: K);

    /// Returns the borrowed [`Self::Schema`] representation.
    fn as_schema<'i>(&self, sources: &Sources) -> Self::Schema<'i>;
}

/// A collection of schemas indexed by [`AbsoluteUri`]s.
#[derive(Debug, Clone)]
pub struct Schemas<S, K: Key> {
    schemas: SlotMap<K, S>,
    keys: HashMap<AbsoluteUri, K>,
}
impl<S, K: Key> Default for Schemas<S, K> {
    fn default() -> Self {
        Self::new()
    }
}
impl<S, K: Key> Schemas<S, K> {
    /// Creates a new schema graph.
    pub fn new() -> Self {
        Self {
            schemas: SlotMap::with_key(),
            keys: HashMap::new(),
        }
    }
}

impl<S, K> Schemas<S, K>
where
    S: CompiledSchema<K>,
    K: Key,
{
    /// Inserts `schema` into the graph and returns its key.
    pub fn insert(&mut self, schema: S) -> K {
        let key = self.schemas.insert(schema);
        self.schemas.get_mut(key).unwrap().set_key(key);
        key
    }

    /// Assigns an `AbsoluteUri` to a schema key.
    ///
    /// # Errors
    /// Returns [`DuplicateLinkError`] if a schema is already linked to the
    /// given `uri`.
    pub fn assign(&mut self, uri: AbsoluteUri, key: K) -> Result<(), DuplicateLinkError<K>> {
        match self.keys.get(&uri).copied() {
            Some(existing) => ensure!(existing == key, DuplicateLinkSnafu { existing, uri }),
            None => self.insert_uri(uri, key),
        }
        Ok(())
    }

    fn insert_uri(&mut self, uri: AbsoluteUri, key: K) {
        self.keys.insert(uri, key);
    }
    /// Returns [`Self::C::Schema`](CompiledSchema::Schema) for the supplied
    /// [`AbsoluteUri`], if it exists.
    pub fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&S> {
        self.keys.get(uri).copied().and_then(|k| self.get_by_key(k))
    }
    /// Returns a reference to compiled schema ([`Self::C`](`CompiledSchema`))
    /// with the supplied `key` (``)
    pub fn get_by_key(&self, key: K) -> Option<&S> {
        self.schemas.get(key)
    }

    /// Returns a mutable reference to the schema ([`C`](`CompiledSchema`)) with
    /// the given key.
    pub fn get_mut(&mut self, key: K) -> Option<&mut S> {
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

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use slotmap::DefaultKey;

    use super::*;

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    struct Compiled {
        key: DefaultKey,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct TestSchema<'i> {
        key: DefaultKey,
        _marker: PhantomData<&'i ()>,
    }
    impl<'i> Schema<'i, DefaultKey> for TestSchema<'i> {
        fn key(&self) -> DefaultKey {
            self.key
        }
    }

    impl CompiledSchema<DefaultKey> for Compiled {
        type Schema<'i> = TestSchema<'i>;

        fn set_key(&mut self, key: DefaultKey) {
            self.key = key;
        }

        fn as_schema<'i>(&self, _sources: &Sources) -> Self::Schema<'i> {
            TestSchema {
                key: self.key,
                _marker: PhantomData,
            }
        }
    }

    #[test]
    fn test_insert_ref() {
        let mut schemas: Schemas<Compiled, DefaultKey> = Schemas::new();
        let key = schemas.insert(Compiled::default());

        assert_ne!(key, DefaultKey::default());
        let uri = AbsoluteUri::parse("https://example.com/schema.json").unwrap();
        schemas.assign(uri.clone(), key).unwrap();
        assert_eq!(schemas.get_by_uri(&uri).unwrap().key, key);
    }
}
