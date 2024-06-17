use std::fmt::Debug;

use grill_core::Key;
use grill_uri::AbsoluteUri;
use snafu::Snafu;

use crate::spec::Specification;

/// Failed to compile a schema.
#[derive(Debug, Snafu)]
#[snafu(display("failed to compile schema \"{uri}\""))]
pub struct CompileError<E: 'static + Debug> {
    /// [`AbsoluteUri`] of the schema.
    pub uri: AbsoluteUri,

    /// Cause of the error.
    #[snafu(source, backtrace)]
    pub cause: CompileErrorCause<E>,
}

/// The cause of a [`CompileError`].
#[derive(Debug, Snafu)]
pub enum CompileErrorCause<E> {
    #[snafu(display(""))]
    Temp { e: E },
}

pub struct Compiler<'i, S, K>
where
    S: 'i + Specification<K>,
    K: 'static + Key + Send,
{
    ctx: S::Compile<'i>,
}

impl<'i, S, K> Compiler<'i, S, K>
where
    S: 'i + Specification<K>,
    K: 'static + Key + Send,
{
    pub fn new(ctx: S::Compile<'i>) -> Self {
        Self { ctx }
    }
}
