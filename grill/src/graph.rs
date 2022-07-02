use std::collections::HashMap;

use crate::{Schema, UnidentifiedSchemaError};
use petgraph::algo::has_path_connecting;
use petgraph::graph::NodeIndex;
use petgraph::Graph as PetGraph;
use uniresid::Uri;

/// Builds a graph of schemas whose edges correspond to references.
/// The goal is to determine compilation sequences and to recognize when
/// schemas are recursively referenced.
#[derive(Debug)]
pub(crate) struct Graph {
    index: HashMap<Uri, NodeIndex>,
    graph: PetGraph<Uri, ()>,
}

impl Graph {
    pub fn new(schemas: &[Schema]) -> Result<Graph, UnidentifiedSchemaError> {
        let mut g = Graph {
            index: HashMap::new(),
            graph: PetGraph::new(),
        };
        for schema in schemas.iter().cloned() {
            g.add(schema)?;
        }
        Ok(g)
    }

    fn index(&mut self, id: &Uri) -> NodeIndex {
        let Graph {
            ref mut index,
            ref mut graph,
        } = *self;
        let id = id.clone();
        *index
            .entry(id.clone())
            .or_insert_with(|| graph.add_node(id))
    }

    pub fn add(&mut self, schema: Schema) -> Result<(), UnidentifiedSchemaError> {
        if schema.id().is_none() {
            return Err(UnidentifiedSchemaError { schema });
        }
        let schema_index = self.index(&schema.id().as_deref().cloned().ok_or(
            UnidentifiedSchemaError {
                schema: schema.clone(),
            },
        )?);

        for r in schema.references().iter() {
            let ref_index = self.index(r);
            self.graph.add_edge(schema_index, ref_index, ());
        }
        Ok(())
    }

    /// Returns `true` if `reference` is referenced by `source`.
    pub fn is_referenced(&self, reference: &Uri, source: &Uri) -> bool {
        let outer = match self.index.get(reference) {
            Some(outer) => *outer,
            None => return false,
        };
        let inner = match self.index.get(source) {
            Some(inner) => *inner,
            None => return false,
        };

        has_path_connecting(&self.graph, outer, inner, None)
    }
}
