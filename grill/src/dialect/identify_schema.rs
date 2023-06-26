use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;

use crate::{error::IdentifyError, Uri};

/// Identifies schemas when possible.
/// # Implementations
pub trait IdentifySchema: Send + Sync + DynClone {
    /// Identifies a schema
    /// # Errors
    /// Returns [`IdentifyError`] if:
    ///   - The identity fails to parse as a [`Uri`]
    ///   - The identity contains a fragment (e.g. `"example#fragment"`)
    ///     for a [`Dialect`] which does not allow fragments (i.e. 2019-09, 2020-12).
    fn identify_schema(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError>;
}
clone_trait_object!(IdentifySchema);

impl<F> IdentifySchema for F
where
    F: Clone + Send + Sync + Fn(&Value) -> Result<Option<Uri>, IdentifyError>,
{
    fn identify_schema(&self, value: &Value) -> Result<Option<Uri>, IdentifyError> {
        (self)(value)
    }
}
