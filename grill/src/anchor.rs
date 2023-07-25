use serde_json::Value;

use crate::schema::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor<'v> {
    /// Value of the anchor.  
    pub value: &'v str,
    /// The containing `Value`
    pub container: &'v Value,
    /// The keyword of the anchor
    pub keyword: Keyword<'v>,
}
