use crate::{Error, Interrogator, Schema};

/// `Initializer`s are run when a `Schema` is created or reloaded. The primary
/// purpose and internal usage is to identify the schema and any references to
/// other `Schema`s it may have.
///
/// By seperating `Initializer`s from `Applicator`s, we can ensure that the
/// `Interrogrator`'s graph of `Schema`s is fully populated before any
/// `Applicator`s are run.
pub trait Initializer {
    fn call(&self, interrogator: Interrogator, schema: Schema) -> Result<(), Error>;
}

impl<F> Initializer for F
where
    F: Fn(Interrogator, Schema) -> Result<(), Error>,
{
    fn call(&self, interrogator: Interrogator, schema: Schema) -> Result<(), Error> {
        self(interrogator, schema)
    }
}
