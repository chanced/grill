use std::{ops::ControlFlow, sync::Arc};

use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dialect<W, E> {
    pub uri: AbsoluteUri,
    pub keywords: Vec<W>,
    pub sources: Vec<(AbsoluteUri, Arc<Value>)>,
    pub embedded_schema_paths: Vec<E>,
}

/// A result from `anchor` of [`Keyword`]
pub struct FoundAnchor<'v> {
    /// path of the anchor
    pub path: Pointer,
    /// anchor value
    pub anchor: &'v str,
    /// keyword of the anchor
    pub keyword: &'static str,
}
