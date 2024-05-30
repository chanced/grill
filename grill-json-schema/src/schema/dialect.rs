use std::{collections::HashMap, sync::Arc};

use grill_core::new_key_type;
use grill_uri::AbsoluteUri;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dialect<W> {
    pub uri: AbsoluteUri,
    pub keywords: Vec<W>,
    pub sources: HashMap<AbsoluteUri, Arc<Value>>,
}

new_key_type! {
    pub struct DialectKey;
}
