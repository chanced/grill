use num_rational::BigRational;
use serde_json::Number;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum BoolOrNumber {
    Bool(bool),
    Number(Number),
}

pub enum CompiledBoolOrNumber {
    Bool(bool),
    Number(BigRational),
}
