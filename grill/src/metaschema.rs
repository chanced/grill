

use crate::{uri::AbsoluteUri, Object};

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
