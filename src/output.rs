use std::{borrow::Cow, fmt, mem};

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::Location;

/// A trait which represents a validation error to be used as the `"error"` field in output
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait Error: fmt::Display + fmt::Debug + DynClone + Send + Sync {}
dyn_clone::clone_trait_object!(Error);

impl Error for String {}

/// Detail contains the information about the keyword that was evaluated
/// and the result of the evaluation.
#[derive(Debug, Clone, Default)]
pub struct Detail {
    /// Location of the keyword
    pub location: Location,
    /// Additional properties
    pub additional_props: Map<String, Value>,
    /// A validation error
    pub error: Option<Box<dyn Error>>,
    annotations: Vec<Annotation>,
    errors: Vec<Annotation>,
}

impl Detail {
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

    /// Adds a nested Annotation
    pub fn add(&mut self, detail: Detail) {
        if detail.is_error() {
            self.errors.push(Annotation::Invalid(detail));
        } else {
            self.annotations.push(Annotation::Valid(detail));
        }
    }
}

impl From<SerializedDetail<'_>> for Detail {
    fn from(value: SerializedDetail) -> Self {
        let SerializedDetail {
            location,
            additional_props,
            error,
            annotations,
            errors,
        } = value;

        let error: Option<Box<dyn Error>> = if let Some(err) = error {
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

impl Serialize for Detail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: SerializedDetail = self.into();
        s.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Detail {
    fn deserialize<D>(deserializer: D) -> Result<Detail, D::Error>
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

impl<'a> From<&'a Detail> for SerializedDetail<'a> {
    fn from(v: &'a Detail) -> Self {
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
    Valid(Detail),
    /// Invalid annotation, meaning that the [`Detail`] either contains an [`Error`] or
    /// has nested invalid annotations.
    Invalid(Detail),
}

impl<'de> Deserialize<'de> for Annotation {
    fn deserialize<D>(deserializer: D) -> Result<Annotation, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let d: Detail = Deserialize::deserialize(deserializer)?;
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
    pub fn detail(&self) -> &Detail {
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

        let a = Annotation::Invalid(Detail {
            additional_props,
            location: Location {
                keyword_location: "/".try_into().unwrap(),
                absolute_location: "http://example.com".try_into().unwrap(),
                instance_location: "/".try_into().unwrap(),
                absolute_keyword_location: None,
            },
            error: None,
            annotations: vec![Annotation::Valid(Detail {
                annotations: vec![],
                errors: vec![],
                location: Location {
                    instance_location: Pointer::new(&["baddata"]),
                    keyword_location: Pointer::new(&["error-keyword"]),
                    absolute_location: "http://example.com#error-keyword".try_into().unwrap(),
                    ..Default::default()
                },
                error: Some(Box::new(String::from("bad data"))),
                ..Default::default()
            })],
            errors: vec![Annotation::Invalid(Detail {
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
