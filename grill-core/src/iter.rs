//! Various [`Iterator`]s.

use slotmap::Key;

use crate::{
    lang::{
        schema::{CompiledSchema, InvalidKeyError},
        source::Sources,
        Schemas,
    },
    DefaultKey,
};

/// TODO: stubbed
pub struct Ancestors<'int, S, K> {
    _schema: S,
    _key: K,
    _marker: std::marker::PhantomData<&'int ()>,
}

pub fn all_compiled_schemas<S, K: Key>(schemas: &Schemas<S, K>) -> AllCompiledSchemas<'_, S, K> {
    AllCompiledSchemas::new(schemas)
}

#[derive(Debug)]
pub struct AllCompiledSchemas<'int, S, K: Key = DefaultKey> {
    iter: slotmap::basic::Iter<'int, K, S>,
}

impl<'int, S, K: Key> AllCompiledSchemas<'int, S, K> {
    pub fn new(schemas: &'int Schemas<S, K>) -> Self {
        Self {
            iter: schemas.map.iter(),
        }
    }
    pub fn into_all_schemas(self, sources: &'int Sources) -> AllSchemas<'int, S, K> {
        AllSchemas {
            compiled: self,
            sources,
        }
    }
}
impl<'int, S, K: Key> Iterator for AllCompiledSchemas<'int, S, K> {
    type Item = &'int S;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct Iter<'int, I, S, K: Key> {
    iter: I,
    schemas: &'int Schemas<S, K>,
    sources: &'int Sources,
}

impl<'int, I, S, K> Iterator for Iter<'int, I, S, K>
where
    I: Iterator,
    I::Item: AsRef<K>,
    S: CompiledSchema<K>,
    K: Key,
{
    type Item = Result<S::Schema<'int>, InvalidKeyError<K>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| *i.as_ref())
            .map(|k| self.schemas.get(k).map(|k| k.to_schema(self.sources)))
    }
}

impl<'int, I, S, K> Iter<'int, I, S, K>
where
    I: Iterator<Item = K>,
    K: Key,
{
    pub fn new(schemas: &'int Schemas<S, K>, sources: &'int Sources, keys: I) -> Self {
        Self {
            iter: keys,
            schemas,
            sources,
        }
    }
}

pub struct AllSchemas<'int, S, K: Key> {
    compiled: AllCompiledSchemas<'int, S, K>,
    sources: &'int Sources,
}

impl<'int, S, K> Iterator for AllSchemas<'int, S, K>
where
    S: CompiledSchema<K>,
    K: Key,
{
    type Item = S::Schema<'int>;

    fn next(&mut self) -> Option<Self::Item> {
        self.compiled.next().map(|s| s.to_schema(self.sources))
    }
}

pub struct Unchecked<I> {
    iter: I,
}

impl<I, T, K> Iterator for Unchecked<I>
where
    K: Key,
    I: Iterator<Item = Result<T, InvalidKeyError<K>>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Result::unwrap)
    }
}
