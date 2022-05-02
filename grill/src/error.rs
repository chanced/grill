use std::path::PathBuf;
use thiserror::Error;

use std::result::Result as StdResult;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error deserializing schema: {}", source)]
    Deserialization { source: serde_json::Error },
    #[error("could not open schema file from {}: {}", filename.display(), source)]
    OpeningLocalSchema {
        filename: PathBuf,
        source: std::io::Error,
    },
    #[error("unsupported schema version: {schema}")]
    UnsupportedSchema { schema: String },
    #[error("invalid schema version: {schema}")]
    MalformedSchemaDraft { schema: String },
}
impl From<serde_json::Error> for Error {
    fn from(source: serde_json::Error) -> Self {
        Error::Deserialization { source }
    }
}
pub type Result<T> = StdResult<T, Error>;
