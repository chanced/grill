use std::collections::HashMap;

use crate::Keyword;
use petgraph::{prelude::NodeIndex, Directed, Graph as DirectedGraph};

#[derive(Debug, Clone)]
/// DependencyGraph contains a graph of dependencies referenced v`ref`)
/// by schemas. It is to detect cyclic sc
pub(crate) struct DependencyGraph {
    ext_refs_graph: DirectedGraph<String, String, Directed>,
    indexes: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            indexes: HashMap::new(),
            ext_refs_graph: DirectedGraph::new(),
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
