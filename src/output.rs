use std::{borrow::Cow, fmt, mem};

use crate::Location;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// A trait which represents a validation error to be used as the `"error"` field in output
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait ValidationError: fmt::Display + fmt::Debug + DynClone + Send + Sync {}
dyn_clone::clone_trait_object!(ValidationError);

impl ValidationError for String {}

pub struct Flag {}

pub struct Basic {}

pub struct Detailed {}

pub struct Verbose {}

pub struct Complete(pub Annotation);

pub enum Output {
    Flag(Flag),
    Basic(Basic),
    Detailed(Detailed),
    Verbose(Verbose),
    Complete(Complete),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Structure {
    Flag,
    /// The "Basic" structure is a flat list of output units.
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
    /// The "Detailed" structure is based on the schema and can be more readable for both humans and machines. Having the structure organized this way makes associations between the errors more apparent. For example, the fact that the missing "y" property and the extra "z" property both stem from the same location in the instance is not immediately obvious in the "Basic" structure. In a hierarchy, the correlation is more easily identified.
    ///
    /// The following rules govern the construction of the results object:
    ///
    /// - All applicator keywords (`"*Of"`, `"$ref"`, `"if"`/`"then"`/`"else"`, etc.) require a node.
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
    /// ## `Detailed` Output:
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
    /// ```
    Detailed,
    Verbose,
    Complete,
}

/// An output unit for a given keyword. Contains the keyword's location, sub
/// annotations and errors, possibly a [`ValidationError`] and any additional fields pertinent to the
/// keyword and output [`Structure`].
#[derive(Debug, Clone, Default)]
pub struct Unit {
    /// Location of the keyword
    pub location: Location,
    /// Additional properties
    pub additional_props: Map<String, Value>,
    /// A validation error
    pub error: Option<Box<dyn ValidationError>>,
    annotations: Vec<Annotation>,
    errors: Vec<Annotation>,
}

impl Unit {
    /// Returns `true` if there is an `error` or sub-annotations which are errors.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.error.is_none() && self.errors.is_empty()
    }
    /// Nested invalid `Annotation`s
    #[must_use]
    pub fn errors(&self) -> &[Annotation] {
        &self.errors
    }

    /// Nested valid `Annotation`s
    #[must_use]
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }

    /// Adds a nested [`Annotation`]
    pub fn add_nested(&mut self, detail: Unit) {
        if detail.is_error() {
            self.errors.push(Annotation::Invalid(detail));
        } else {
            self.annotations.push(Annotation::Valid(detail));
        }
    }
}

impl From<SerializedDetail<'_>> for Unit {
    fn from(value: SerializedDetail) -> Self {
        let SerializedDetail {
            location,
            additional_props,
            error,
            annotations,
            errors,
        } = value;

        let error: Option<Box<dyn ValidationError>> = if let Some(err) = error {
            Some(Box::new(err))
        } else {
            None
        };

        Self {
            location: location.into_owned(),
            additional_props: additional_props.into_owned(),
            error,
            annotations: annotations.into_owned(),
            errors: errors.into_owned(),
        }
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: SerializedDetail = self.into();
        s.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Unit, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: SerializedDetail = Deserialize::deserialize(deserializer)?;
        Ok(s.into())
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedDetail<'a> {
    #[serde(flatten)]
    pub location: Cow<'a, Location>,
    #[serde(flatten)]
    pub additional_props: Cow<'a, Map<String, Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    annotations: Cow<'a, [Annotation]>,
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    errors: Cow<'a, [Annotation]>,
}

impl<'a> From<&'a Unit> for SerializedDetail<'a> {
    fn from(v: &'a Unit) -> Self {
        Self {
            location: Cow::Borrowed(&v.location),
            additional_props: Cow::Borrowed(&v.additional_props),
            error: v.error.as_ref().map(std::string::ToString::to_string),
            annotations: (&v.annotations).into(),
            errors: (&v.errors).into(),
        }
    }
}

/// Represents a valid or invalid (error) annotation
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Annotation {
    /// Valid annotation
    Valid(Unit),
    /// Invalid annotation, meaning that the [`Detail`] either contains an [`ValidationError`] or
    /// has nested invalid annotations.
    Invalid(Unit),
}

impl<'de> Deserialize<'de> for Annotation {
    fn deserialize<D>(deserializer: D) -> Result<Annotation, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let d: Unit = Deserialize::deserialize(deserializer)?;
        if d.is_error() {
            Ok(Annotation::Valid(d))
        } else {
            Ok(Annotation::Invalid(d))
        }
    }
}

impl Annotation {
    /// Adds a nested annotation
    pub fn add(&mut self, annotation: Annotation) {
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
    pub fn detail(&self) -> &Unit {
        match self {
            Annotation::Invalid(detail) | Annotation::Valid(detail) => detail,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use jsonptr::Pointer;

    use super::*;

    #[test]
    fn test_annotiation_serde() {
        let mut additional_props = Map::new();
        additional_props.insert("example".into(), 34.into());

        let a = Annotation::Invalid(Unit {
            additional_props,
            location: Location {
                keyword_location: "/".try_into().unwrap(),
                instance_location: "/".try_into().unwrap(),
                absolute_keyword_location: None,
            },
            error: None,
            annotations: vec![Annotation::Valid(Unit {
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
            errors: vec![Annotation::Invalid(Unit {
                annotations: vec![],
                errors: vec![],
                error: Some(Box::new(String::from("nested error"))),
                location: Location {
                    absolute_keyword_location: Some("http://example.com".try_into().unwrap()),
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
