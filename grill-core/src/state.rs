use std::{future::Future, pin::Pin};

use slotmap::Key;

use crate::{cache::Cache, schema::Schemas, source::Sources, Language};

pub struct State<L: Language> {
    /// Schema graph
    pub schemas: Schemas<L::CompiledSchema>,
    /// Source repository
    pub sources: Sources,
    /// Value and number cache
    pub cache: Cache,
}

impl<L: Language> State<L> {
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
        F: Send + FnOnce(Transaction<L>) -> Pin<Box<dyn Future<Output = Result<O, E>>>>,
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

impl<L: Language> Default for State<L> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Transaction<'int, 'txn, L: Language> {
    pub schemas: &'txn mut Schemas<L::CompiledSchema>,
    pub sources: &'txn mut Sources,
    pub cache: &'int mut Cache,
}

impl<'int, 'txn, L: Language> Transaction<'int, 'txn, L> {
    pub fn new(
        schemas: &'txn mut Schemas<L::CompiledSchema>,
        sources: &'txn mut Sources,
        cache: &'int mut Cache,
    ) -> Self {
        Self {
            schemas,
            sources,
            cache,
        }
    }
}
