use core::fmt;

use dyn_clone::DynClone;
use serde::Serialize;

/// A trait which represents a validation error to be used as the `"error"` field in output
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait ValidationError<'v>: fmt::Display + fmt::Debug + DynClone + Send + Sync {
    fn into_owned(self) -> Box<dyn ValidationError<'static>>;
}

dyn_clone::clone_trait_object!(<'v> ValidationError<'v>);

impl Serialize for dyn ValidationError<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl ValidationError<'_> for String {
    fn into_owned(self) -> Box<dyn ValidationError<'static>> {
        Box::new(self)
    }
}
