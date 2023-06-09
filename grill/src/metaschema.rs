use crate::{uri::AbsoluteUri, Object, Uri};

pub struct Metaschema {
    pub id: AbsoluteUri,
    pub schema: Object,
}
