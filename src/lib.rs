#![doc = include_str!("../README.md")]
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
    clippy::implicit_hasher
)]

#[allow(clippy::wildcard_imports)]
mod location;
pub use location::Location;

mod interrogator;

pub use interrogator::Interrogator;

pub use uniresid as uri;
pub use uniresid::Uri;
/// A dialect represents the set of keywords and semantics that can be used to
/// evaluate a schema.
pub mod dialect;

/// Meta schemas and their respective [`Uri`]s for each JSON Schema draft release.
pub mod draft;

mod source;
pub use source::Source;

/// JSON Schema and supporting types.
pub mod schema;
/// A JSON Schema.
pub use schema::Schema;

mod scope;
pub use scope::Scope;

mod handler;

pub use handler::Handler;

/// Logical errors that can occur during setup, compilation, and evaluation of a
/// schema.
///
/// Validation errors are defined within their respective keyword's module.
pub mod error;

/// Output formats, annotations, and errors
pub mod output;

/// Traits and implementations for loading JSON Schema source definitions.
pub mod resolve;
pub use resolve::Resolve;

pub(crate) fn value_type_name(value: &serde_json::Value) -> &'static str {
    use serde_json::Value::*;
    match value {
        Null => "null",
        Bool(_) => "boolean",
        Number(_) => "number",
        String(_) => "string",
        Array(_) => "array",
        Object(_) => "object",
    }
}
