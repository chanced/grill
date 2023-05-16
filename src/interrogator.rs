/// A JSON Schema compiler. It manages state, handlers, and schema data as well as compiled schemas.
pub struct Interrogator {}

impl Interrogator {
    /// Creates a new [`Interrogator`].
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }

    // pub fn add_post_processor(&mut self, )
}

impl Default for Interrogator {
    fn default() -> Self {
        Self::new()
    }
}
