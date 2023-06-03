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
    clippy::needless_pass_by_value
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]

#[allow(clippy::wildcard_imports)]
mod location;
pub use location::Location;

mod interrogator;

pub use interrogator::Interrogator;

// pub use uniresid as uri;
// pub use uniresid::{AbsoluteUri, Uri};

/// A dialect represents the set of keywords and semantics that can be used to
/// evaluate a schema.
pub mod dialect;

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
pub use output::{Output, Structure};

/// Traits and implementations for loading JSON Schema source definitions.
pub mod resolve;
pub use resolve::Resolve;

mod compile;
pub use compile::Compile;

pub(crate) const ISSUES_URL: &str = "https://github.com/chanced/grill/issues/new";

pub mod state;
pub use state::State;

mod integration;

mod keyword;
pub use keyword::Keyword;

pub(crate) mod graph;

mod uri;
pub use uri::{AbsoluteUri, Uri};

#[cfg(test)]
pub mod test;
