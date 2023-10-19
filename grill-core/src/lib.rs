#![doc = include_str!("../../README.md")]
#![cfg_attr(all(doc, CHANNEL_NIGHTLY), feature(doc_auto_cfg))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::all, clippy::pedantic)]
// TODO: enable this once I get to documenting
// #![deny(missing_docs)]
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

mod interrogator;
pub use interrogator::{Build, Interrogator};

pub mod error;

pub mod output;
pub use output::{Output, Structure};

pub mod keyword;

pub mod uri;
pub use uri::{AbsoluteUri, RelativeUri, Uri};

pub mod schema;
pub use schema::{Key, Schema};

pub mod source;
pub(crate) use source::Src;

pub mod big;

pub mod anymap;

#[cfg(test)]
pub mod test;

/// Returns a string literal of the supplied JSON.
///
/// # Example
/// ```rust
/// # use grill_macros::json_str;
/// let s = json_str!({"foo": "str", "bar": [34, 72]});
/// assert_eq!(s, r#"{"foo":"str","bar":[34,72]}"#);
/// ```
#[macro_export]
macro_rules! json_str {
    ($($json:tt)+) => {
        grill_macros::json_str($($json)+)
    };
}

/// Returns a pretty-printed string literal of the supplied JSON.
///
/// ```rust
/// # use grill_macros::json_pretty_str;
/// let s = json_pretty_str!({
///     "foo": "str",
///     "bar": [
///         34,
///         72
///     ]
/// });
/// assert_eq!(s, r#"{
///     "foo": "str",
///     "bar": [
///         34,
///         72
///     ]
/// }"#);
/// ```
#[macro_export]
macro_rules! json_pretty_str {
    ($($json:tt)+) => {
        grill_macros::json_pretty_str($($json)+)
    };
}
