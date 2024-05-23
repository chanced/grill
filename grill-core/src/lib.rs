//! # grill-core
//!

#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]
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
    clippy::module_inception
)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

use lang::{Language, Numbers, Schemas, Sources, Values};
use slotmap::Key;

pub mod big;
pub mod iter;
pub mod lang;

pub use lang::schema::DefaultKey;

pub struct Interrogator<L: Language<K>, K: Key = DefaultKey> {
    schemas: Schemas<L::CompiledSchema, K>,
    sources: Sources,
    values: Values,
    numbers: Numbers,
}

impl<L: Language<K>, K: Key> Interrogator<L, K> {
    pub fn new() -> Self {
        Self {
            schemas: Schemas::default(),
            sources: Sources::default(),
            values: Values::default(),
            numbers: Numbers::default(),
        }
    }
}
