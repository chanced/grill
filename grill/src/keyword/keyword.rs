use super::{Compile, Context};
use crate::{
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, UriError},
    output::{self, Translator},
    schema::{Anchor, Identifier, Ref},
    AbsoluteUri, Schema,
};
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not implemented")
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Kind {
    /// The [`Keyword`] is singular, evaluating the logic of a specific
    /// JSON Schema Keyword.
    Single(&'static str),
    /// The [`Keyword`] is a composite of multiple keywords with additional
    /// logic which handles co-dependencies between the embedded keywords.
    ///
    /// The output of this keyword should be a transient
    /// [`Output`](`crate::Output`), with `is_transient` set to `true`.
    /// Depending on the specified `Structure`, the [`Output`] may be expanded
    /// into multiple nodes.
    Composite(&'static [&'static str]),
}

impl PartialEq<&str> for Kind {
    fn eq(&self, other: &&str) -> bool {
        if let Kind::Single(s) = self {
            s == other
        } else {
            false
        }
    }
}

impl From<&'static str> for Kind {
    fn from(s: &'static str) -> Self {
        Kind::Single(s)
    }
}
impl From<&'static [&'static str]> for Kind {
    fn from(s: &'static [&'static str]) -> Self {
        Kind::Composite(s)
    }
}

/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
#[allow(unused_variables)]
#[async_trait]
pub trait Keyword: Send + Sync + DynClone + fmt::Debug {
    /// The [`Kind`] of the keyword. `Kind` can be either `Single`, which will
    /// be the name of the keyword or `Composite`, which will be a list of
    /// keywords.
    fn kind(&self) -> Kind;

    /// For each `Schema` compiled by the [`Interrogator`], this `Keyword` is
    /// cloned and [`setup`] is called.
    ///
    /// If the keyword is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`evaluate`](`Self::evaluate`) should not
    /// be called for the given [`Schema`].
    async fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    async fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<output::Output<'v>>, EvaluateError>;

    fn set_translate(&mut self, translator: &Translator) -> Result<(), Unimplemented> {
        Err(Unimplemented)
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<Pointer>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    fn anchors(&self, schema: &Value) -> Result<Result<Vec<Anchor>, AnchorError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// At least `Keyword` must implement the method `identify` for a given `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::keywords::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json" }));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    fn identify(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<Identifier>, IdentifyError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the `dialect` method for a given
    /// `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaKeyword;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaKeyword.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    fn dialect(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<AbsoluteUri>, IdentifyError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    fn refs(&self, schema: &Value) -> Result<Result<Vec<Ref>, UriError>, Unimplemented> {
        Err(Unimplemented)
    }
}

clone_trait_object!(Keyword);

#[cfg(test)]
mod tests {
    use crate::anymap::AnyMap;

    #[test]
    fn test_get() {
        let mut state = AnyMap::new();
        let i: i32 = 1;
        state.insert(i);
        let x = state.get_mut::<i32>().unwrap();
        *x += 1;

        assert_eq!(state.get::<i32>(), Some(&2));
    }
}
