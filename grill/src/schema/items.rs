use serde::{Deserialize, Serialize};

use crate::Schema;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Items {
    Schema(Box<Schema>),
    Array(Vec<Schema>),
}
