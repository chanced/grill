#[macro_use]
pub(crate) mod macros;

pub mod annotation;
pub use annotation::Annotation;

mod format;
pub use format::*;

mod error;
pub use error::*;

mod context;
pub use context::*;

mod interrogator;
pub use interrogator::Interrogator;

mod schema;
pub use schema::Schema;

mod applicator;
pub use applicator::*;

mod resolver;
pub use resolver::*;

mod extensions;
pub(crate) use extensions::Extensions;

mod graph;
pub(crate) use graph::Graph;

mod injectable;
pub use injectable::*;

mod implementation;
pub use implementation::*;

/// Temporary placeholder for a future implementation.
pub type Value = serde_json::Value;
/// Temporary placeholder for a future implementation.
pub type Number = serde_json::Number;

/// Temporary placeholder for a future implementation.
pub type Object<K, V> = serde_json::Map<K, V>;
