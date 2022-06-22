use std::sync::Arc;

use serde_json::Value;

use crate::{Annotation, Interrogator, Result, Schema};

pub struct Next {}

pub type ApplicatorFn =
    dyn 'static + Send + Sync + Fn(&serde_json::Value, Next) -> Result<Annotation>;

impl Next {
    pub fn call(&self, value: Value) -> Result<Annotation> {
        todo!()
    }
}
pub trait Applicator {
    fn setup(&self, interrogator: Interrogator, schema: Schema) -> Arc<ApplicatorFn>;
}

fn x(a: impl Applicator) {
    let i: Interrogator = Interrogator::new();
    let s = Schema::new(Value::Null);
    let x = a.setup(i, s);
}

impl<F> Applicator for F
where
    F: Fn() -> Box<dyn Fn(&Value, Annotation) -> Result<Annotation>>,
{
    fn setup(&self, int: Interrogator, schema: Schema) -> Arc<ApplicatorFn> {
        let f = self();
        Arc::new(|value, next| -> Result<Annotation> {
            // match f(imp, value.clone(), eval) {
            //     Ok(eval) => {
            //         let sub_eval: Result<Annotation> = next.call(value);
            //         // todo: merge sub_eval with eval
            //         sub_eval
            //     }
            //     Err(err) => Err(err),
            todo!()
        })
    }
}
#[cfg(test)]
mod test {
    #[derive(Clone)]
    struct TestImpl {}
}
