use serde_json::Value;

use crate::{Annotation, Error, Interrogator, Next, Schema};

pub type ApplicatorFn =
    dyn 'static + Send + Sync + Fn(&Value, Annotation, Next) -> Result<Annotation, Error>;

pub trait Applicator {
    fn setup(
        &self,
        interrogator: Interrogator,
        schema: Schema,
    ) -> Result<Option<Box<ApplicatorFn>>, Error>;
}

#[cfg(test)]
mod test {
    #[derive(Clone)]
    struct TestImpl {}
}
