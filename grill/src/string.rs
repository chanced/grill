use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
    String(String),
    Strings(Vec<String>),
}
