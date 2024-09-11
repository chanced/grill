use std::{future::Future, pin::Pin};

use slotmap::Key;

use crate::{
    cache::Cache,
    lang::{Schemas, Sources},
    Language,
};

pub struct State<L: Language<K>, K: 'static + Key + Send + Sync> {
    /// Schema graph
    pub schemas: Schemas<L::CompiledSchema, K>,
    /// Source repository
    pub sources: Sources,
    /// Value and number cache
    pub cache: Cache,
}

impl<L: Language<K>, K: 'static + Key + Send + Sync> State<L, K> {
    pub fn new() -> Self {
        Self {
            schemas: Schemas::new(),
            sources: Sources::new(),
            cache: Cache::new(),
        }
    }

    // TODO: figure out how to get this async closure to work.
    pub async fn transaction<F, O, E>(&mut self, f: F) -> Result<O, E>
    where
        L::CompiledSchema: Send + Sync + 'static,
        F: Send + FnOnce(Transaction<L, K>) -> Pin<Box<dyn Future<Output = Result<O, E>>>>,
        O: Send + 'static,
        E: Send + 'static,
    {
        let mut schemas = self.schemas.clone();
        let mut sources = self.sources.clone();

        let txn = Transaction::new(&mut schemas, &mut sources, &mut self.cache);
        let v = f(txn).await?;
        self.schemas = schemas;
        self.sources = sources;
        Ok(v)
    }
}

impl<L: Language<K>, K: 'static + Key + Send + Sync> Default for State<L, K> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Transaction<'x, 'int, L: Language<K>, K: 'static + Key + Send + Sync> {
    pub schemas: &'x mut Schemas<L::CompiledSchema, K>,
    pub sources: &'x mut Sources,
    pub cache: &'int mut Cache,
}

impl<'x, 'int, L: Language<K>, K: 'static + Key + Send + Sync> Transaction<'x, 'int, L, K> {
    pub fn new(
        schemas: &'x mut Schemas<L::CompiledSchema, K>,
        sources: &'x mut Sources,
        cache: &'int mut Cache,
    ) -> Self {
        Self {
            schemas,
            sources,
            cache,
        }
    }
}
