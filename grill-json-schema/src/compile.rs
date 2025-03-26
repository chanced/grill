use crate::spec::{self, Compile, Specification};
use grill_core::{resolve::Error as ResolveError, source::SourceConflictError, Key, Resolve};
use grill_uri::AbsoluteUri;
use scan::Scanner;
use std::{error::Error as StdError, fmt};

mod scan;
use std::collections::VecDeque;

use crate::schema::Reference;

#[derive(Debug)]
pub(super) struct Pending<K> {
    pub(super) key: Option<K>,
    pub(super) uri: AbsoluteUri,
    pub(super) parent: Option<K>,
    /// Whether to diregard some errors
    pub(super) continue_on_err: bool,
    pub(super) reference: Option<Reference<K>>,
    // index in the output queue
    pub(super) index: Option<usize>,
}

pub(super) struct Compiled<K> {
    pub(super) key: K,
    pub(super) index: Option<usize>,
}

#[derive(Debug)]
pub(super) struct Queue<K> {
    items: VecDeque<Pending<K>>,
}
impl<K> IntoIterator for Queue<K> {
    type Item = Pending<K>;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<K> Queue<K>
where
    K: Key,
{
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
    pub(super) fn push_back(&mut self, item: Pending<K>) {
        self.items.push_back(item);
    }
    pub(super) fn push_front(&mut self, item: Pending<K>) {
        self.items.push_front(item);
    }
    pub(super) fn pop(&mut self) -> Option<Pending<K>> {
        self.items.pop_front()
    }
    pub(super) fn contains_key(&self, key: K) -> bool {
        self.items.iter().any(|item| item.key == Some(key))
    }
}

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
