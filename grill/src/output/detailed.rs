use super::Node;

#[derive(Debug)]
pub struct Detailed<'v> {
    node: super::Node<'v>,
}

impl<'v> Detailed<'v> {
    #[must_use]
    pub fn new(node: Node<'v>) -> Self {
        Self { node }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.node.is_valid()
    }
}
