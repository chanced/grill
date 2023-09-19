use serde_json::Value;

use crate::AbsoluteUri;

#[derive(Clone, Debug)]
pub struct Metaschema {
    pub id: AbsoluteUri,
    pub schema: Value,
}
