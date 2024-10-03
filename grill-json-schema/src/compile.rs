use crate::{
    keyword::context,
    spec::{self, Compile, Specification},
};
use grill_core::{
    resolve::Error as ResolveError,
    source::{LinkError, SourceConflictError},
    Key, Resolve,
};
use grill_uri::AbsoluteUri;
use item::{Compiled, Pending, Queue};
use scan::Scanner;
use std::{
    error::Error as StdError,
    f32::consts::E,
    fmt,
    ops::{Deref, DerefMut},
};

mod item;
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
    InvalidSchema(Box<InvalidSchemaError<O>>),
    FailedToResolve(Box<ResolveError<R>>),
    SourceConflict(Box<SourceConflictError>),
    InvalidUri(Box<grill_uri::Error>),
}

impl<O, R> From<ResolveError<R>> for Error<O, R>
where
    R: 'static + Resolve,
{
    fn from(value: ResolveError<R>) -> Self {
        Self::FailedToResolve(Box::new(value))
    }
}

impl<O, R> StdError for Error<O, R>
where
    O: 'static + StdError,
    R: 'static + Resolve,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::InvalidSchema(e) => Some(e),
            Error::FailedToResolve(e) => Some(e),
            Error::SourceConflict(e) => Some(e),
            Error::InvalidUri(e) => Some(e),
        }
    }
}

impl<R, S, K> spec::CompileError<R, S, K> for Error<S::Report<'static>, R>
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
            Error::InvalidSchema(r) => fmt::Display::fmt(r, f),
            Error::FailedToResolve(r) => fmt::Display::fmt(r, f),
            Error::SourceConflict(r) => fmt::Display::fmt(r, f),
            Error::InvalidUri(r) => fmt::Display::fmt(r, f),
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
    scanner: Scanner<K>,
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
        let scanner = Scanner::new();
        let mut this = Self { ctx, scanner };
        let mut q = Queue::<K>::new(this.ctx.interrogator().targets.clone());
        let mut compiled = vec![K::default(); q.len()];
        while !q.is_empty() {
            let item = q.pop().unwrap();
            let Some(schema) = this.compile(&mut q, item).await? else {
                continue;
            };
            if let Some(index) = schema.index {
                compiled[index] = schema.key;
            }
        }
        Ok(compiled)
    }

    async fn compile(
        &mut self,
        q: &mut Queue<K>,
        item: Pending<K>,
    ) -> Result<Option<Compiled<K>>, S::CompileError<R>> {
        let scan = self.scanner.scan(self.ctx.language(), &item.uri).await;
        todo!()
    }
}

fn handle_err<T, E, S, K, R>(e: E, _continue_on_err: bool) -> Result<T, E>
where
    K: 'static + Key + Send + Sync,
    E: spec::CompileError<R, S, K>,
    S: Specification<K>,
    R: 'static + Resolve + Send + Sync,
{
    // TODO determine when to continue on error and if the trait should be responsible for it
    Err(e)
}
