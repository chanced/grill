use std::fmt;
/// Declares the granularity of the serialized output for an `Evaluation`.
///
/// Each `Annotation` must specify the level of detail that it is to be included
/// in.
#[derive(Clone)]
pub enum Output {
    /// Provides information in a flat list structure.
    ///
    /// ## Examples
    /// ### JSON Schema
    /// ```json
    /// {
    ///     "valid": false,
    ///     "errors": [
    ///       {
    ///         "keywordLocation": "",
    ///         "instanceLocation": "",
    ///         "error": "A subschema had errors."
    ///       },
    ///       {
    ///         "keywordLocation": "/items/$ref",
    ///         "absoluteKeywordLocation":
    ///           "https://example.com/polygon#/$defs/point",
    ///         "instanceLocation": "/1",
    ///         "error": "A subschema had errors."
    ///       },
    ///       {
    ///         "keywordLocation": "/items/$ref/required",
    ///         "absoluteKeywordLocation":
    ///           "https://example.com/polygon#/$defs/point/required",
    ///         "instanceLocation": "/1",
    ///         "error": "Required property 'y' not found."
    ///       },
    ///       {
    ///         "keywordLocation": "/items/$ref/additionalProperties",
    ///         "absoluteKeywordLocation":
    ///           "https://example.com/polygon#/$defs/point/additionalProperties",
    ///         "instanceLocation": "/1/z",
    ///         "error": "Additional property 'z' found but was invalid."
    ///       },
    ///       {
    ///         "keywordLocation": "/minItems",
    ///         "instanceLocation": "",
    ///         "error": "Expected at least 3 items but found 2"
    ///       }
    ///     ]
    ///   }
    /// ```
    Basic,
    /// Provides information in a condensed hierarchical structure
    /// based on the structure of the schema.
    Detailed,
    /// Provides information in an uncondensed hierarchical structure that
    /// matches the exact structure of the schema.
    Verbose,
    /// For serialization of annotations exclusively for internal use.
    ///
    /// Specifying `Internal` as the output format includes all annotations of
    /// `Verbose`, `Detailed`, and `Basic`
    Internal,
}
impl fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic"),
            Self::Detailed => write!(f, "Detailed"),
            Self::Verbose => write!(f, "Verbose"),
            Self::Internal => write!(f, "Internal"),
        }
    }
}
impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic => write!(f, "Basic"),
            Self::Detailed => write!(f, "Detailed"),
            Self::Verbose => write!(f, "Verbose"),
            Self::Internal => write!(f, "Internal"),
        }
    }
}
