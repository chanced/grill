#[macro_use]
pub(crate) mod macros;

pub mod error;
pub use error::*;

pub mod context;
pub use context::*;

pub mod interrogator;
pub use interrogator::Interrogator;

pub mod schema;
pub use schema::Schema;

pub mod applicator;
pub use applicator::*;

pub mod resolver;
pub use resolver::*;

pub mod eval;
pub use eval::*;

mod next;
pub use next::Next;

mod extensions;
pub(crate) use extensions::Extensions;
mod graph;
pub(crate) use graph::Graph;
mod injectable;
pub use injectable::*;

/// A temporary placeholder for a future implementation.
pub type Value = serde_json::Value;
