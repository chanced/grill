use std::ops::Deref;

use crate::Injectable;

#[derive(Clone)]
pub struct Context<T>(pub T);

impl<T> Injectable for Context<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Value = T;
    fn inject(&self) -> Self::Value {
        self.0.clone()
    }
}

impl<T> From<T> for Context<T> {
    fn from(t: T) -> Self {
        Context(t)
    }
}

impl<T> Deref for Context<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
