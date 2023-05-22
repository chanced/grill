use crate::schema::Subschema;

#[derive(Debug, Clone)]
pub struct Compiler<'s> {
    location: jsonptr::Pointer,
    schemas: Vec<(String, &'s Subschema<'s>)>,
}
