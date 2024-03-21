#![doc = include_str!("../../README.md")]
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
    clippy::module_inception
)]
#![cfg_attr(test, allow(clippy::too_many_lines))]

pub mod cache;
pub mod criterion;
pub mod error;
/// Keywords
pub mod visitor;
pub use grill_uri as uri;

pub mod schema;
use once_cell::sync::Lazy;
pub use schema::{DefaultKey, Schema};

pub mod source;

pub mod big;

#[cfg(test)]
pub mod test;

use std::{
    any,
    borrow::Cow,
    collections::HashSet,
    fmt::Debug,
    ops::{ControlFlow, Deref},
};

use jsonptr::Pointer;
use serde_json::{Number, Value};

/// See [`slotmap`](`slotmap`) for more information.
pub use slotmap::{new_key_type, Key};

/// A static reference to [`Value::Bool`] of `true`
pub const TRUE: &Value = &Value::Bool(true);
/// A static reference to [`Value::Bool`] of `false`
pub const FALSE: &Value = &Value::Bool(false);

pub const EMPTY_OBJ: Lazy<Value> = Lazy::new(|| Value::Object(Default::default()));

pub(crate) trait ControlFlowExt<B, C>: Sized {
    /// Converts the `ControlFlow` into an `Option` which is `Some` if the
    /// `ControlFlow` was `Break` and `None` otherwise.
    ///
    /// Named `break_val` to avoid conflict with the `break` keyword and the
    /// nightly `break_value` method of `ControlFlow`.

    fn break_val(self) -> Option<B>;
    fn continue_val(self) -> Option<C>;
    fn map_continue<U, F: FnOnce(C) -> Option<U>>(self, f: F) -> Option<U>;
    fn map_break<U, F: FnOnce(B) -> Option<U>>(self, f: F) -> Option<U>;
    fn unwrap_break(self) -> B {
        self.break_val().unwrap()
    }
    fn unwrap_continue(self) -> C {
        self.continue_val().unwrap()
    }
}

impl<B, C> ControlFlowExt<B, C> for ControlFlow<B, C> {
    fn break_val(self) -> Option<B> {
        match self {
            ControlFlow::Break(b) => Some(b),
            _ => None,
        }
    }

    fn continue_val(self) -> Option<C> {
        match self {
            ControlFlow::Continue(c) => Some(c),
            _ => None,
        }
    }

    fn map_continue<U, F: FnOnce(C) -> Option<U>>(self, f: F) -> Option<U> {
        match self {
            ControlFlow::Continue(_) => todo!(),
            ControlFlow::Break(_) => todo!(),
        }
    }

    fn map_break<U, F: FnOnce(B) -> Option<U>>(self, f: F) -> Option<U> {
        todo!()
    }
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs::File;
//     use std::io::prelude::*;
//     #[tokio::test]
//     async fn test_build() {
//         let interrogator = Build::default()
//             .json_schema_2020_12()
//             .source_str("https://example.com/schema.json", r#"{"type": "string"}"#)
//             .unwrap()
//             .finish()
//             .await
//             .unwrap();

//         let mut file = File::create("foo.txt").unwrap();
//         file.write_all(format!("{interrogator:#?}").as_bytes())
//             .unwrap();
//     }
// }
