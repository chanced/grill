//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.
#![recursion_limit = "256"]
mod consts;
pub use consts::*;

pub mod common;
pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;

pub trait Build {
    /// Returns a new [`Build`] with the JSON Schema Draft 2020-12 [`Dialect`] that is
    /// set as the default dialect.
    #[must_use]
    #[allow(unused_must_use)]
    fn json_schema_2020_12() -> grill_core::Build {
        Build::default().default_dialect(crate::draft_2020_12::dialect())
    }
}
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
