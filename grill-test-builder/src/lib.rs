#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
// #![warn(missing_docs)]
#![allow(
    clippy::module_name_repetitions,
    clippy::result_large_err,
    clippy::enum_glob_use,
    clippy::implicit_hasher,
    clippy::needless_pass_by_value,
    clippy::similar_names,
    clippy::missing_panics_doc, // TODO: remove after todo!()s are removed
    clippy::missing_errors_doc, // TODO: remove when I get around to documenting
    clippy::wildcard_imports,
    clippy::module_inception,
    clippy::unreadable_literal
)]

use std::{collections::HashMap, path::PathBuf};

use grill::AbsoluteUri;
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod fs;
mod generate;

pub enum Out {
    Dir(Vec<Out>),
    File { path: PathBuf, content: String },
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq, Eq)]
pub struct Config {
    #[serde(flatten)]
    pub suite: HashMap<String, Suite>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Suite {
    /// the input path for the test suite - that is where the json files reside
    pub input: PathBuf,
    /// the output path for the test suite - that is where the generated files
    /// will be written
    pub output: PathBuf,
    /// glob patterns for json files which should be sourced, relative to
    /// `input`
    pub sources: Vec<PathBuf>,

    /// The base URI for the test suite
    #[serde(alias = "base_uri", rename = "base-uri")]
    pub base_uri: AbsoluteUri,
    /// a map of test cases to glob patterns for test files which should be
    /// executed
    ///
    /// a test file should be in the format of:
    /// ```json
    /// [
    ///     {
    ///         "description": "short description of test",
    ///         "schema": {
    ///             "$schema": "https://json-schema.org/draft/2020-12/schema",
    ///             "properties": {
    ///                 "foo": {"$ref": "#"}
    ///             },
    ///             "additionalProperties": false
    ///         },
    ///         "tests": [
    ///             {
    ///                 "description": "match",
    ///                 "data": {"foo": false},
    ///                 "valid": true
    ///             },
    ///             {
    ///                 "description": "recursive match",
    ///                 "data": {"foo": {"foo": false}},
    ///                 "valid": true
    ///             },
    ///             {
    ///                 "description": "mismatch",
    ///                 "data": {"bar": false},
    ///                 "valid": false
    ///             },
    ///             {
    ///                 "description": "recursive mismatch",
    ///                 "data": {"foo": {"bar": false}},
    ///                 "valid": false
    ///             }
    ///         ]
    ///     }
    /// ]
    /// ```
    #[serde(flatten)]
    pub tests: HashMap<PathBuf, HashMap<PathBuf, Vec<String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Case {
    pub description: String,
    pub schema: Value,
    pub tests: Vec<Test>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Test {
    pub description: String,
    pub data: serde_json::Value,
    pub valid: bool,
}

pub fn generate(cfg: Config) -> Result<String, anyhow::Error> {
    todo!()
    // Ok(String::default())
}

fn default_tests_dir() -> PathBuf {
    PathBuf::from("tests")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_cwd() {
        let mut cwd = std::env::current_dir().unwrap();
        if cwd.ends_with("grill-test-builder") {
            cwd.pop();
        }
        std::env::set_current_dir(cwd).unwrap();
    }

    #[test]
    fn test_spike() {
        set_cwd();
        let cfg = std::fs::read_to_string("./grill-test-builder/fixtures/tests.toml").unwrap();
        let cfg: Config = toml::from_str(&cfg).unwrap();
        println!("{cfg:?}");
        // let op = super::generate(cfg).unwrap();
        // println!("{op}");
    }
}
