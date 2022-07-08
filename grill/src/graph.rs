use std::collections::HashMap;

use crate::{error::UnidentifiedSchemaError, Schema};
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
    nodes: HashMap<NodeIndex, Schema>,
}

impl Graph {
    pub fn new(schemas: &[Schema]) -> Result<Graph, UnidentifiedSchemaError> {
        let mut g = Graph {
            index: HashMap::new(),
            graph: PetGraph::new(),
            nodes: HashMap::new(),
        };
        for schema in schemas.iter().cloned() {
            g.add(schema)?;
        }
        Ok(g)
    }

    pub fn rebuild(&mut self, schemas: &[Schema]) -> Result<(), UnidentifiedSchemaError> {
        self.index.clear();
        self.graph.clear();
        self.nodes.clear();
        for schema in schemas.iter().cloned() {
            self.add(schema)?;
        }
        Ok(())
    }

    fn index(&mut self, id: &Uri) -> NodeIndex {
        let Graph {
            ref mut index,
            ref mut graph,
            nodes: _,
        } = *self;
        let id = id.clone();
        *index
            .entry(id.clone())
            .or_insert_with(|| graph.add_node(id))
    }

    pub fn add(&mut self, schema: Schema) -> Result<(), UnidentifiedSchemaError> {
        let schema_index = self.index(&schema.id().as_deref().cloned().ok_or(
            UnidentifiedSchemaError {
                schema: schema.clone(),
            },
        )?);

        for r in schema.references().iter() {
            let ref_index = self.index(r);
            self.graph.add_edge(schema_index, ref_index, ());
        }
        self.nodes.insert(schema_index, schema);
        Ok(())
    }

    /// Returns `true` if `reference` is referenced by `source`.
    pub fn is_referenced(&self, reference: &Schema, source: &Schema) -> bool {
        let rid = reference.id();
        let sid = source.id();

        if rid.is_none() || sid.is_none() {
            return false;
        }
        let rid = rid.unwrap();
        let sid = sid.unwrap();

        let outer = match self.index.get(&rid) {
            Some(outer) => *outer,
            None => return false,
        };
        let inner = match self.index.get(&sid) {
            Some(inner) => *inner,
            None => return false,
        };

        has_path_connecting(&self.graph, outer, inner, None)
    }

    pub fn nodes(&self) -> Nodes {
        Nodes::new(
            petgraph::algo::kosaraju_scc(&self.graph),
            self.nodes.clone(),
        )
    }
}

pub struct Nodes {
    indexes: Vec<Vec<NodeIndex>>,
    nodes: HashMap<NodeIndex, Schema>,
    index: (usize, usize),
}
impl Nodes {
    fn new(indexes: Vec<Vec<NodeIndex>>, nodes: HashMap<NodeIndex, Schema>) -> Self {
        Self {
            indexes,
            nodes,
            index: (0, 0),
        }
    }
}
impl Iterator for Nodes {
    type Item = Schema;
    fn next(&mut self) -> Option<Self::Item> {
        let (mut i, mut x) = self.index;
        let idx = loop {
            if i >= self.indexes.len() {
                return None;
            }
            if x >= self.indexes[i].len() {
                x = 0;
                i += 1;
                continue;
            }
            break self.indexes[i][0];
        };
        self.index = (i, x);
        self.nodes.get(&idx).cloned()
    }
}
