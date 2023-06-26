use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;

/// Determines whether or not a [`Value`] is a schema of this [`Dialect`].
pub trait IsSchema: Send + Sync + DynClone {
    /// Determines whether or not a [`Value`] is a schema of this [`Dialect`].
    ///
    /// This method errs on the side of caution and should report `false` even if
    /// it is possible `value` is a schema of this [`Dialect`].
    ///
    /// # Example
    /// ```
    /// use grill::json_schema::
    fn is_schema(&self, value: &Value) -> bool;
}
clone_trait_object!(IsSchema);

impl<F> IsSchema for F
where
    F: Fn(&Value) -> bool + Send + Sync + Clone,
{
    fn is_schema(&self, value: &Value) -> bool {
        (self)(value)
    }
}
