use crate::{
    schema::{Anchor, Subschema},
    Location,
};

#[derive(Debug, Clone)]
pub struct Compiler<'s> {
    location: Location,
    // TODO:       ↓ these could probably be &str
    schemas: Vec<(String, Subschema<'s>)>,
    //             ↓
    anchors: Vec<(String, Anchor<'s>)>,
}
impl<'s> Compiler<'s> {
    pub fn anchor(&mut self, anchor: Anchor<'s>) {
        let (base, _) = self
            .location
            .absolute_keyword_location
            .split_once('#')
            .unwrap_or((&self.location.absolute_keyword_location, ""));
        self.anchors.push((base.to_string(), anchor));
    }
}
