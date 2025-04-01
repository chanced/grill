use crate::spec::{self, Compile, Specification};
use grill_core::{Key, Resolve, resolve::Error as ResolveError, source::SourceConflictError};
use grill_uri::AbsoluteUri;
use scan::Scanner;
use std::{error::Error as StdError, fmt};

mod scan;
use std::collections::VecDeque;

use crate::schema::Reference;

#[derive(Debug)]
pub(super) struct Pending {
    pub(super) key: Option<Key>,
    pub(super) uri: AbsoluteUri,
    pub(super) parent: Option<Key>,
    /// Whether to diregard some errors
    pub(super) continue_on_err: bool,
    pub(super) reference: Option<Reference>,
    // index in the output queue
    pub(super) index: Option<usize>,
}

pub(super) struct Compiled {
    pub(super) key: Key,
    pub(super) index: Option<usize>,
}

#[derive(Debug)]
pub(super) struct Queue {
    items: VecDeque<Pending>,
}
impl IntoIterator for Queue {
    type Item = Pending;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl Queue {
    pub(super) fn new(uris: Vec<AbsoluteUri>) -> Self {
        Self {
            items: uris
                .into_iter()
                .enumerate()
                .map(|(index, uri)| Pending {
                    key: None,
                    uri,
                    parent: None,
                    continue_on_err: false,
                    reference: None,
                    index: Some(index),
                })
                .collect(),
        }
    }
    pub(super) fn len(&self) -> usize {
        self.items.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub(super) fn push_back(&mut self, item: Pending) {
        self.items.push_back(item);
    }
    pub(super) fn push_front(&mut self, item: Pending) {
        self.items.push_front(item);
    }
    pub(super) fn pop(&mut self) -> Option<Pending> {
        self.items.pop_front()
    }
    pub(super) fn contains_key(&self, key: Key) -> bool {
        self.items.iter().any(|item| item.key == Some(key))
    }
}

pub(crate) async fn compile<'int, 'txn, 'res, R, S>(
    ctx: S::Compile<'int, 'txn, 'res, R>,
) -> Result<Vec<Key>, S::CompileError<R>>
where
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification,
    'int: 'res,
    'txn: 'int,
    'res: 'int,
{
    Compiler::<R, S>::execute(ctx).await
}

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

impl<R, S> spec::CompileError<R, S> for Error<S::Report<'static>, R>
where
    S: 'static + Specification + Send + Sync,
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

struct Compiler<'int, 'txn, 'res, R, S>
where
    S: 'static + Specification,
    R: 'static + Resolve + Send + Sync,
    Self: 'txn + 'int + 'res,
    'int: 'txn,
{
    ctx: S::Compile<'int, 'txn, 'res, R>,
    scanner: Scanner,
}

impl<'int, 'txn, 'res, R, S> Compiler<'int, 'txn, 'res, R, S>
where
    R: 'static + Resolve + Send + Sync,
    S: 'static + Specification,
{
    async fn execute(ctx: S::Compile<'int, 'txn, 'res, R>) -> Result<Vec<Key>, S::CompileError<R>>
    where
        R: 'static + Resolve + Send + Sync,
        S: Specification,
    {
        let scanner = Scanner::new();
        let mut this = Self { ctx, scanner };
        let mut q = Queue::new(this.ctx.instance().targets.clone());
        let mut compiled = Vec::new();

        while !q.is_empty() {
            let item = q.pop().unwrap();
            let Some(schema) = this.compile(&mut q, item).await? else {
                continue;
            };
            if schema.index.is_some() {
                compiled.push(schema)
            }
        }
        compiled.sort_by_key(|schema| schema.index.unwrap());
        Ok(compiled.into_iter().map(|schema| schema.key).collect())
    }

    async fn compile(
        &mut self,
        q: &mut Queue,
        item: Pending,
    ) -> Result<Option<Compiled>, S::CompileError<R>> {
        let scan = self.scanner.scan(self.ctx.language(), &item.uri).await;
        todo!()
    }
}

fn handle_err<T, E, S, R>(e: E, _continue_on_err: bool) -> Result<T, E>
where
    E: spec::CompileError<R, S>,
    S: Specification,
    R: 'static + Resolve + Send + Sync,
{
    // TODO determine when to continue on error and if the trait should be responsible for it
    Err(e)
}
