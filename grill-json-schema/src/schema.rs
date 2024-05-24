pub mod dialect;

use grill_core::{lang, new_key_type, Key};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledSchema<K> {
    key: K,
}

impl<'i, K: Key> lang::schema::Schema<'i, K> for Schema<'i, K> {
    fn key(&self) -> K {
        self.key
    }
}

impl<K: 'static + Key> lang::schema::CompiledSchema<K> for CompiledSchema<K> {
    type Schema<'i> = Schema<'i, K>;

    fn set_key(&mut self, key: K) {
        self.key = key;
    }

    fn as_schema<'i>(&self, sources: &lang::Sources) -> Self::Schema<'i> {
        todo!()
    }
}

/// A JSON Schema.
pub struct Schema<'i, K> {
    key: K,
    _marker: std::marker::PhantomData<&'i K>,
}
