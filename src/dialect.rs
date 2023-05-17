use std::collections::HashMap;

use uniresid::Uri;

use crate::Handler;
/// Defines a set of keywords and semantics that can be used to evaluate a
/// JSON Schema document.
#[derive(Clone, Debug, Default)]
pub struct Vocabulary {
    /// The URI of the vocabulary.
    pub id: Uri,
    /// Set of handlers for keywords defined by the vocabulary.
    pub handlers: Vec<Box<dyn Handler>>,
}

impl Vocabulary {
    // pub fn add_keyword(&mut self, applicator: impl Applicator + 'static) {
    //     self.applicators.push(Box::new(applicator))
    // }
}

/// A composition of [`Vocabulary`].
pub struct Dialect {
    pub id: Uri,
    /// The [`Vocabulary`]s in this `Dialect` mapped to a `bool` indicating
    /// whether they are required.
    pub vocabularies: HashMap<Uri, Vocabulary>,
}
