use serde::Deserialize;

use grill::Schema;

pub type TestCases = Vec<TestCase>;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub description: String,
    pub schema: Schema,
    pub tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}
