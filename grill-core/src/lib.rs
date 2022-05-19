#[macro_use]
pub(crate) mod macros;

pub mod error;
pub use error::Error;

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

pub mod annotations;
pub use annotations::*;

mod extensions;
mod graph;

pub mod eval;
pub use eval::*;

mod injectable;
pub use injectable::*;

/// A temporary placeholder for a future implementation.
pub type Value = serde_json::Value;
