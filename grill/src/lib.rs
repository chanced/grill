#![doc = include_str!("../../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![recursion_limit = "256"]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
// TODO: enable this once I get to documenting
// #![deny(missing_docs)]
#![allow(
    clippy::module_name_repetitions,
    clippy::result_large_err,
    clippy::large_enum_variant,
    clippy::enum_glob_use,
    clippy::implicit_hasher,
    clippy::needless_pass_by_value,
    clippy::similar_names, 
    clippy::missing_panics_doc, // TODO: remove after todo!()s are removed
    clippy::missing_errors_doc, // TODO: remove when I get around to documenting
    clippy::wildcard_imports,
    clippy::module_inception
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]



mod interrogator;
pub use interrogator::{Interrogator, Builder};

pub use slotmap::new_key_type;

pub mod error;

pub mod output;
pub use output::{Output, Structure};

pub mod uri;
pub use uri::{Uri, AbsoluteUri, RelativeUri};

pub mod schema;
pub use schema::{Schema, SchemaKey};

pub mod handler;

pub mod json_schema;



/// A JSON object.
/// 
/// Alias for [`serde_json::Map<String, serde_json::Value>`](`serde_json::Map`).
pub type Object = serde_json::Map<String, serde_json::Value>;
/// A JSON array.
/// 
/// Alias for `Vec<serde_json::Value>`.
pub type Array = Vec<serde_json::Value>;


pub mod source;
pub(crate) use source::Source;


#[cfg(test)]
pub mod test;

pub(crate) mod number;

// mod integration;
// pub use integration::Integration;
