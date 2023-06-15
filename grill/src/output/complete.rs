#[derive(Debug)]
pub struct Complete<'v> {
    node: super::Node<'v>,
}
impl<'v> Complete<'v> {
    #[must_use]
    pub fn new(node: super::Node<'v>) -> Self {
        Self { node }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.node.is_valid()
    }
}
