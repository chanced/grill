//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

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
    clippy::module_inception,
    clippy::unreadable_literal
)]
#![cfg_attr(test, allow(clippy::redundant_clone, clippy::too_many_lines))]
#![recursion_limit = "256"]

use serde_json::Value;

pub mod keyword;
pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;



/// A trait for adding JSON Schema dialects to a [`Build`](grill_core::Build).
pub trait Build: Sized {
    /// Adds the JSON Schema Draft 2020-12 [`Dialect`](grill_core::schema::Dialect).
    #[must_use]
    fn json_schema_2020_12(self) -> grill_core::Build;

    /// Adds the JSON Schema Draft 2019-09 [`Dialect`](grill_core::schema::Dialect).
    #[must_use]
    fn json_schema_2019_09(self) -> grill_core::Build;

    /// Adds the JSON Schema Draft 07 [`Dialect`](grill_core::schema::Dialect).
    #[must_use]
    fn json_schema_07(self) -> grill_core::Build;

    /// Adds the JSON Schema Draft 04 [`Dialect`](grill_core::schema::Dialect).
    #[must_use]
    fn json_schema_04(self) -> grill_core::Build;
}

impl Build for grill_core::Build {
    fn json_schema_2020_12(self) -> grill_core::Build {
        self.dialect(draft_2020_12::dialect())
    }

    fn json_schema_2019_09(self) -> grill_core::Build {
        self.dialect(draft_2019_09::dialect())
    }

    fn json_schema_07(self) -> grill_core::Build {
        self.dialect(draft_07::dialect())
    }

    fn json_schema_04(self) -> grill_core::Build {
        self.dialect(draft_04::dialect())
    }
}

/// Generates two `fn`s: one which returns the static
/// [`AbsoluteUri`](grill::uri::AbsoluteUri) and another that returns the static
/// `Value`
/// 
/// # Example
/// ```
/// # use grill_macros::metaschema;
/// metaschema!(
///     [JSON Schema 2020_12 Content]("https://json-schema.org/draft/2020-12/meta/content")
///     {
///         "$schema": "https://json-schema.org/draft/2020-12/schema",
///         "$id": "https://json-schema.org/draft/2020-12/meta/content",
///         "$vocabulary": {
///             "https://json-schema.org/draft/2020-12/vocab/content": true
///         },
///         "$dynamicAnchor": "meta",
/// 
///         "title": "Content vocabulary meta-schema",
/// 
///         "type": ["object", "boolean"],
///         "properties": {
///             "contentEncoding": { "type": "string" },
///             "contentMediaType": { "type": "string" },
///             "contentSchema": { "$dynamicRef": "#meta" }
///         }
///     }
/// );
/// assert_eq!(json_schema_2020_12_content_uri().as_str(), "https://json-schema.org/draft/2020-12/meta/content");
/// assert_eq!(json_schema_2020_12_content_value().get("$id").unwrap(), &serde_json::Value::String("https://json-schema.org/draft/2020-12/meta/content".to_string()));
/// ```
#[macro_export]
macro_rules! metaschema {
    (
        $(#[$meta:meta])*
        [$($name:tt)+]($uri:literal)
        $($json:tt)+
    ) => {
        paste::paste!{
            #[doc = "Returns the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) for " $($name " ")+]
            #[doc = ""]
            #[doc = "<" $uri ">"]
            #[must_use]
            pub fn [<$($name:lower _)* uri>]() -> &'static ::grill_core::uri::AbsoluteUri {
                use ::once_cell::sync::Lazy;
                use grill_core::uri::AbsoluteUri;
                static URI: Lazy<AbsoluteUri> = Lazy::new(|| AbsoluteUri::parse($uri).unwrap());
                &URI
            }
            $(#[$meta])*
            #[must_use]
            #[doc = "Returns the [`Value`](`::serde_json::Value`) for " $($name " ")+]
            #[doc = "```json"]
            #[doc=grill_macros::json_pretty_str!($($json)+)]
            #[doc = "```"]
            pub fn [<$($name:lower _)* value>]() -> &'static ::serde_json::Value {
                use ::once_cell::sync::Lazy;
                use ::serde_json::{json, Value};
                static VALUE: Lazy<Value> = Lazy::new(|| json!($($json)*));
                &VALUE
            }
        }
    };
}
