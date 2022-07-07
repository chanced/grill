use crate::Applicator;

pub struct Keyword {
    pub field: String,
    pub applicator: Box<dyn Applicator>,
}

impl Keyword {
    /// Creates a new Keyword
    pub fn new(field: String, applicator: impl 'static + Applicator) -> Self {
        Self {
            field,
            applicator: Box::new(applicator),
        }
    }
}
