use std::collections::HashMap;

use crate::dialect::Dialect;

/// A JSON Schema compiler and store.
pub struct Interrogator {
    dialects: HashMap<String, Dialect>,
}

impl Interrogator {
    /// Creates a new [`Interrogator`].
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new() -> Self {
        let mut this = Self {
            dialects: HashMap::new(),
        };
        todo!()
    }

    // pub fn add_post_processor(&mut self, )
}

impl Default for Interrogator {
    fn default() -> Self {
        Self::new()
    }
}
