//! Schema definitions and data structures.

use core::fmt;
use grill_uri::AbsoluteUri;
use slotmap::{new_key_type, Key, SlotMap};
use std::collections::HashMap;

use crate::iter::{AllCompiledSchemas, AllSchemas, Iter};

use super::source::{Source, SourceKey, Sources};

new_key_type! {
    /// Default key type used as a unique identifier for a schema.
    pub struct DefaultKey;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  EmbeddedIn                                  ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait which indicates that a schema is capable of being embedded in
/// another schema.
pub trait EmbeddedIn<K> {
    /// Returns the key of the schema that this schema is embedded in, if any.
    fn embedded_in(&self) -> Option<K>;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 ReferencedBy                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait ReferencedBy<K> {
    type Ref;
    fn referenced_by(&self) -> &[Self::Ref];
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  Reference                                   ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait Reference<K> {
    fn reference(&self) -> K;
    fn referrer(&self) -> K;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                  References                                  ║
║                                 ¯¯¯¯¯¯¯¯¯¯¯¯                                 ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait References<K> {
    type Ref: Reference<K>;
    fn references(&self) -> &[Self::Ref];
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Embedded                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait which indicates that a schema is capable of having subschemas.
pub trait Embedded<K> {
    /// Returns a slice of subschema keys for this schema.
    fn embedded(&self) -> &[K];
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Schema                                    ║
║                                   ¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait satisfied by a type that represents a borrowed (but ownable) schema.
///
/// This trait is satisfied by [`Language`](crate::lang::Language)
/// implementations. See your desired language's documentation for more
/// information.
pub trait Schema<'int, K>: AsRef<K> {
    fn key(&self) -> K;
    fn source(&'int self) -> Source<'int>;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                CompiledSchema                                ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait satisfied by a type that represents a compiled schema.
///
/// This trait is satisfied by [`Language`](crate::lang::Language)
/// implementations. See your desired language's documentation for more
/// information.
pub trait CompiledSchema<K>: AsRef<K> + fmt::Debug + Clone + PartialEq + Send + Sync {
    /// The borrowed schema representation.
    type Schema<'int>: Schema<'int, K>
    where
        Self: 'int;

    fn source_key(&self) -> SourceKey;

    /// Returns the key of the schema.
    fn key(&self) -> K;

    /// Sets the key of the schema.
    fn set_key(&mut self, key: K);

    /// Returns the borrowed [`Self::Schema`] representation.
    fn to_schema<'int>(&'int self, sources: &'int Sources) -> Self::Schema<'int>;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Schemas                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A graph of schemas indexed by [`AbsoluteUri`]s.
#[derive(Debug, Clone)]
pub struct Schemas<S, K: Key> {
    pub(crate) map: SlotMap<K, S>,
    uris: HashMap<AbsoluteUri, K>,
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
            map: SlotMap::with_key(),
            uris: HashMap::new(),
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
        let key = self.map.insert(schema);
        self.map.get_mut(key).unwrap().set_key(key);
        key
    }

    /// Assigns an `AbsoluteUri` to a schema key.
    ///
    /// # Errors
    /// Returns [`DuplicateLinkError`] if a schema is already linked to the
    /// given `uri`.
    pub fn assign(&mut self, uri: AbsoluteUri, key: K) -> Result<(), DuplicateLinkError<K>> {
        match self.uris.get(&uri).copied() {
            Some(existing) => {
                if existing != key {
                    DuplicateLinkError::fail(uri, existing)?
                }
            }
            None => self.insert_uri(uri, key),
        }
        Ok(())
    }

    fn insert_uri(&mut self, uri: AbsoluteUri, key: K) {
        self.uris.insert(uri, key);
    }

    pub fn get_key_of(&self, uri: &AbsoluteUri) -> Option<K> {
        self.uris.get(uri).copied()
    }
    /// Returns [`Self::C::Schema`](CompiledSchema::Schema) for the supplied
    /// [`AbsoluteUri`], if it exists.
    pub fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&S> {
        self.uris.get(uri).copied().map(|k| self.get_compiled(k))
    }

    /// Returns a reference to compiled schema ([`Self::C`](`CompiledSchema`))
    /// with the supplied `key` or returns `InvalidKeyError` if the key does
    /// not exist.
    pub fn get_compiled(&self, key: K) -> &S {
        self.map.get(key).expect("invalid schema key")
    }

    /// Returns a mutable reference to the schema ([`C`](`CompiledSchema`)) with
    /// the given key.
    pub fn get_mut(&mut self, key: K) -> Option<&mut S> {
        self.map.get_mut(key)
    }

    pub fn all_compiled_schemas(&self) -> AllCompiledSchemas<'_, S, K> {
        AllCompiledSchemas::new(self)
    }
    pub fn all_schemas<'int>(&'int self, sources: &'int Sources) -> AllSchemas<'int, S, K> {
        self.all_compiled_schemas().into_all_schemas(sources)
    }

    pub fn iter<'int, I>(&'int self, sources: &'int Sources, keys: I) -> Iter<'int, I, S, K>
    where
        I: 'int + Iterator<Item = K>,
    {
        Iter::new(self, sources, keys)
    }
    pub fn keys(&self) -> impl Iterator<Item = K> + '_ {
        self.map.keys()
    }
    pub fn keys_and_compiled_schemas(&self) -> impl Iterator<Item = (K, &S)> + '_ {
        self.map.iter()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn contains_key(&self, key: K) -> bool {
        self.map.contains_key(key)
    }
    pub fn contains_uri(&self, uri: &AbsoluteUri) -> bool {
        self.uris.contains_key(uri)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                              DuplicateLinkError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A duplicate [`CompiledSchema`] already exists at the given `uri`.
#[derive(Debug, PartialEq)]
pub struct DuplicateLinkError<K> {
    /// The URI that the schema is already linked to.
    pub uri: AbsoluteUri,
    /// The key of the existing schema.
    pub existing: K,
}
impl<K> DuplicateLinkError<K> {
    /// Creates a new `DuplicateLinkError` with the given `uri` and `existing`
    /// key.
    pub fn new(uri: AbsoluteUri, existing: K) -> Self {
        Self { uri, existing }
    }

    pub fn fail<T>(uri: AbsoluteUri, existing: K) -> Result<T, Self> {
        Err(Self::new(uri, existing))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    tests                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use slotmap::DefaultKey;

    use super::*;

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    struct Compiled {
        key: DefaultKey,
    }

    impl AsRef<DefaultKey> for Compiled {
        fn as_ref(&self) -> &DefaultKey {
            &self.key
        }
    }
    #[derive(Debug, PartialEq, Eq)]
    struct TestSchema<'int> {
        key: DefaultKey,
        _marker: PhantomData<&'int ()>,
    }
    impl<'int> AsRef<DefaultKey> for TestSchema<'int> {
        fn as_ref(&self) -> &DefaultKey {
            &self.key
        }
    }
    impl<'int> Schema<'int, DefaultKey> for TestSchema<'int> {
        fn key(&self) -> DefaultKey {
            self.key
        }

        fn source(&'int self) -> Source<'int> {
            todo!()
        }
    }

    impl CompiledSchema<DefaultKey> for Compiled {
        type Schema<'int> = TestSchema<'int>;

        fn set_key(&mut self, key: DefaultKey) {
            self.key = key;
        }

        fn to_schema<'int>(&'int self, _sources: &'int Sources) -> Self::Schema<'int> {
            TestSchema {
                key: self.key,
                _marker: PhantomData,
            }
        }

        fn key(&self) -> DefaultKey {
            self.key
        }

        fn source_key(&self) -> SourceKey {
            todo!()
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
