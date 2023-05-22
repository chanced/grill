mod structure;

pub use structure::Structure;

use crate::Location;
use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::{borrow::Cow, fmt, mem};

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
    pub additional_props: Map<String, Value>,
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
            pub additional_props: &'x Map<String, Value>,
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
            pub additional_props: Map<String, Value>,
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
impl<'v> Annotation<'v> {
    #[must_use]
    pub fn new(location: Location) -> Self {
        Self::Valid(Node {
            location,
            ..Default::default()
        })
    }

    /// Adds a nested annotation
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

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use jsonptr::Pointer;

    use super::*;

    #[test]
    fn test_annotiation_serde() {
        let mut additional_props = Map::new();
        additional_props.insert("example".into(), 34.into());

        let a = Annotation::Invalid(Node {
            additional_props,
            location: Location {
                keyword_location: "/".try_into().unwrap(),
                instance_location: "/".try_into().unwrap(),
                absolute_keyword_location: None,
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
