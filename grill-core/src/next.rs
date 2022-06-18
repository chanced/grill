use crate::{Evaluation, Result, Value};

pub struct Next<I> {
    prefix: String,
    func: Option<Box<dyn FnOnce(Value) -> Result<Evaluation<I>>>>,
}
impl<I> Next<I> {
    pub fn call(self, value: Value) -> Result<Evaluation<I>> {
        self.func.map_or(todo!(), |f| match f(value) {
            Ok(eval) => eval.prefix(&self.prefix),
            Err(err) => Err(err),
        })
    }
}
