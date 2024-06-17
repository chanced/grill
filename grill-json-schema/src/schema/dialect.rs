use std::{borrow::Cow, collections::HashMap, fmt::Debug, sync::Arc};

use grill_core::new_key_type;
use grill_uri::{AbsoluteUri, Uri};
use serde_json::Value;
use snafu::Snafu;

use crate::keyword::invalid_type::{Actual, Expectated, InvalidTypeError};

new_key_type! {
    pub struct DialectKey;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dialect<W> {
    /// Primary [`AbsoluteUri`] of the dialect (e.g. `"https://json-schema.org/draft/2020-12/schema")
    pub uri: AbsoluteUri,
    /// All possible keywords of this `Dialect`
    pub keywords: Vec<W>,
    /// Metaschema sources of this `Dialect`
    pub sources: HashMap<AbsoluteUri, Arc<Value>>,
}

/// Trait implemented types which can identify a schema. This
pub trait Identify: Clone + Debug {
    /// Identifies a schema, (e.g.`$id` for JSON Schema 07 and later or `id` for
    /// JSON Schema 04 and earlier)
    fn identify<'v>(&self, schema: &'v Value) -> Result<Option<Cow<'v, str>>, InvalidTypeError>;
}

#[derive(Clone, Debug)]
pub struct IdentifyModern;
impl Identify for IdentifyModern {
    fn identify<'v>(&self, schema: &'v Value) -> Option<Value> {
        let Some(id) = schema.get(crate::keyword::ID) else {
            return Ok(None);
        };
        let id = id
            .as_str()
            .ok_or_else(|| InvalidTypeError::new(schema.clone(), Expectated::String))?;
        Ok(Some(Cow::Borrowed(id)))
    }
}

/// Identifies current JSON Schemas by reading and parsing the `$id` keyword.
pub struct IdentifyLegacy;

pub trait IsApplicable: Clone + Debug {
    fn is_applicable(schema: &Value, uri: &AbsoluteUri) -> bool;
}

#[derive(Debug, Snafu)]
pub enum IdentifyError {}
