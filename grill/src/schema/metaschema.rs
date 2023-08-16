use crate::{AbsoluteUri, Object};

#[derive(Clone, Debug)]
pub struct Metaschema {
    pub id: AbsoluteUri,
    pub schema: Object,
}

impl Metaschema {
    #[must_use]
    pub fn new(id: AbsoluteUri, schema: Object) -> Self {
        Self { id, schema }
    }
}
