use serde_json::Value;

use crate::{Error, Evaluation, Interrogator, Next, Schema};

pub type ApplicatorFn =
    dyn 'static + Send + Sync + Fn(&Value, Evaluation, Next) -> Result<Evaluation, Error>;

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
