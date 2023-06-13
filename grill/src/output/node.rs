use jsonptr::Pointer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, error::Error, fmt::Display};

use crate::{Location, Uri};

use super::ValidationError;

/// An output node for a given keyword. Contains the keyword's location, sub
/// annotations and errors, possibly a [`ValidationError`] and any additional
/// fields pertinent to the keyword and output [`Structure`](`crate::output::Structure`).
#[derive(Default, Debug, Clone)]
pub struct Node<'v> {
    /// Location of the keyword
    location: Location,
    /// Additional properties
    additional_properties: BTreeMap<String, Value>,
    /// A validation error
    error: Option<Box<dyn 'v + ValidationError<'v>>>,
    valid: Vec<Node<'v>>,
    invalid: Vec<Node<'v>>,
}

impl<'v> Node<'v> {

    pub fn is_valid(&self) -> bool {
        return self.is_annotation()
    }

    #[must_use]
    pub fn is_annotation(&self) -> bool {
        self.error.is_none() && self.invalid.is_empty()
    }
    /// Returns `true` if there is an `error` or sub-nodes which are errors.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.error.is_none() && self.invalid.is_empty()
    }
    pub fn additional_properties(&self) -> &BTreeMap<String, Value> {
        &self.additional_properties
    }
    pub fn absolute_keyword_location(&self) -> &Uri {
        &self.location.absolute_keyword_location()
    }
    pub fn keyword_location(&self) -> &Pointer {
        self.location.keyword_location()
    }
    pub fn instance_location(&self) -> &Pointer {
        self.location.instance_location()
    }
    pub fn location(&self) -> &Location {
        &self.location
    }
    pub fn error(&self) -> Option<&dyn ValidationError<'v>> {
        self.error.as_deref()
    }

    pub fn errors(&self) -> &[Node<'v>] {
        &self.invalid
    }
    pub fn annotations(&self) -> &[Node<'v>] {
        &self.valid
    }
    /// Inserts a nested [`Annotation`]
    pub fn insert(&mut self, detail: Node<'v>) {
        if detail.is_error() {
            self.invalid.push(detail);
        } else {
            self.valid.push(detail);
        }
    }
}
impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(err) = &self.error {
            write!(f, "{}", err)
        } else {
            write!(
                f,
                "{} passed evaluation",
                self.location.absolute_keyword_location()
            )
        }
    }
}

impl Error for Node<'_> {}

impl<'n> Serialize for Node<'n> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Data<'x, 'v> {
            #[serde(flatten)]
            pub location: &'x Location,
            #[serde(flatten)]
            pub additional_props: &'x BTreeMap<String, Value>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<String>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            annotations: &'x [Node<'v>],
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            errors: &'x [Node<'v>],
        }
        let data = Data {
            location: &self.location,
            additional_props: &self.additional_properties,
            error: self.error.as_ref().map(ToString::to_string),
            annotations: &self.valid,
            errors: &self.invalid,
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
            annotations: Vec<Node<'static>>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            errors: Vec<Node<'static>>,
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
            additional_properties: additional_props,
            error: error.map(|e| Box::new(e) as Box<dyn ValidationError<'static>>),
            valid: annotations,
            invalid: errors,
        })
    }
}
