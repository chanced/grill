#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum BoolOrNumber {
    Bool(bool),
    Number(serde_json::Number),
}
