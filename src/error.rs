use snafu::prelude::*;
use std::path::PathBuf;

use std::result::Result as StdResult;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("error deserializing schema: {}", source))]
    Deserialization { source: serde_json::Error },
    #[snafu(display("could not open schema file from {}: {}", filename.display(), source))]
    OpeningLocalSchema {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("unsupported schema version: {schema}"))]
    UnsupportedSchema { schema: String },
    #[snafu(display("invalid schema version: {schema}"))]
    MalformedSchemaDraft { schema: String },
}
impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::Deserialization { source }
    }
}
pub type Result<T> = StdResult<T, Error>;
