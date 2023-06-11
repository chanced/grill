use serde::Deserialize;
use serde_json::Value;

pub type TestCases = Vec<TestCase>;

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub description: String,
    pub schema: Value,
    pub tests: Vec<Test>,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}