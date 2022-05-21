use crate::{Evaluation, Result, Value};

pub struct Next {
    prefix: String,
    func: Option<Box<dyn FnOnce(Value) -> Result<Evaluation>>>,
}
impl Next {
    pub fn call(self, value: Value) -> Result<Evaluation> {
        self.func.map_or(Ok(Evaluation::new()), |f| match f(value) {
            Ok(eval) => eval.prefix(&self.prefix),
            Err(err) => Err(err),
        })
    }
}
