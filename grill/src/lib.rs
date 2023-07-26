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
    clippy::wildcard_imports
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]

mod anchor;
pub use anchor::Anchor;

pub mod location;
pub use location::Location;

mod interrogator;
pub use interrogator::{Builder, Interrogator};

pub use slotmap::new_key_type;

pub mod dialect;

mod handler;
pub use handler::{Scope, Handler};

pub mod error;

pub mod output;
pub use output::{Output, Structure};

pub mod resolve;
pub use resolve::Resolve;

mod compile;
pub use compile::Compile;

mod metaschema;
pub use metaschema::Metaschema;
pub mod uri;
pub use uri::{Uri, AbsoluteUri};

pub mod json_schema;

pub mod deserialize;
pub use deserialize::Deserializer;

pub mod schema;
pub use schema::{Schema, SchemaKey};

/// A JSON object.
/// 
/// Alias for [`serde_json::Map<String, serde_json::Value>`](`serde_json::Map`).
pub type Object = serde_json::Map<String, serde_json::Value>;
/// A JSON array.
/// 
/// Alias for `Vec<serde_json::Value>`.
pub type Array = Vec<serde_json::Value>;

new_key_type! {
    pub struct ValueKey;
}

pub mod source;
pub use source::Source;

#[cfg(test)]
pub mod test;


// mod integration;
// pub use integration::Integration;
