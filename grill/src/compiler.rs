use once_cell::sync::OnceCell;

use crate::{
    schema::{Anchor, CompiledSubschema, Subschema},
    Location,
};

#[derive(Debug, Clone, Default)]
pub struct Compiler<'s> {
    location: Location,
    schemas: Vec<(String, Subschema<'s>)>,
    anchors: Vec<(String, Anchor<'s>)>,
}

impl<'s> Compiler<'s> {
    pub fn anchor(&mut self, anchor: Anchor<'s>) {
        self.anchors
            .push((self.location.absolute_keyword_location.clone(), anchor));
    }
    pub fn schema(&mut self, keyword: &str, schema: Subschema<'s>) -> CompiledSubschema {
        self.schemas
            .push((self.location.absolute_keyword_location.clone(), schema));
        CompiledSubschema {
            keyword_location: self.location.keyword_location.clone(),
            schema: OnceCell::default(),
        }
    }
}
