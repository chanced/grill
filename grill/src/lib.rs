// #![warn(missing_docs)]
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

mod error;
pub use error::*;
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

mod draft;
pub use draft::*;

mod vocabulary;
pub use vocabulary::*;

mod keyword;
pub use keyword::*;

pub mod dialect;
pub use dialect::Dialect;
