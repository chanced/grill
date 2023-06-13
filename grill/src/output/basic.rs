use serde::Serialize;

use super::Node;

#[derive(Debug, Serialize, serde::Deserialize)]
pub struct Basic<'v> {
    nodes: Vec<Node<'v>>,
}
impl<'v> Basic<'v> {
    #[must_use]
    pub fn new(node: Node<'v>) -> Self {
        if node.is_error() {}
    }
    pub fn is_valid(&self) -> bool {
        todo!()
    }
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }
}
