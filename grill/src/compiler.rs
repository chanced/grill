use std::collections::HashMap;

use once_cell::sync::OnceCell;
use serde_json::Number;

use crate::{
    schema::{Anchor, CompiledSubschema, Subschema},
    Location,
};

#[derive(Debug, Clone, Default)]
pub struct Compiler<'s> {
    location: Location,
    anchors: Vec<(String, Anchor<'s>)>,
    schemas: HashMap<&'s str, Subschema<'s>>,
    numbers: HashMap<&'s str, &'s Number>,
}

impl<'s> Compiler<'s> {
    pub fn anchor(&mut self, anchor: Anchor<'s>) {
        self.anchors
            .push((self.location.absolute_keyword_location.clone(), anchor));
    }
    pub fn schema(&mut self, keyword: &'s str, schema: Subschema<'s>) {
        self.schemas.insert(keyword, schema);
    }
    pub fn number(&mut self, keyword: &'s str, number: &'s Number) {
        self.numbers.insert(keyword, number);
    }
}
