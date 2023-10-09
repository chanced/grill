use super::Translations;
use core::fmt::{Debug, Display};
use dyn_clone::{clone_trait_object, DynClone};
use serde::{Deserialize, Deserializer, Serialize};

pub type BoxedError<'v> = Box<dyn 'v + Send + Sync + Error<'v>>;

/// An validation error, used as the value of `"error"` in [`Output`](`crate::Output`).
///
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait Error<'v>: DynClone + Display + Debug + Send + Sync {
    fn make_owned(self: Box<Self>) -> Box<dyn Error<'static>>;
    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        lang: &Translations,
    ) -> std::fmt::Result;
}

clone_trait_object!(<'v> Error<'v>);

impl<'v> Serialize for Box<dyn 'v + Send + Sync + Error<'v>> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Box<dyn Error<'static>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Box::new(String::deserialize(deserializer)?))
    }
}

impl Error<'_> for String {
    fn make_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        self
    }
    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _lang: &Translations,
    ) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<'v> Error<'v> for &'v str {
    fn make_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        Box::new(self.to_string())
    }

    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _lang: &Translations,
    ) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
