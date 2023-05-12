use uniresid::Uri;

pub struct Vocabulary {
    pub id: Uri,
    // pub keywords: Vec<Box<dyn Keyword>>,
}

impl Vocabulary {
    pub fn new(&self, id: Uri) -> Self {
        Self { id }
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
