use crate::spec::{self, Compile, Specification};
use grill_core::{resolve::ResolveError, Key, Resolve};
use grill_uri::AbsoluteUri;
use item::{Compiled, Pending, Queue};
use std::{error::Error, fmt};

mod item;
mod resolve;

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
    let a = "".try_into().unwrap();
    scan::<R, S, K>(&mut ctx, &a);
    Compiler::<R, S, K>::execute(ctx).await
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 CompileError                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Failed to compile a schema.
#[derive(Debug)]
pub enum CompileError<O, R>
where
    R: 'static + Resolve,
{
    InvalidSchema(InvalidSchemaError<O>),
    FailedToResolve(ResolveError<R>),
}

impl<O, R> Error for CompileError<O, R>
where
    O: 'static + Error,
    R: 'static + Resolve,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidSchema(e) => Some(e),
            CompileError::FailedToResolve(e) => Some(e),
        }
    }
}

impl<S, K, R> spec::CompileError<S, K, R> for CompileError<S::Report<'static>, R>
where
    K: 'static + Key + Send + Sync,
    S: 'static + Specification<K> + Send + Sync,
    R: 'static + Resolve,
{
}

impl<O, R> fmt::Display for CompileError<O, R>
where
    O: fmt::Display,
    R: 'static + Resolve,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(r) => fmt::Display::fmt(r, f),
            CompileError::FailedToResolve(r) => fmt::Display::fmt(r, f),
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
impl<O: 'static + Error> Error for InvalidSchemaError<O> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
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
        let mut q = Queue::<K>::new(this.ctx.targets().to_vec());
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

#[allow(clippy::unused_async)]

async fn scan<R, S, K>(ctx: &mut S::Compile<'_, '_, '_, R>, _uri: &AbsoluteUri)
where
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve + Send + Sync,
{
    let r = ctx.resolve();
}

fn handle<E, S, K, R>(
    compiled: &mut Vec<K>,
    result: Result<Option<Compiled<K>>, E>,
    continue_on_err: bool,
) -> Result<(), E>
where
    K: 'static + Key + Send + Sync,
    E: spec::CompileError<S, K, R>,
    S: Specification<K>,
    R: 'static + Resolve + Send + Sync,
{
    match result {
        Ok(ok) => handle_ok(compiled, ok),
        Err(err) => handle_err(err, continue_on_err),
    }
}

#[allow(clippy::unnecessary_wraps)]
fn handle_ok<E, K>(compiled: &mut Vec<K>, result: Option<Compiled<K>>) -> Result<(), E>
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
    E: spec::CompileError<S, K, R>,
    S: Specification<K>,
    R: 'static + Resolve + Send + Sync,
{
    // TODO determine when to continue on error and if the trait should be responsible for it
    Err(e)
}
