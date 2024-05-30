pub mod dialect;

use grill_core::{lang, Key};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSchema<W, K> {
    key: K,
    keywords: Box<[W]>,
}

impl<W, K: 'static + Key> lang::schema::CompiledSchema<K> for CompiledSchema<W, K> {
    type Schema<'i> = Schema<'i, K>;

    fn set_key(&mut self, key: K) {
        self.key = key;
    }

    fn as_schema<'i>(&self, sources: &lang::Sources) -> Self::Schema<'i> {
        todo!()
    }
}
