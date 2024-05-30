pub mod dialect;

use grill_core::{lang, Key};

use crate::spec::{self, Specification};

/// A JSON Schema.
pub struct Schema<'i, K> {
    key: K,
    _marker: std::marker::PhantomData<&'i K>,
}
impl<'i, K: Key> lang::schema::Schema<'i, K> for Schema<'i, K> {
    fn key(&self) -> K {
        self.key
    }
}

#[derive(Debug)]
pub struct CompiledSchema<S: Specification<K>, K: Key> {
    key: K,
    keywords: Box<[S::Keyword]>,
}
impl<S, K> PartialEq for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: Key,
{
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.keywords == other.keywords
    }
}
impl<S, K> Clone for CompiledSchema<S, K>
where
    S: Specification<K>,
    K: Key,
{
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            keywords: self.keywords.clone(),
        }
    }
}

impl<S, K> lang::schema::CompiledSchema<K> for CompiledSchema<S, K>
where
    K: 'static + Send + Key,
    S: Specification<K>,
{
    type Schema<'i> = Schema<'i, K>;

    fn set_key(&mut self, key: K) {
        self.key = key;
    }

    fn as_schema<'i>(&self, sources: &lang::Sources) -> Self::Schema<'i> {
        todo!()
    }
}
