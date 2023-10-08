//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

pub mod common;
pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;

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
            pub fn [<$($name:lower _)* uri>]() -> &'static crate::uri::AbsoluteUri {
                use ::once_cell::sync::Lazy;
                use crate::uri::AbsoluteUri;
                static URI: Lazy<AbsoluteUri> = Lazy::new(|| AbsoluteUri::parse($uri).unwrap());
                &URI
            }
            $(#[$meta])*
            #[must_use]
            #[doc = "Returns the [`Value`](`::serde_json::Value`) for " $($name " ")+]
            #[doc = "```json\r"]
            #[doc=grill_macros::json_pretty_str!($($json)+)]
            #[doc = "```\r"]
            pub fn [<$($name:lower _)* value>]() -> &'static ::serde_json::Value {
                use ::once_cell::sync::Lazy;
                use ::serde_json::{json, Value};
                static VALUE: Lazy<Value> = Lazy::new(|| json!($($json)*));
                &VALUE
            }
        }
    };
}
use metaschema;

#[cfg(test)]
mod tests {}
