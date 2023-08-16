use jsonptr::Pointer;

use crate::AbsoluteUri;

use super::Keyword;

pub struct Reference {
    pub keyword: Keyword<'static>,
    pub path: Pointer,
    pub reference: AbsoluteUri,
}
