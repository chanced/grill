mod annotation;
mod node;

pub use annotation::Annotation;
pub use node::Node;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Structure {
    Flag,
    /// The `Basic` structure is a flat list of output units.
    /// # Example
    /// ```json
    /// {
    ///   "valid": false,
    ///   "errors": [
    ///     {
    ///       "keywordLocation": "",
    ///       "instanceLocation": "",
    ///       "error": "A subschema had errors."
    ///     },
    ///     {
    ///       "keywordLocation": "/items/$ref",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point",
    ///       "instanceLocation": "/1",
    ///       "error": "A subschema had errors."
    ///     },
    ///     {
    ///       "keywordLocation": "/items/$ref/required",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point/required",
    ///       "instanceLocation": "/1",
    ///       "error": "Required property 'y' not found."
    ///     },
    ///     {
    ///       "keywordLocation": "/items/$ref/additionalProperties",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point/additionalProperties",
    ///       "instanceLocation": "/1/z",
    ///       "error": "Additional property 'z' found but was invalid."
    ///     },
    ///     {
    ///       "keywordLocation": "/minItems",
    ///       "instanceLocation": "",
    ///       "error": "Expected at least 3 items but found 2"
    ///     }
    ///   ]
    /// }
    /// ```
    Basic,
    /// The `Detailed` structure is based on the schema and can be more readable
    /// for both humans and machines. Having the structure organized this way
    /// makes associations between the errors more apparent. For example, the
    /// fact that the missing "y" property and the extra "z" property both stem
    /// from the same location in the instance is not immediately obvious in the
    /// "Basic" structure. In a hierarchy, the correlation is more easily
    /// identified.
    ///
    /// The following rules govern the construction of the results object:
    ///
    /// - All applicator keywords (`"*Of"`, `"$ref"`, `"if"`/`"then"`/`"else"`,
    ///   etc.) require a node.
    /// - Nodes that have no children are removed.
    /// - Nodes that have a single child are replaced by the child.
    /// - Branch nodes do not require an error message or an annotation.
    ///
    /// # Example
    ///
    /// ## Schema:
    /// ```json
    /// {
    ///   "$id": "https://example.com/polygon",
    ///   "$schema": "https://json-schema.org/draft/2020-12/schema",
    ///   "$defs": {
    ///     "point": {
    ///       "type": "object",
    ///       "properties": {
    ///         "x": { "type": "number" },
    ///         "y": { "type": "number" }
    ///       },
    ///       "additionalProperties": false,
    ///       "required": [ "x", "y" ]
    ///     }
    ///   },
    ///   "type": "array",
    ///   "items": { "$ref": "#/$defs/point" },
    ///   "minItems": 3
    /// }
    /// ```
    /// ## Instance:
    /// ```json
    /// [ { "x": 2.5, "y": 1.3 }, { "x": 1, "z": 6.7 } ]
    /// ```
    /// ## Output:
    ///
    /// ```json
    /// {
    ///   "valid": false,
    ///   "keywordLocation": "",
    ///   "instanceLocation": "",
    ///   "errors": [
    ///     {
    ///       "valid": false,
    ///       "keywordLocation": "/items/$ref",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point",
    ///       "instanceLocation": "/1",
    ///       "errors": [
    ///         {
    ///           "valid": false,
    ///           "keywordLocation": "/items/$ref/required",
    ///           "absoluteKeywordLocation":
    ///             "https://example.com/polygon#/$defs/point/required",
    ///           "instanceLocation": "/1",
    ///           "error": "Required property 'y' not found."
    ///         },
    ///         {
    ///           "valid": false,
    ///           "keywordLocation": "/items/$ref/additionalProperties",
    ///           "absoluteKeywordLocation":
    ///             "https://example.com/polygon#/$defs/point/additionalProperties",
    ///           "instanceLocation": "/1/z",
    ///           "error": "Additional property 'z' found but was invalid."
    ///         }
    ///       ]
    ///     },
    ///     {
    ///       "valid": false,
    ///       "keywordLocation": "/minItems",
    ///       "instanceLocation": "",
    ///       "error": "Expected at least 3 items but found 2"
    ///     }
    ///   ]
    /// }
    ///
    Detailed,
    Verbose,
    Complete,
}

impl Structure {
    /// Returns `true` if the structure is [`Flag`].
    ///
    /// [`Flag`]: Structure::Flag
    #[must_use]
    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag)
    }

    /// Returns `true` if the structure is [`Basic`].
    ///
    /// [`Basic`]: Structure::Basic
    #[must_use]
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic)
    }

    /// Returns `true` if the structure is [`Detailed`].
    ///
    /// [`Detailed`]: Structure::Detailed
    #[must_use]
    pub fn is_detailed(&self) -> bool {
        matches!(self, Self::Detailed)
    }

    /// Returns `true` if the structure is [`Verbose`].
    ///
    /// [`Verbose`]: Structure::Verbose
    #[must_use]
    pub fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose)
    }

    /// Returns `true` if the structure is [`Complete`].
    ///
    /// [`Complete`]: Structure::Complete
    #[must_use]
    pub fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// A trait which represents a validation error to be used as the `"error"` field in output
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait ValidationError<'v>: fmt::Display + fmt::Debug + DynClone + Send + Sync {}
dyn_clone::clone_trait_object!(<'v> ValidationError<'v>);

impl Serialize for dyn ValidationError<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl ValidationError<'_> for String {}

pub struct Flag<'v>(Annotation<'v>);

pub struct Basic<'v>(Annotation<'v>);

pub struct Detailed<'v>(Annotation<'v>);
pub struct Verbose<'v>(Annotation<'v>);

pub struct Complete<'v>(pub Annotation<'v>);

pub enum Output<'v> {
    Flag(Flag<'v>),
    Basic(Basic<'v>),
    Detailed(Detailed<'v>),
    Verbose(Verbose<'v>),
    Complete(Complete<'v>),
}
impl<'v> Output<'v> {
    pub(crate) fn new(structure: Structure, annotation: Annotation<'v>) -> Output {
        match structure {
            Structure::Flag => Output::Flag(Flag(annotation)), // TODO
            Structure::Basic => Output::Basic(Basic(annotation)), // TODO
            Structure::Detailed => Output::Detailed(Detailed(annotation)), // TODO
            Structure::Verbose => Output::Verbose(Verbose(annotation)), // TODO
            Structure::Complete => Output::Complete(Complete(annotation)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, collections::BTreeMap};

    use jsonptr::Pointer;

    use crate::Location;

    use super::*;

    #[test]
    fn test_annotiation_serde() {
        let mut additional_props = BTreeMap::new();
        additional_props.insert("example".into(), 34.into());

        let a = Annotation::Invalid(Node {
            additional_props,
            location: Location {
                keyword_location: "/".try_into().unwrap(),
                instance_location: "/".try_into().unwrap(),
                absolute_keyword_location: "http://example.com".to_string(),
            },
            error: None,
            annotations: vec![Annotation::Valid(Node {
                annotations: vec![],
                errors: vec![],
                location: Location {
                    instance_location: Pointer::new(&["baddata"]),
                    keyword_location: Pointer::new(&["error-keyword"]),
                    ..Default::default()
                },
                error: Some(Box::new(String::from("bad data"))),
                ..Default::default()
            })],
            errors: vec![Annotation::Invalid(Node {
                annotations: vec![],
                errors: vec![],
                error: Some(Box::new(String::from("nested error"))),
                location: Location {
                    absolute_keyword_location: "http://example.com".to_string(),
                    ..Default::default()
                },

                ..Default::default()
            })],
        });

        let s = serde_json::to_string(&a).unwrap();
        let des_val: Annotation = serde_json::from_str(&s).unwrap();
        let des_str = serde_json::to_string(&des_val).unwrap();

        assert_eq!(s, des_str);
    }
}
