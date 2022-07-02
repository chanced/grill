use std::sync::Arc;

use crate::{Error, Interrogator, Schema};

#[derive(Clone)]
pub enum SubSchema {
    Single(Schema),
    Array(Arc<Vec<Schema>>),
}

impl SubSchema {
    pub fn is_single(&self) -> bool {
        matches!(self, SubSchema::Single(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, SubSchema::Array(_))
    }
    pub(crate) fn setup(&self, interrogator: &Interrogator) -> Result<(), Error> {
        match self {
            SubSchema::Single(schema) => schema.setup(interrogator),
            SubSchema::Array(schemas) => {
                for schema in schemas.iter() {
                    schema.setup(interrogator)?;
                }
                Ok(())
            }
        }
    }
}
