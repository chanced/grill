//! # grill-core
//!

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![deny(clippy::all, clippy::pedantic)]
// #![warn(missing_docs)]
#![allow(
    // this needs to be done on a per-fn basis
    clippy::must_use_candidate,
    clippy::implicit_hasher,
    clippy::wildcard_imports,
    clippy::module_name_repetitions
)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

pub use interrogator::Interrogator;
pub use lang::Language;
pub use resolve::Resolve;
pub use schema::DefaultKey;
pub use slotmap::{new_key_type, Key};

mod interrogator;

pub mod big;
pub mod cache;
pub mod iter;
pub mod lang;
pub mod resolve;
pub mod schema;
pub mod source;
pub mod state;
