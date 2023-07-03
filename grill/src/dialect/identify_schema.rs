use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;

use crate::{error::IdentifyError, Uri};

/// Identifies schemas when possible.
/// # Implementations
pub trait IdentifySchema: Send + Sync + DynClone {
    /// Identifies a schema
    /// # Errors
    /// Returns [`IdentifyError`] if:
    ///   - The identifier fails to parse as a [`Uri`]
    ///   - The identifier does not conform to the [`Dialect`]'s requirements
    ///     (e.g. containing a fragment for specific dialects such as 2019-09,
    ///     2020-12)
    ///   - An anchor is found which does not conform to the [`Dialect`]'s
    ///     requirements (e.g. not starting with a letter containing characters
    ///     are not [AZaz0-9-_.])
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
