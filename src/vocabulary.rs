use std::collections::HashSet;

use uniresid::Uri;

use crate::Handler;
/// Defines a set of keywords and semantics that can be used to evaluate a
/// JSON Schema document.
#[derive(Clone, Debug, Default)]
pub struct Vocabulary {
    /// The URI of the vocabulary.
    pub id: Uri,
    /// Set of handlers for keywords defined by the vocabulary.
    pub handlers: HashSet<Box<dyn Handler>>,
}

impl Vocabulary {
    /// Returns a new [`Vocabulary`] with the given URI.
    #[must_use]
    pub fn new(&self, id: Uri) -> Self {
        Self {
            id,
            handlers: HashSet::default(),
        }
    }
    // pub fn add_keyword(&mut self, applicator: impl Applicator + 'static) {
    //     self.applicators.push(Box::new(applicator))
    // }
}

// impl Hash for Vocabulary {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.id.hash(state);
//     }
// }
