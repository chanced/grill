use crate::Applicator;
use uniresid::Uri;

pub struct Vocabulary {
    pub id: Uri,
    pub applicators: Vec<Box<dyn Applicator>>,
    is_known: bool,
}

impl Vocabulary {
    pub fn new(&self, id: Uri) -> Self {
        Vocabulary {
            id,
            applicators: Vec::default(),
            is_known: false,
        }
    }
    pub fn push(&mut self, applicator: impl Applicator + 'static) {
        self.applicators.push(Box::new(applicator))
    }
}

impl std::hash::Hash for Vocabulary {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
