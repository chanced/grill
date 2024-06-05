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
pub struct Ancestors<'i, S, K> {
    _schema: S,
    _key: K,
    _marker: std::marker::PhantomData<&'i ()>,
}

pub fn all_compiled_schemas<S, K: Key>(schemas: &Schemas<S, K>) -> AllCompiledSchemas<'_, S, K> {
    AllCompiledSchemas {
        schemas,
        iter: schemas.schemas.iter(),
    }
}

#[derive(Debug)]
pub struct AllCompiledSchemas<'i, S, K: Key = DefaultKey> {
    schemas: &'i Schemas<S, K>,
    iter: slotmap::basic::Iter<'i, K, S>,
}

impl<'i, S, K: Key> Iterator for AllCompiledSchemas<'i, S, K> {
    type Item = &'i S;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct Iter<'i, I, S, K: Key> {
    iter: I,
    schemas: &'i Schemas<S, K>,
    sources: &'i Sources,
}

impl<'i, I, S, K> Iterator for Iter<'i, I, S, K>
where
    I: Iterator,
    I::Item: AsRef<K>,
    S: CompiledSchema<K>,
    K: Key,
{
    type Item = Result<S::Schema<'i>, InvalidKeyError<K>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|i| *i.as_ref())
            .map(|k| self.schemas.get(k).map(|k| k.to_schema(&self.sources)))
    }
}

pub struct AllSchemas<'i, S, K: Key>(Unchecked<Iter<'i, AllCompiledSchemas<'i, S, K>, S, K>>);

impl<'i, S, K> Iterator for AllSchemas<'i, S, K>
where
    S: CompiledSchema<K>,
    K: Key,
{
    type Item = S::Schema<'i>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
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
