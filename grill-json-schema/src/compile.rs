use grill_core::lang::source::{InsertError, LinkError, SourceError};
use grill_uri::AbsoluteUri;
use serde_json::Value;
use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(display("failed to compile schema \"{uri}\""))]
pub struct CompileError {
    pub uri: AbsoluteUri,
    pub value: Box<Value>,
    #[snafu(source, backtrace)]
    pub cause: CompileErrorCause,
}

#[derive(Debug, Snafu)]
pub enum CompileErrorCause {
    #[snafu(display("failed to source schema"))]
    Source { source: SourceError },
}
