use crate::{
    spec::{CompileError as _, Specification},
    Report,
};
use grill_core::{resolve::ResolveError, Key, Resolve};
use grill_uri::AbsoluteUri;
use item::{Compiled, Pending, Queue};
use resolve::Resolved;
use std::fmt;

mod item;
mod resolve;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    compile                                   ║
║                                   ¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub(crate) async fn compile<'int, 'txn, 'res, R, S, K>(
    ctx: S::Compile<'int, 'txn, 'res, R>,
) -> Result<Vec<K>, S::CompileError<R>>
where
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
    'int: 'res,
    'txn: 'int,
    'res: 'int,
{
    Compiler::<R, S, K>::new(ctx).compile().await
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
    InvalidSchema(O),
    FailedToResolve(ResolveError<R>),
}

impl<O, R> std::error::Error for CompileError<O, R>
where
    O: 'static + std::error::Error,
    R: 'static + Resolve,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidSchema(e) => Some(e),
            CompileError::FailedToResolve(e) => Some(e),
        }
    }
}

impl<S, K, R> crate::spec::CompileError<S, K, R> for CompileError<S::Report<'static>, R>
where
    K: 'static + Key + Send + Sync,
    S: 'static + Specification<K> + Send + Sync,
    R: 'static + Resolve,
{
    fn is_recoverable(&self) -> bool {
        todo!()
    }
}

impl<A, E, R> From<Report<A, E>> for CompileError<Report<A, E>, R>
where
    R: 'static + Resolve,
{
    fn from(r: Report<A, E>) -> Self {
        Self::InvalidSchema(r)
    }
}

impl<O, R> fmt::Display for CompileError<O, R>
where
    O: fmt::Display,
    R: 'static + Resolve,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(r) => write!(f, "schema was invalid{}", r),
            CompileError::FailedToResolve(_) => todo!(),
        }
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
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification<K>,
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
    fn new(ctx: S::Compile<'int, 'txn, 'res, R>) -> Self {
        Self { ctx }
    }
    async fn compile(mut self) -> Result<Vec<K>, S::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: Specification<K>,
        K: 'static + Key + Send,
    {
        // let mut q = Queue::<K>::new(self.ctx.targets().to_vec());
        // let mut compiled = Vec::with_capacity(self.ctx.targets().len());
        // while !q.is_empty() {
        //     let item = q.pop().unwrap();
        //     let continue_on_err = item.continue_on_err;
        //     match self.compile_schema(&mut q, item).await {
        //         Ok(schema) => self.complete(&mut compiled, schema),
        //         Err(e) => self.handle_error(e, continue_on_err),
        //     }?;
        // }
        // Ok(compiled)
        todo!()
    }

    async fn compile_schema(
        &mut self,
        q: &mut Queue<K>,
        item: Pending<K>,
    ) -> Result<Option<Compiled<K>>, S::CompileError<R>> {
        todo!()
    }

    async fn resolve(&mut self, uri: &AbsoluteUri) -> Result<Resolved, ResolveError<R>> {
        todo!()
    }

    fn complete(
        &mut self,
        compiled: &mut Vec<K>,
        result: Option<Compiled<K>>,
    ) -> Result<(), S::CompileError<R>> {
        let Some(schema) = result else { return Ok(()) };
        if schema.is_target {
            compiled.push(schema.key);
        }
        Ok(())
    }

    fn handle_error(
        &mut self,
        e: S::CompileError<R>,
        continue_on_err: bool,
    ) -> Result<(), S::CompileError<R>> {
        if continue_on_err && e.is_recoverable() {
            Ok(())
        } else {
            Err(e)
        }
    }
}

fn scan<R, S, K>(ctx: &mut S::Compile<'_, '_, '_, R>, uri: &AbsoluteUri)
where
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification<K>,
    K: 'static + Key + Send + Sync,
{
    todo!()
}
