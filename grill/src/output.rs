//! Output formats, annotations, and errors
//!
// pub mod annotation;
mod node;

// pub use annotation::Annotation;
pub use node::Node;

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

use crate::{AbsoluteUri, Uri};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Structure {
    /// A concise [`Output`] [`Structure`] which only contains a single
    /// `"valid"` `bool` field.
    ///
    /// This `Structure` may have a positive impact on
    /// performance as [`Handler`]s are expected to short circuit and return errors as
    /// soon as possible.
    ///
    /// # Example
    /// ```json
    /// { "valid": false }
    /// ```
    ///
    /// - [JSON Schema Core 2020-12 # 12.4.1
    ///   `Flag`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-flag)
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
pub trait ValidationError<'v>: fmt::Display + fmt::Debug + DynClone + Send + Sync {
    fn into_owned(self) -> Box<dyn ValidationError<'static>>;
}

dyn_clone::clone_trait_object!(<'v> ValidationError<'v>);

impl Serialize for dyn ValidationError<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl ValidationError<'_> for String {
    fn into_owned(self) -> Box<dyn ValidationError<'static>> {
        Box::new(self)
    }
}

/// A concise [`Output`] [`Structure`] which only contains a single `"valid"` `bool` field.
///
/// [`Handler`]s should short circuit and return errors as soon as possible when using this
/// structure.
#[derive(Debug)]
pub struct Flag(pub bool);
impl Flag {
    #[must_use]
    pub fn new(node: Node) -> Self {
        Self(node.is_valid())
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.0
    }
}

#[derive(Debug)]
pub struct Basic<'v> {
    nodes: Vec<Node<'v>>,
}
impl<'v> Basic<'v> {
    #[must_use]
    pub fn new(node: Node<'v>) -> Self {
        todo!()
    }
    pub fn is_valid(&self) -> bool {
        todo!()
    }
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}

#[derive(Debug)]
pub struct Detailed<'v>(Node<'v>);

impl<'v> Detailed<'v> {
    #[must_use]
    pub fn new(node: Node<'v>) -> Self {
        Self(node)
    }
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

#[derive(Debug)]
pub struct Verbose<'v>(Node<'v>);
impl<'v> Verbose<'v> {
    #[must_use]
    pub fn new(node: Node<'v>) -> Self {
        Self(node)
    }
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

#[derive(Debug)]
pub struct Complete<'v>(pub Node<'v>);
impl<'v> Complete<'v> {
    #[must_use]
    pub fn new(annotation: Node<'v>) -> Self {
        Self(annotation)
    }
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

#[derive(Debug)]
pub enum Output<'v> {
    Flag(Flag),
    Basic(Basic<'v>),
    Detailed(Detailed<'v>),
    Verbose(Verbose<'v>),
    Complete(Complete<'v>),
}

impl<'v> Output<'v> {
    #[must_use]
    pub fn new(structure: Structure, node: Node<'v>) -> Output {
        match structure {
            Structure::Flag => Flag::new(node).into(),
            Structure::Basic => Basic::new(node).into(),
            Structure::Detailed => Detailed::new(node).into(),
            Structure::Verbose => Verbose::new(node).into(),
            Structure::Complete => Complete(node).into(),
        }
    }
    pub fn structure(&self) -> Structure {
        match self {
            Output::Flag(_) => Structure::Flag,
            Output::Basic(_) => Structure::Basic,
            Output::Detailed(_) => Structure::Detailed,
            Output::Verbose(_) => Structure::Verbose,
            Output::Complete(_) => Structure::Complete,
        }
    }
    pub fn absolute_keyword_location(&self) -> Option<&Uri> {
        match self {
            Output::Flag(flag) => None,
            Output::Basic(basic) => todo!(),
            Output::Detailed(detailed) => todo!(),
            Output::Verbose(verbose) => todo!(),
            Output::Complete(complete) => todo!(),
        }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match self {
            Output::Flag(flag) => flag.is_valid(),
            Output::Basic(basic) => basic.is_valid(),
            Output::Detailed(detailed) => detailed.is_valid(),
            Output::Verbose(verbose) => verbose.is_valid(),
            Output::Complete(complete) => complete.is_valid(),
        }
    }
}

impl From<Flag> for Output<'_> {
    fn from(flag: Flag) -> Self {
        Self::Flag(flag)
    }
}
impl From<Basic<'_>> for Output<'_> {
    fn from(basic: Basic<'_>) -> Self {
        Self::Basic(basic)
    }
}

impl From<Detailed<'_>> for Output<'_> {
    fn from(detailed: Detailed<'_>) -> Self {
        Self::Detailed(detailed)
    }
}
impl From<Verbose<'_>> for Output<'_> {
    fn from(verbose: Verbose<'_>) -> Self {
        Self::Verbose(verbose)
    }
}

impl From<Complete<'_>> for Output<'_> {
    fn from(complete: Complete<'_>) -> Self {
        Self::Complete(complete)
    }
}

impl Display for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, collections::BTreeMap};

    use jsonptr::Pointer;

    use crate::Location;

    use super::*;

    // #[test]
    // fn test_annotiation_serde() {
    //     let mut additional_props = BTreeMap::new();
    //     additional_props.insert("example".into(), 34.into());

    //     let a = Annotation::Invalid(Node {
    //         additional_props,
    //         location: Location {
    //             keyword_location: "/".try_into().unwrap(),
    //             instance_location: "/".try_into().unwrap(),
    //             absolute_keyword_location: "http://example.com".to_string(),
    //         },
    //         error: None,
    //         valid: vec![Annotation::Valid(Node {
    //             valid: vec![],
    //             invalid: vec![],
    //             location: Location {
    //                 instance_location: Pointer::new(["bad-data"]),
    //                 keyword_location: Pointer::new(["error-keyword"]),
    //                 ..Default::default()
    //             },
    //             error: Some(Box::new(String::from("bad data"))),
    //             ..Default::default()
    //         })],
    //         invalid: vec![Annotation::Invalid(Node {
    //             valid: vec![],
    //             invalid: vec![],
    //             error: Some(Box::new(String::from("nested error"))),
    //             location: Location {
    //                 absolute_keyword_location: "http://example.com".to_string(),
    //                 ..Default::default()
    //             },

    //             ..Default::default()
    //         })],
    //     });

    //     let s = serde_json::to_string(&a).unwrap();
    //     let des_val: Annotation = serde_json::from_str(&s).unwrap();
    //     let des_str = serde_json::to_string(&des_val).unwrap();

    //     assert_eq!(s, des_str);
    // }
}
