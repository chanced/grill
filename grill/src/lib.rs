#![recursion_limit = "256"]
// #![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![doc = include_str!("../../README.md")]
/// Contains data structures pertaining to the response of the method `evaluate`
/// of a [`Schema`](crate::Schema).
///
/// See Output Formatting in the JSON Schema Specification for more information:
/// - [2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting)
/// - [2019-09](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.10)
pub mod evaluation;
pub use evaluation::Evaluation;

mod output_fmt;
pub use output_fmt::OutputFmt;

pub mod error;
pub use error::Error;
/// Contains data structures pertaining to
pub mod interrogator;
pub use interrogator::Interrogator;
/// Contains data structures pertaining to JSON Schemas, including
/// [crate::Schema] and [crate::schema::Builder]
pub mod schema;
pub use schema::{MetaSchema, Schema, SubSchema};

pub mod applicator;
pub use applicator::Applicator;

mod resolver;
pub use resolver::*;

mod next;
pub use next::Next;

mod graph;
pub(crate) use graph::Graph;

pub use uniresid as uri;

pub use uri::Uri;

pub use jsonptr;
pub use jsonptr::Pointer;

pub mod draft;
pub use draft::{
    create_hyper_schema_04, create_hyper_schema_07, create_hyper_schema_2019_09,
    create_hyper_schema_2020_12, create_schema_04, create_schema_07, create_schema_2019_09,
    create_schema_2020_12,
};
mod vocabulary;
pub use vocabulary::*;

pub mod dialect;
pub use dialect::Dialect;
