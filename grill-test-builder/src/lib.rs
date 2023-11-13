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

mod fs;
mod generate;

pub use fs::write;

use camino::{Utf8Path, Utf8PathBuf};
use glob::GlobError;
use grill::AbsoluteUri;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snafu::{ResultExt, Snafu};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    #[serde(default = "default_output", rename = "tests-dir", alias = "tests_dir")]
    pub tests_dir: Utf8PathBuf,
    #[serde(flatten)]
    pub suite: HashMap<Utf8PathBuf, Suite>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sources {
    #[serde(rename = "strip-prefix", alias = "strip_prefix", default)]
    pub strip_prefix: Option<Utf8PathBuf>,
    pub paths: Vec<Utf8PathBuf>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Suite {
    /// glob patterns for json files which should be sourced, relative to
    /// `input`
    #[serde(default)]
    pub sources: Option<Sources>,

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
    pub tests: HashMap<Utf8PathBuf, HashMap<Utf8PathBuf, Vec<Utf8PathBuf>>>,
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

pub fn generate(_cwd: PathBuf, cfg: &Config) -> Result<Vec<(Utf8PathBuf, String, bool)>, Error> {
    for (path, suite) in &cfg.suite {
        let suite = generate::gen_suite(&[&cfg.tests_dir, path], suite)?;
    }
    todo!()
}

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display(
        "failed to read or access file\"{}\"\ncaused by:\n\n{}\n", 
        path.display(),
        source
    ))]
    Io {
        pattern: Option<Utf8PathBuf>,
        path: PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("path is not valid utf8: {path:?}\n"))]
    NotUtf8 { path: PathBuf },

    #[snafu(display("failed to parse glob pattern \"{pattern}\"\ncaused by:\n\n{source}\n"))]
    Glob {
        pattern: Utf8PathBuf,
        source: glob::PatternError,
    },

    #[snafu(display("failed to load config{path:?}:\ncaused by:\n\n{source}\n"))]
    Toml {
        path: PathBuf,
        source: toml::de::Error,
    },

    #[snafu(display("failed to load test case {path:?}:\ncaused by:\n\n{source}\n"))]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },

    #[snafu(display("failed to parse token stream:\n{content}\n\ncaused by:\n\n{source}\n"))]
    Syn { content: String, source: syn::Error },

    #[snafu(display("failed to format source:\ncaused by:\n\n{stdout}"))]
    RustFmt { stdout: String, stderr: String },
}

impl Error {
    #[must_use]
    pub fn from_glob_error(err: GlobError, pattern: Utf8PathBuf) -> Self {
        let path = err.path().to_owned();
        Error::Io {
            pattern: Some(pattern.clone()),
            source: err.into_error(),
            path,
        }
    }
}
pub fn load_cfg(path: impl AsRef<str>) -> Result<Config, Error> {
    let path = Utf8Path::new(path.as_ref());
    let cfg = std::fs::read_to_string(path).with_context(|_| IoSnafu {
        path: path.to_owned(),
        pattern: None,
    })?;
    toml::from_str(&cfg).with_context(|_| TomlSnafu {
        path: path.to_owned(),
    })
}

fn default_output() -> Utf8PathBuf {
    Utf8PathBuf::from("tests")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn set_test_cwd() {
        let mut cwd = std::env::current_dir().unwrap();
        if cwd.ends_with("grill-test-builder") {
            cwd.pop();
        }
        std::env::set_current_dir(cwd).unwrap();
    }

    #[test]
    fn test_spike() {
        set_test_cwd();
        let cfg = load_cfg(Utf8Path::new("grill-test-builder/fixtures/tests.toml"));
        println!("{cfg:?}");
        // let op = super::generate::suite(path, suite);
        // println!("{op}");
    }
}
