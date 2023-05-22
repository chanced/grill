use crate::Location;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt, mem};

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

/// An output node for a given keyword. Contains the keyword's location, sub
/// annotations and errors, possibly a [`ValidationError`] and any additional
/// fields pertinent to the keyword and output [`Structure`].
#[derive(Debug, Clone, Default)]
pub struct Node<'v> {
    /// Location of the keyword
    pub location: Location,
    /// Additional properties
    pub additional_props: HashMap<String, Value>,
    /// A validation error
    pub error: Option<Box<dyn ValidationError<'v>>>,
    annotations: Vec<Annotation<'v>>,
    errors: Vec<Annotation<'v>>,
}

impl<'v> Node<'v> {
    /// Returns `true` if there is an `error` or sub-annotations which are errors.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.error.is_none() && self.errors.is_empty()
    }
    /// Nested invalid `Annotation`s
    #[must_use]
    pub fn errors(&self) -> &[Annotation<'v>] {
        &self.errors
    }

    /// Nested valid `Annotation`s
    #[must_use]
    pub fn annotations(&self) -> &[Annotation<'v>] {
        &self.annotations
    }

    /// Adds a nested [`Annotation`]
    pub fn add(&mut self, detail: Node<'v>) {
        if detail.is_error() {
            self.errors.push(Annotation::Invalid(detail));
        } else {
            self.annotations.push(Annotation::Valid(detail));
        }
    }
}

impl<'n> Serialize for Node<'n> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Data<'x, 'n> {
            #[serde(flatten)]
            pub location: &'x Location,
            #[serde(flatten)]
            pub additional_props: &'x HashMap<String, Value>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: &'x Option<Box<dyn ValidationError<'n>>>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            annotations: &'x [Annotation<'n>],
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            errors: &'x [Annotation<'n>],
        }
        let data = Data {
            location: &self.location,
            additional_props: &self.additional_props,
            error: &self.error,
            annotations: &self.annotations,
            errors: &self.errors,
        };
        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Node<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Node<'static>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Data {
            #[serde(flatten)]
            pub location: Location,
            #[serde(flatten)]
            pub additional_props: HashMap<String, Value>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<String>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            annotations: Vec<Annotation<'static>>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            errors: Vec<Annotation<'static>>,
        }
        let Data {
            additional_props,
            annotations,
            errors,
            error,
            location,
        } = Data::deserialize(deserializer)?;

        Ok(Self {
            location,
            additional_props,
            error: error.map(|e| Box::new(e) as Box<dyn ValidationError<'static>>),
            annotations,
            errors,
        })
    }
}

/// Represents a valid or invalid (error) annotation
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Annotation<'v> {
    /// Valid annotation
    Valid(Node<'v>),
    /// Invalid annotation, meaning that the [`Detail`] either contains an [`ValidationError`] or
    /// has nested invalid annotations.
    Invalid(Node<'v>),
}

impl<'v> Annotation<'v> {
    #[must_use]
    pub fn new(location: Location, _value: &'v Value) -> Self {
        Self::Valid(Node {
            location,
            ..Default::default()
        })
    }
    pub fn error(&mut self, error: impl 'static + ValidationError<'v>) {
        let error = Some(Box::new(error) as Box<dyn ValidationError<'v>>);
        match self {
            Annotation::Valid(n) => {
                n.error = error;
                *self = Annotation::Invalid(mem::take(n));
            }
            Annotation::Invalid(n) => {
                n.error = error;
            }
        }
    }
    pub fn field(&mut self, field: String, value: Value) {
        match self {
            Annotation::Invalid(n) | Annotation::Valid(n) => {
                n.additional_props.insert(field, value);
            }
        }
    }

    /// Pushes a new nested annotation
    pub fn add(&mut self, annotation: Annotation<'v>) {
        match self {
            Annotation::Valid(detail) => match annotation {
                Annotation::Valid(_) => {
                    detail.annotations.push(annotation);
                }
                Annotation::Invalid(_) => {
                    detail.errors.push(annotation);
                    let detail = mem::take(detail);
                    *self = Annotation::Invalid(detail);
                }
            },
            Annotation::Invalid(detail) => match annotation {
                Annotation::Valid(_) => {
                    detail.annotations.push(annotation);
                }
                Annotation::Invalid(_) => {
                    detail.errors.push(annotation);
                }
            },
        }
    }

    /// Returns `true` if the annotation is [`Valid`].
    ///
    /// [`Valid`]: Annotation::Valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid(..))
    }

    /// Returns `true` if the annotation is [`Invalid`].
    ///
    /// [`Invalid`]: Annotation::Invalid
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(..))
    }

    /// Returns the [`Detail`] of this annotation.
    #[must_use]
    pub fn detail(&self) -> &Node<'v> {
        match self {
            Annotation::Invalid(detail) | Annotation::Valid(detail) => detail,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Annotation::Valid(v) => v.annotations.is_empty(),
            Annotation::Invalid(i) => i.annotations.is_empty(),
        }
    }
}

impl<'de> Deserialize<'de> for Annotation<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Annotation<'static>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let d: Node = Deserialize::deserialize(deserializer)?;
        if d.is_error() {
            Ok(Annotation::Valid(d))
        } else {
            Ok(Annotation::Invalid(d))
        }
    }
}

impl<'v> Default for Annotation<'v> {
    fn default() -> Self {
        Self::Valid(Node::default())
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use jsonptr::Pointer;

    use super::*;

    #[test]
    fn test_annotiation_serde() {
        let mut additional_props = HashMap::new();
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
