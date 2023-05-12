#![recursion_limit = "256"]

mod location;
pub use location::Location;

mod interrogator;

pub use interrogator::Interrogator;

pub use uniresid as uri;
pub use uniresid::Uri;

pub mod dialect;
pub mod draft;

pub mod schema;
pub use schema::Schema;

mod scope;
pub use scope::Scope;

mod vocabulary;
pub use vocabulary::Vocabulary;

mod handler;

pub mod errors;
pub use errors::SetupError;

mod types;
pub use types::{Type, Types};

pub mod annotation;
pub use annotation::{Annotation, Detail, Error};

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
