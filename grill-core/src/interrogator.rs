use crate::{
    cache::Cache,
    lang::{Compile, Evaluate},
    state::{State, Transaction},
    DefaultKey, Language, Resolve,
};

use grill_uri::AbsoluteUri;
use serde_json::Value;
use slotmap::Key;

/// Evaluates the integrity of data through a schema language.
pub struct Interrogator<L: Language<K>, K: 'static + Key + Send + Sync = DefaultKey> {
    lang: L,
    state: State<L, K>,
}

impl<L, K> Interrogator<L, K>
where
    L: 'static + Language<K>,
    K: 'static + Key + Send + Sync,
{
    fn init(mut self) -> Self {
        self.lang.init(&mut self.state);
        self
    }

    // fn iter(&self) -> (){
    //     self.schemas.
    // }
    /// Creates a new `Interrogator`.
    pub fn new(lang: L) -> Self {
        Self {
            lang,
            state: State::new(),
        }
        .init()
    }

    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    pub async fn compile<R>(
        &mut self,
        uri: AbsoluteUri,
        resolve: &R,
    ) -> Result<K, L::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
    {
        // not crazy about a vec being allocated as part of this round-trip.
        Ok(self
            .compile_all(vec![uri], resolve)
            .await?
            .into_iter()
            .next()
            .expect("compile returned with an empty vec"))
    }

    pub async fn compile_all<R>(
        &mut self,
        uris: Vec<AbsoluteUri>,
        resolve: &R,
    ) -> Result<Vec<K>, L::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
    {
        self._compile(uris, resolve, true).await
    }

    async fn _compile<'int, 'txn, 'res, R>(
        &'int mut self,
        uris: Vec<AbsoluteUri>,
        resolve: &'res R,
        validate: bool,
    ) -> Result<Vec<K>, L::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
    {
        // I really wanted to use a closure here, but I simply could not get it to work.
        let mut schemas = self.state.schemas.clone();
        let mut sources = self.state.sources.clone();
        let transaction = Transaction::new(&mut schemas, &mut sources, &mut self.state.cache);
        let keys = self
            .lang
            .compile(Compile::new(uris, transaction, resolve, validate))
            .await?;
        self.state.schemas = schemas;
        self.state.sources = sources;
        Ok(keys)
    }

    /// Evaluates a schema for the given [`Evaluate`] request.
    pub fn evaluate<'int, 'val>(
        &'int self,
        key: K,
        context: L::Context,
        value: &'val Value,
    ) -> L::EvaluateResult<'val>
    where
        L::Context: 'int,
    {
        // TODO: pool these
        let mut eval = Cache::new();

        self.lang.evaluate(Evaluate {
            context,
            value,
            key,
            state: &self.state,
            eval: &mut eval,
        })
    }
}
