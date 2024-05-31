use std::fmt::Debug;

use grill_uri::AbsoluteUri;
use snafu::Snafu;

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
