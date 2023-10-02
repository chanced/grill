use core::fmt::{Debug, Display};
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Deserializer, Serialize};

/// An annotation or validation error of [`Output`](`crate::Output`).
///
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait Detail<'v>: DynClone + Display + Debug + Send + Sync {
    fn make_owned(self: Box<Self>) -> Box<dyn Detail<'static>>;
}
clone_trait_object!(<'v> Detail<'v>);

impl Serialize for dyn Detail<'_> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}
impl<'de> Deserialize<'de> for Box<dyn Detail<'static>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Box::new(String::deserialize(deserializer)?))
    }
}

impl Detail<'_> for String {
    fn make_owned(self: Box<Self>) -> Box<dyn Detail<'static>> {
        self
    }
}

impl Detail<'_> for &'_ str {
    fn make_owned(self: Box<Self>) -> Box<dyn Detail<'static>> {
        self
    }
}
