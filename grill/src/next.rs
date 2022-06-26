use crate::{Annotation, ApplicatorFn, Error};
use serde_json::Value;
use std::sync::Arc;

pub struct Next {
    fns: Arc<Vec<Box<ApplicatorFn>>>,
    idx: usize,
}
impl Next {
    pub(crate) fn new(fns: Arc<Vec<Box<ApplicatorFn>>>, idx: usize) -> Self {
        Self { fns, idx }
    }
}

impl Next {
    pub fn call(&self, annotation: Annotation, value: &Value) -> Result<Annotation, Error> {
        if let Some(f) = self.fns.get(self.idx) {
            let next = Self::new(self.fns.clone(), self.idx + 1);
            f(value, annotation, next)
        } else {
            Ok(annotation)
        }
    }
}
