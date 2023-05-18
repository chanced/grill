use std::collections::VecDeque;

use crate::schema::Subschema;

#[derive(Debug, Clone)]
pub struct Compiler<'s> {
    compilation_queue: VecDeque<Subschema<'s>>,
}
