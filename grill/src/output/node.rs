use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Location;

use super::{Annotation, ValidationError};

/// An output node for a given keyword. Contains the keyword's location, sub
/// annotations and errors, possibly a [`ValidationError`] and any additional
/// fields pertinent to the keyword and output [`Structure`](`crate::output::Structure`).
#[derive(Debug, Clone, Default)]
pub struct Node<'v> {
    /// Location of the keyword
    pub location: Location,
    /// Additional properties
    pub additional_props: BTreeMap<String, Value>,
    /// A validation error
    pub error: Option<Box<dyn 'v + ValidationError<'v>>>,
    pub annotations: Vec<Annotation<'v>>,
    pub errors: Vec<Annotation<'v>>,
}

impl<'v> Node<'v> {
    /// Returns `true` if there is an `error` or sub-annotations which are errors.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.error.is_none() && self.errors.is_empty()
    }

    /// Inserts a nested [`Annotation`]
    pub fn insert(&mut self, detail: Node<'v>) {
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
            pub additional_props: &'x BTreeMap<String, Value>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<String>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            annotations: &'x [Annotation<'n>],
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            errors: &'x [Annotation<'n>],
        }
        let data = Data {
            location: &self.location,
            additional_props: &self.additional_props,
            error: self.error.as_ref().map(ToString::to_string),
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
            pub additional_props: BTreeMap<String, Value>,
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
