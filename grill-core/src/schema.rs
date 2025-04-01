//! Schema definitions and data structures.

use core::fmt;
use grill_uri::AbsoluteUri;
use slotmap::SlotMap;
use std::collections::HashMap;

// use crate::iter::{AllCompiledSchemas, AllSchemas, Iter};

use crate::key::InternalKey;
use crate::Key;

use super::source::{Source, SourceKey, Sources};

/// A trait which indicates that a schema is capable of being embedded in
/// another schema.
pub trait EmbeddedIn {
    /// Returns the key of the schema that this schema is embedded in, if any.
    fn embedded_in(&self) -> Option<Key>;
}

pub trait ReferencedBy {
    type Ref;
    fn referenced_by(&self) -> &[Self::Ref];
}

pub trait Reference {
    fn reference(&self) -> Key;
    fn referrer(&self) -> Key;
}

pub trait References {
    type Ref: Reference;
    fn references(&self) -> &[Self::Ref];
}

pub trait Embedded {
    fn embedded(&self) -> &[Key];
}

pub trait Schema<'i> {
    fn key(&self) -> Key;
    fn source(&'i self) -> Source<'i>;
}

pub trait CompiledSchema {
    /// The borrowed schema representation.
    type Schema<'i>: Schema<'i>
    where
        Self: 'i;

    fn source_key(&self) -> SourceKey;

    fn set_key(&mut self, key: Key);

    /// Returns the key of the schema.
    fn key(&self) -> Key;

    /// Returns the borrowed [`Self::Schema`] representation.
    fn to_schema<'int>(&'int self, sources: &'int Sources) -> Self::Schema<'int>;
}

#[derive(Debug, Clone)]
pub struct Schemas<S> {
    pub(crate) map: SlotMap<InternalKey, S>,
    uris: HashMap<AbsoluteUri, InternalKey>,
}

impl<S> Default for Schemas<S> {
    fn default() -> Self {
        Self::new()
    }
}
impl<S> Schemas<S> {
    /// Creates a new schema graph.
    pub fn new() -> Self {
        Self {
            map: SlotMap::with_key(),
            uris: HashMap::new(),
        }
    }
}

impl<S> Schemas<S>
where
    S: CompiledSchema,
{
    /// Inserts `schema` into the graph and returns its key.
    pub fn insert(&mut self, schema: S) -> Key {
        let key: Key = self.map.insert(schema).into();
        self.map.get_mut(key.0).unwrap().set_key(key);
        key
    }

    /// Assigns an `AbsoluteUri` to a schema key.
    ///
    /// # Errors
    /// Returns [`DuplicateLinkError`] if a schema is already linked to the
    /// given `uri`.
    pub fn assign(&mut self, uri: AbsoluteUri, key: Key) -> Result<(), DuplicateLinkError> {
        match self.uris.get(&uri).copied() {
            Some(existing) => {
                if existing != key.0 {
                    DuplicateLinkError::fail(uri, existing.into())?
                }
            }
            None => self.insert_uri(uri, key),
        }
        Ok(())
    }

    fn insert_uri(&mut self, uri: AbsoluteUri, key: Key) {
        self.uris.insert(uri, key.0);
    }

    pub fn get_key_of(&self, uri: &AbsoluteUri) -> Option<Key> {
        self.uris.get(uri).copied().map(Into::into)
    }
    /// Returns [`Self::C::Schema`](CompiledSchema::Schema) for the supplied
    /// [`AbsoluteUri`], if it exists.
    pub fn get_by_uri(&self, uri: &AbsoluteUri) -> Option<&S> {
        self.uris
            .get(uri)
            .copied()
            .map(|k| self.get_compiled(k.into()))
    }

    /// Returns a reference to compiled schema ([`Self::C`](`CompiledSchema`))
    /// with the supplied `key` or returns `InvalidKeyError` if the key does
    /// not exist.
    pub fn get_compiled(&self, key: Key) -> &S {
        self.map.get(key.0).expect("invalid schema key")
    }

    /// Returns a mutable reference to the schema ([`C`](`CompiledSchema`)) with
    /// the given key.
    pub fn get_mut(&mut self, key: Key) -> Option<&mut S> {
        self.map.get_mut(key.0)
    }

    // pub fn all_compiled_schemas(&self) -> AllCompiledSchemas<'_, S, Key> {
    //     AllCompiledSchemas::new(self)
    // }

    // pub fn all_schemas<'int>(&'int self, sources: &'int Sources) -> AllSchemas<'int, S, Key> {
    //     self.all_compiled_schemas().into_all_schemas(sources)
    // }

    // pub fn iter<'int, I>(&'int self, sources: &'int Sources, keys: I) -> Iter<'int, I, S, Key>
    // where
    //     I: 'int + Iterator<Item = Key>,
    // {
    //     Iter::new(self, sources, keys)
    // }
    // pub fn keys(&self) -> impl Iterator<Item = Key> + '_ {
    //     self.map.keys()
    // }
    // pub fn keys_and_compiled_schemas(&self) -> impl Iterator<Item = (Key, &S)> + '_ {
    //     self.map.iter()
    // }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn contains_key(&self, key: Key) -> bool {
        self.map.contains_key(key.0)
    }
    pub fn contains_uri(&self, uri: &AbsoluteUri) -> bool {
        self.uris.contains_key(uri)
    }
}

/// A duplicate [`CompiledSchema`] already exists at the given `uri`.
#[derive(Debug, PartialEq)]
pub struct DuplicateLinkError {
    /// The URI that the schema is already linked to.
    pub uri: AbsoluteUri,
    /// The key of the existing schema.
    pub existing: Key,
}
impl DuplicateLinkError {
    /// Creates a new `DuplicateLinkError` with the given `uri` and `existing`
    /// key.
    pub fn new(uri: AbsoluteUri, existing: Key) -> Self {
        Self { uri, existing }
    }

    pub fn fail<T>(uri: AbsoluteUri, existing: Key) -> Result<T, Self> {
        Err(Self::new(uri, existing))
    }
}

// #[cfg(test)]
// mod tests {
//     use std::marker::PhantomData;

//     use slotmap::DefaultKey;

//     use super::*;

//     #[derive(Clone, Debug, PartialEq, Eq)]
//     struct Compiled {
//         key: Key,
//     }

//     impl AsRef<DefaultKey> for Compiled {
//         fn as_ref(&self) -> &Key {
//             &self.key
//         }
//     }
//     #[derive(Debug, PartialEq, Eq)]
//     struct TetSchema<'int> {
//         key: DefaultKey,
//         _marker: PhantomData<&'int ()>,
//     }
//     impl<'int> AsRef<DefaultKey> for TestSchema<'int> {
//         fn as_ref(&self) -> &DefaultKey {
//             &self.key
//         }
//     }
//     impl<'int> Schema<'int> for TestSchema<'int> {
//         fn key(&self) -> Key {
//             self.key
//         }

//         fn source(&'int self) -> Source<'int> {
//             todo!()
//         }
//     }

//     impl CompiledSchema for Compiled {
//         type Schema<'int> = TestSchema<'int>;

//         fn set_key(&mut self, key: Key) {
//             self.key = key;
//         }

//         fn to_schema<'int>(&'int self, _sources: &'int Sources) -> Self::Schema<'int> {
//             TestSchema {
//                 key: self.key,
//                 _marker: PhantomData,
//             }
//         }

//         fn key(&self) -> Key {
//             self.key
//         }

//         fn source_key(&self) -> SourceKey {
//             todo!()
//         }
//     }

//     #[test]
//     fn test_insert_ref() {
//         let mut schemas: Schemas<Compiled, DefaultKey> = Schemas::new();
//         let key = schemas.insert(Compiled::default());

//         assert_ne!(key, DefaultKey::default());
//         let uri = AbsoluteUri::parse("https://example.com/schema.json").unwrap();
//         schemas.assign(uri.clone(), key).unwrap();
//         assert_eq!(schemas.get_by_uri(&uri).unwrap().key, key);
//     }
// }
