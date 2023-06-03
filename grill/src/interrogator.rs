use std::collections::HashMap;

use serde_json::Value;

use crate::{dialect::Dialect, graph::DependencyGraph, AbsoluteUri};

/// A JSON Schema compiler and store.
pub struct Interrogator {
    dialects: HashMap<AbsoluteUri, Dialect>,
    graph: DependencyGraph,
    base_uri: Option<String>,
    sources: HashMap<String, Value>,
}

impl Interrogator {}

impl Default for Interrogator {
    fn default() -> Self {
        todo!()
    }
}
/// Constructs an [`Interrogator`].
pub struct InterrogatorBuilder {
    dialects: HashMap<AbsoluteUri, Dialect>,
    base_uri: Option<AbsoluteUri>,
    sources: HashMap<AbsoluteUri, Value>,
}
impl Default for InterrogatorBuilder {
    fn default() -> Self {
        // Self::new()
        todo!()
    }
}
