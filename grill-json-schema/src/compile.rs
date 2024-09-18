use crate::spec::{self, compile::Context, Specification};
use grill_core::{resolve::Error as ResolveError, Key, Resolve};
use grill_uri::AbsoluteUri;
use item::{Compiled, Pending, Queue};
use std::{error::Error as StdError, fmt};

mod item;
mod resolve;
mod scan;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   compile                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub(crate) async fn compile<'int, 'txn, 'res, R, S, K>(
    mut ctx: S::Compile<'int, 'txn, 'res, R>,
) -> Result<Vec<K>, S::CompileError<R>>
where
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    'int: 'res,
    'txn: 'int,
    'res: 'int,
{
    Compiler::<R, S, K>::execute(ctx).await
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Error                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Failed to compile a schema.
#[derive(Debug)]
pub enum Error<O, R>
where
    R: 'static + Resolve,
{
    InvalidSchema(InvalidSchemaError<O>),
    FailedToResolve(ResolveError<R>),
}

impl<O, R> From<ResolveError<R>> for Error<O, R>
where
    R: 'static + Resolve,
{
    fn from(value: ResolveError<R>) -> Self {
        Self::FailedToResolve(value)
    }
}

impl<O, R> StdError for Error<O, R>
where
    O: 'static + StdError,
    R: 'static + Resolve,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::InvalidSchema(e) => Some(e),
            Error::FailedToResolve(e) => Some(e),
        }
    }
}

impl<R, S, K> spec::compile::Error<R, S, K> for Error<S::Report<'static>, R>
where
    K: 'static + Key + Send + Sync,
    S: 'static + Specification<K> + Send + Sync,
    R: 'static + Resolve,
{
}

impl<O, R> fmt::Display for Error<O, R>
where
    O: fmt::Display,
    R: 'static + Resolve,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(r) => fmt::Display::fmt(r, f),
            Error::FailedToResolve(r) => fmt::Display::fmt(r, f),
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                              InvalidSchemaError                              ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug)]
pub struct InvalidSchemaError<O> {
    pub report: O,
    pub uri: AbsoluteUri,
}

impl<O: fmt::Display> fmt::Display for InvalidSchemaError<O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "schema \"{}\" failed validation", self.uri)
    }
}
impl<O: 'static + StdError> StdError for InvalidSchemaError<O> {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.report)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Compiler                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

struct Compiler<'int, 'txn, 'res, R, S, K>
where
    S: 'static + Specification<K>,
    R: 'static + Resolve + Send + Sync,
    K: 'static + Key + Send + Sync,
    Self: 'txn + 'int + 'res,
    'int: 'txn,
{
    ctx: S::Compile<'int, 'txn, 'res, R>,
}

impl<'int, 'txn, 'res, R, S, K> Compiler<'int, 'txn, 'res, R, S, K>
where
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
{
    async fn execute(ctx: S::Compile<'int, 'txn, 'res, R>) -> Result<Vec<K>, S::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: Specification<K>,
        K: 'static + Key + Send,
    {
        let mut this = Self { ctx };

        let mut q = Queue::<K>::new(this.ctx.core_ctx().targets.clone());
        let mut compiled = vec![K::default(); q.len()];
        while !q.is_empty() {
            let item = q.pop().unwrap();
            let continue_on_err = item.continue_on_err;
            let result = this.compile(&mut q, item).await;
            handle(&mut compiled, result, continue_on_err)?;
        }
        Ok(compiled)
    }

    async fn compile(
        &mut self,
        q: &mut Queue<K>,
        item: Pending<K>,
    ) -> Result<Option<Compiled<K>>, S::CompileError<R>> {
        todo!()
    }
}

fn handle<E, S, K, R>(
    compiled: &mut [K],
    result: Result<Option<Compiled<K>>, E>,
    continue_on_err: bool,
) -> Result<(), E>
where
    K: 'static + Key + Send + Sync,
    E: spec::compile::Error<R, S, K>,
    S: Specification<K>,
    R: 'static + Resolve + Send + Sync,
{
    match result {
        Ok(ok) => handle_ok(compiled, ok),
        Err(err) => handle_err(err, continue_on_err),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn handle_ok<E, K>(compiled: &mut [K], result: Option<Compiled<K>>) -> Result<(), E>
where
    K: 'static + Key + Send + Sync,
{
    let Some(schema) = result else { return Ok(()) };
    if let Some(index) = schema.index {
        compiled[index] = schema.key;
    }
    Ok(())
}

fn handle_err<T, E, S, K, R>(e: E, _continue_on_err: bool) -> Result<T, E>
where
    K: 'static + Key + Send + Sync,
    E: spec::compile::Error<R, S, K>,
    S: Specification<K>,
    R: 'static + Resolve + Send + Sync,
{
    // TODO determine when to continue on error and if the trait should be responsible for it
    Err(e)
}
