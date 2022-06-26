use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;

use crate::error::IndexError;
use crate::Schema;
use petgraph::algo::has_path_connecting;
use petgraph::graph::NodeIndex;
use petgraph::Graph as PetGraph;

/// Builds a graph of schemas whose edges correspond to references.
/// The goal is to determine compilation sequences and to recognize when
/// schemas are recursively referenced.
#[derive(Debug)]
pub(crate) struct Graph {
    index: HashMap<String, NodeIndex>,
    graph: PetGraph<String, ()>,
}

impl Graph {
    pub fn new(schemas: &[Arc<Schema>]) -> Result<Graph, IndexError> {
        let mut g = Graph {
            index: HashMap::new(),
            graph: PetGraph::new(),
        };
        for schema in schemas.iter().cloned() {
            g.add(schema)?;
        }
        Ok(g)
    }

    fn index(&mut self, id: impl ToString) -> NodeIndex {
        let Graph {
            ref mut index,
            ref mut graph,
        } = *self;

        let id = id.to_string();
        *index
            .entry(id.clone())
            .or_insert_with(|| graph.add_node(id))
    }

    pub fn add(&mut self, schema: Arc<Schema>) -> Result<(), IndexError> {
        if schema.id().is_none() {
            return Err(IndexError::NotIdentified);
        }
        let schema_index = self.index(schema.id().ok_or(IndexError::NotIdentified)?);
        for r in schema.references() {
            let ref_index = self.index(r.as_str());
            self.graph.add_edge(schema_index, ref_index, ());
        }

        Ok(())
    }

    /// Returns `true` if `reference` is referenced by `source`.
    pub fn is_referenced(
        &self,
        reference: impl Deref<Target = str>,
        source: impl Deref<Target = str>,
    ) -> bool {
        let outer = match self.index.get(reference.deref()) {
            Some(outer) => *outer,
            None => return false,
        };
        let inner = match self.index.get(source.deref()) {
            Some(inner) => *inner,
            None => return false,
        };

        has_path_connecting(&self.graph, outer, inner, None)
    }
}
