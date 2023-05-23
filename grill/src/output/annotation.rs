use std::mem;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Location;

use super::{Node, ValidationError};

/// Represents a valid or invalid (error) annotation
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Annotation<'v> {
    /// Valid annotation
    Valid(Node<'v>),
    /// Invalid annotaiton, meaning that the [`Detail`] either contains an [`ValidationError`] or
    /// has nested error annotations.
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

    /// Sets the error of this annotation
    pub fn error(&mut self, error: impl 'v + ValidationError<'v>) {
        let error = Some(Box::new(error) as Box<dyn 'v + ValidationError<'v>>);
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

    #[must_use]
    pub fn nested_errors(&self) -> &[Annotation<'v>] {
        match self {
            Annotation::Valid(n) | Annotation::Invalid(n) => &n.errors,
        }
    }

    #[must_use]
    pub fn annotations(&self) -> &[Annotation<'v>] {
        match self {
            Annotation::Valid(n) | Annotation::Invalid(n) => &n.annotations,
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
