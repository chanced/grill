use std::borrow::Cow;

use uniresid::Uri;

use crate::Schema;

pub enum Subschema<'a> {
    Reference(&'a Uri),
    Inline(Cow<'a, Schema>),
}

pub struct CompiledSubschema {}
