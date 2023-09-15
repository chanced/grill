use super::ValidationError;
use crate::{keyword::Location, Object, Uri};

use jsonptr::Pointer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fmt::Display};

/// An output node for a given keyword. Contains the keyword's location, sub
/// annotations and errors, possibly a [`ValidationError`] and any additional
/// fields pertinent to the keyword and output [`Structure`](`crate::output::Structure`).
#[derive(Default, Debug, Clone)]
pub struct Node<'v> {
    /// Location of the keyword
    location: Location,
    /// Additional properties
    additional_props: Object,
    /// Validation error
    error: Option<Box<dyn 'static + ValidationError<'v>>>,
    /// Sub annotations
    annotations: Vec<Node<'v>>,
    /// Sub errors
    errors: Vec<Node<'v>>,
}

impl<'v> Node<'v> {
    pub fn new(location: Location, err: Option<impl 'static + ValidationError<'v>>) -> Node<'v> {
        Self {
            location,
            error: err.map(|e| Box::new(e) as Box<dyn 'static + ValidationError<'v>>),
            ..Default::default()
        }
    }
    #[must_use]
    pub fn absolute_keyword_location(&self) -> &Uri {
        self.location.absolute_keyword_location()
    }

    #[must_use]
    pub fn keyword_location(&self) -> &Pointer {
        self.location.keyword_location()
    }

    #[must_use]
    pub fn instance_locaiton(&self) -> &Pointer {
        self.location.instance_location()
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.is_annotation()
    }
    #[must_use]
    pub fn into_owned_box(&self) -> Node<'static> {
        Node {
            location: self.location.clone(),
            additional_props: self.additional_props.clone(),
            error: self.error.clone().map(ValidationError::into_owned_box),
            annotations: self.annotations.iter().map(Node::into_owned_box).collect(),
            errors: self.errors.iter().map(Node::into_owned_box).collect(),
        }
    }
    #[must_use]
    pub fn is_annotation(&self) -> bool {
        self.error.is_none() && self.errors.is_empty()
    }
    /// Returns `true` if there is an `error` or sub-nodes which are errors.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.error.is_none() && self.errors.is_empty()
    }
    #[must_use]
    pub fn additional_props(&self) -> &Object {
        &self.additional_props
    }

    pub fn additional_props_mut(&mut self) -> &mut Object {
        &mut self.additional_props
    }

    pub fn insert_additional_prop(&mut self, key: &str, value: Value) {
        self.additional_props.insert(key.to_string(), value);
    }
    pub fn remove_additional_prop(&mut self, key: &str) {
        self.additional_props.remove(key);
    }

    #[must_use]
    pub fn get_additional_prop(&self, key: &str) -> Option<&Value> {
        self.additional_props.get(key)
    }

    #[must_use]
    pub fn error(&self) -> Option<&dyn ValidationError<'v>> {
        self.error.as_deref()
    }

    pub fn set_error(&mut self, error: impl 'static + ValidationError<'v>) {
        self.error = Some(Box::new(error) as Box<dyn 'static + ValidationError<'v>>);
    }

    #[must_use]
    pub fn errors(&self) -> &[Node<'v>] {
        &self.errors
    }

    #[must_use]
    pub fn annotations(&self) -> &[Node<'v>] {
        &self.annotations
    }
    /// Inserts a nested [`Annotation`]
    pub fn insert(&mut self, detail: Node<'v>) {
        if detail.is_error() {
            self.errors.push(detail);
        } else {
            self.annotations.push(detail);
        }
    }
}

impl Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(err) = &self.error {
            write!(f, "{err}")
        } else {
            write!(f, "{} passed evaluation", self.absolute_keyword_location()) // TODO ABSOLUTE KEYWORD LOCATION
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
            pub additional_props: &'x Object,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub error: Option<String>,
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            annotations: &'x [Node<'v>],
            #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
            errors: &'x [Node<'v>],
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
            pub additional_props: Object,
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
            additional_props,
            error: error.map(|e| Box::new(e) as Box<dyn ValidationError<'static>>),
            annotations,
            errors,
        })
    }
}
