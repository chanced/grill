use std::fs;

use grill::{AbsoluteUri, Interrogator, Key};

use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct TestCases {
    pub filename: String,
    test_cases: Vec<TestCase>,
}
impl TestCases {
    async fn setup_interrogator(&mut self, interrogator: &mut Interrogator) {
        for (i, case) in self.test_cases.iter_mut().enumerate() {
            println!("compiling {i} of {}", self.filename);
            let uri = format!("https://example.com/test-suit/{}_{i}", self.filename);
            interrogator
                .source_owned_value(&uri, case.schema.clone())
                .map_err(|err| {
                    println!("{err}");
                    err
                })
                .unwrap();
            case.schema_key = interrogator.compile(&uri).await.unwrap();
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TestCase {
    pub description: String,
    pub schema: Value,
    pub tests: Vec<Test>,
    #[serde(skip)]
    pub schema_key: Key,
}

#[derive(Debug, Deserialize)]
pub struct Test {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}

pub async fn load(interrogator: &mut Interrogator, draft: &str) -> Vec<TestCases> {
    let mut cases = load_cases(draft);
    for case in &mut cases {
        case.setup_interrogator(interrogator).await;
    }
    cases
}

fn load_cases(draft: &str) -> Vec<TestCases> {
    let path = format!("json-schema-test-suite/tests/{}", draft);
    let mut entries = fs::read_dir(path.clone())
        .unwrap_or_else(|_| panic!("failed to read test cases directory {path}"))
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap_or_else(|_| panic!("failed to load test cases for {path}"));
    entries.sort();

    let mut test_cases_set = Vec::new();
    for path in entries {
        if fs::metadata(&path)
            .unwrap_or_else(|_| panic!("failed to load metadata of {path:?}"))
            .is_file()
        {
            let file = fs::File::open(&path).unwrap();
            let test_cases: Vec<TestCase> = serde_json::from_reader(file).unwrap();
            let filename = path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .trim_end_matches(".json")
                .to_string();
            test_cases_set.push(TestCases {
                filename,
                test_cases,
            });
        }
    }
    test_cases_set
}
