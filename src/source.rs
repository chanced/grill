use serde_json::Value;
use uniresid::Uri;

/// A source document containing one or more JSON Schemas.
pub struct Source {
    /// The URI of the source document.
    pub id: Uri,
    /// The JSON Schema contained in the source document.
    pub value: Value,
}

impl From<(Uri, Value)> for Source {
    fn from(value: (Uri, Value)) -> Self {
        let (id, value) = value;
        Self { id, value }
    }
}
