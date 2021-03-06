use std::collections::HashMap;

use crate::Vocabulary;
/// A composition of [`Vocabulary`].
pub struct Dialect {
    /// The [`Vocabulary`]s in this `Dialect` mapped to a `bool` indicating
    /// whether they are required.
    pub vocabularies: HashMap<Vocabulary, bool>,
}

impl Dialect {
    pub fn new(vocabularies: HashMap<Vocabulary, bool>) -> Self {
        Dialect { vocabularies }
    }
}
