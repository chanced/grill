use std::collections::HashMap;

use slotmap::{new_key_type, SlotMap};

use crate::{AbsoluteUri, Handler};

new_key_type! {
    pub struct SchemaKey;
}

#[derive(Clone, Debug)]
pub struct Schemas<Key: slotmap::Key = SchemaKey> {
    schemas: SlotMap<Key, Schema>,
    lookup: HashMap<AbsoluteUri, Key>,
    dep_graph: DependencyGraph,
}

impl<Key: slotmap::Key> Schemas<Key> {
    /// Creates a new [`Schemas<Key>`].
    #[must_use] pub fn new() -> Self {
        Self {
            schemas: SlotMap::default(),
            lookup: HashMap::default(),
            dep_graph: DependencyGraph::default(),
        }
    }

    #[must_use] pub fn get(&self, id: &AbsoluteUri) -> Option<(Key, &Schema)> {
        let key = self.lookup.get(id).copied()?;
        let schema = self.schemas.get(key)?;
        Some((key, schema))
    }
}

impl<Key: slotmap::Key> Default for Schemas<Key> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Schema {
    /// The URI of the schema.
    pub id: AbsoluteUri,
    /// The URI of the schema's `Metaschema`.
    pub meta_schema: AbsoluteUri,
    /// The Handlers associated with the schema.
    pub handlers: Box<[Handler]>,
}
impl PartialEq for Schema {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.meta_schema == other.meta_schema
    }
}
impl Eq for Schema {}

use petgraph::{prelude::NodeIndex, Directed, Graph as DirectedGraph};

#[derive(Debug, Clone)]
/// Contains a graph of schema references in order to detect cyclic
/// dependencies.
pub struct DependencyGraph {
    graph: DirectedGraph<String, String, Directed>,
    indexes: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    #[must_use] pub fn new() -> Self {
        Self {
            indexes: HashMap::new(),
            graph: DirectedGraph::new(),
        }
    }

    // pub fn reference(&mut self, keyword: Keyword, from: String, to: String) {
    //     let from = *self
    //         .indexes
    //         .entry(from.clone())
    //         .or_insert_with(|| self.ext_refs_graph.add_node(from));

    //     let to = *self
    //         .indexes
    //         .entry(to.clone())
    //         .or_insert_with(|| self.ext_refs_graph.add_node(to));
    //     self.ext_refs_graph.add_edge(from, to, keyword.to_string());
    // }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
