#[derive(Debug)]
pub struct Verbose<'v> {
    node: super::Node<'v>,
}
impl<'v> Verbose<'v> {
    #[must_use]
    pub fn new(node: super::Node<'v>) -> Self {
        Self { node }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.node.is_valid()
    }
}
