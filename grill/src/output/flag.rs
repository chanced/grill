use serde::{Deserialize, Serialize};

/// A concise [`Output`] [`Structure`] which only contains a single `"valid"` `bool` field.
///
/// [`Handler`]s should short circuit and return errors as soon as possible when using this
/// structure.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flag {
    valid: bool,
}
impl Flag {
    #[must_use]
    pub fn new(node: super::Node) -> Self {
        Self {
            valid: node.is_valid(),
        }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}
