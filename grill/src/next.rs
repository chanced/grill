use serde_json::Value;

use crate::{applicator::ExecutorFn, Error, Evaluation};
use std::sync::Arc;
#[must_use]
pub struct Next {
    fns: Arc<Vec<Box<ExecutorFn>>>,
    idx: usize,
}

impl Next {
    pub(crate) fn new(fns: Arc<Vec<Box<ExecutorFn>>>) -> Self {
        Self { fns, idx: 0 }
    }
}

impl Next {
    pub fn call(&self, value: &Value, evaluation: Evaluation) -> Result<Evaluation, Error> {
        if let Some(f) = self.fns.get(self.idx) {
            let next = Self {
                fns: self.fns.clone(),
                idx: self.idx + 1,
            };
            // todo: check if call was invoked
            // todo: return an error if f does not return an error and next was not called.
            f(value, evaluation, next)
        } else {
            Ok(evaluation)
        }
    }
}
