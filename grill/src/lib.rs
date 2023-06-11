#![doc = include_str!("../../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![recursion_limit = "256"]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
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
    clippy::missing_errors_doc // TODO: remove when I get around to documenting
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]

#[allow(clippy::wildcard_imports)]
mod location;
pub use location::Location;

mod interrogator;

pub use interrogator::{Builder, Interrogator};

// pub use uniresid as uri;
// pub use uniresid::{AbsoluteUri, Uri};

/// A dialect represents the set of keywords and semantics that can be used to
/// evaluate a schema.
pub mod dialect;

mod scope;
pub use scope::Scope;

mod handler;

pub use handler::Handler;

pub mod error;


pub mod output;
pub use output::{Output, Structure};

/// Traits and implementations for loading JSON Schema source definitions.
pub mod resolve;
pub use resolve::Resolve;

mod compile;
pub use compile::Compile;

pub mod state;
pub use state::State;

// mod integration;
// pub use integration::Integration;

pub(crate) mod graph;

mod metaschema;
pub use metaschema::Metaschema;
pub mod uri;
pub use uri::{Uri, AbsoluteUri};

/// A JSON object.
/// 
/// Alias for [`serde_json::Map<String, serde_json::Value>`](`serde_json::Map`).
pub type Object = serde_json::Map<String, serde_json::Value>;
/// A JSON array.
/// 
/// Alias for `Vec<serde_json::Value>`.
pub type Array = Vec<serde_json::Value>;

pub mod json_schema;

pub use interrogator::SchemaKey;

pub mod deserialize;
pub use deserialize::{Deserializer};

#[cfg(test)]
pub mod test;
