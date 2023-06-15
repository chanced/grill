use std::{borrow::Cow, collections::BTreeMap};

use jsonptr::Pointer;
use serde::Serialize;
use serde_json::Value;

use crate::Location;

use super::ValidationError;

#[derive(Debug, Clone)]
pub struct Basic<'v> {
    nodes: Nodes<'v>,
}
impl<'v> Basic<'v> {
    #[must_use]
    pub fn new(node: super::Node<'v>) -> Self {
        todo!()
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        matches!(self.nodes, Nodes::Annotations(_))
    }
}

impl<'v> Serialize for Basic<'v> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
/// A [`Basic`] output node for a given keyword. Contains the keyword's location, sub
/// annotations and errors, possibly a [`ValidationError`] and any additional
/// fields pertinent to the keyword.
#[derive(Debug, Clone)]
pub struct Node<'v> {
    /// Location of the keyword
    location: Location,
    /// Additional properties
    additional_properties: BTreeMap<String, Value>,
    /// Validation error
    error: Option<Box<dyn ValidationError<'v>>>,
}
impl<'v> Node<'v> {
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }
}
impl TryFrom<Unit<'_>> for Node<'static> {
    type Error = String;

    fn try_from(value: Unit) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
enum Nodes<'v> {
    Errors(Vec<Node<'v>>),
    Annotations(Vec<Node<'v>>),
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Unit<'a> {
    #[serde(rename = "keywordLocation")]
    keyword_location: Cow<'a, Pointer>,
    #[serde(rename = "absoluteKeywordLocation")]
    absolute_keyword_location: Cow<'a, str>,
    #[serde(rename = "instanceLocation")]
    instance_location: Cow<'a, Pointer>,
    valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Cow<'a, String>>,
    #[serde(flatten)]
    additional_properties: Cow<'a, BTreeMap<String, Value>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum Units<'a> {
    Errors(Vec<Unit<'a>>),
    Annotations(Vec<Unit<'a>>),
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Data<'a> {
    valid: bool,
    #[serde(flatten)]
    units: Units<'a>,
}
