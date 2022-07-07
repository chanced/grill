use crate::Keyword;
use std::collections::HashMap;
use uniresid::Uri;

pub struct Vocabulary {
    pub id: Uri,
    pub keywords: HashMap<Uri, Keyword>,
}
