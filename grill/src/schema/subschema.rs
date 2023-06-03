use std::borrow::Cow;

use crate::Schema;

use super::CompiledSchema;

#[derive(Debug, Clone)]
pub enum Subschema<'s> {
    Inline(Cow<'s, Schema>),
    Reference(&'s str),
}

// #[derive(Debug, Clone)]
// pub struct SubschemaRef {
//     keyword: String,
//     schema: CompiledSchema,
// }

// impl SubschemaRef {
//     #[must_use]
//     pub fn absolute_location(&self) -> &str {
//         &self.schema.absolute_location()
//     }
// }
