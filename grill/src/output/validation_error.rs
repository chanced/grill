use core::fmt::{Debug, Display};


use dyn_clone::DynClone;
use serde::Serialize;

/// A validation error which can be used as the `"error"` field in
/// [`Output`](`crate::Output`) [`Node`](`crate::output::Node`)s.
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait ValidationError<'v>: Display + Debug + DynClone + Send + Sync {
    fn into_owned_box(self: Box<Self>) -> Box<dyn ValidationError<'static>>;
}

dyn_clone::clone_trait_object!(<'v> ValidationError<'v>);

impl Serialize for dyn ValidationError<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl ValidationError<'_> for String {
    fn into_owned_box(self: Box<Self>) -> Box<dyn ValidationError<'static>> {
        self
    }
}
