use crate::{
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, UriError},
    output::{self, Structure},
    schema::{Anchor, Identifier, Reference},
    AbsoluteUri, Schema,
};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;
use std::{
    fmt::{self, Display},
    panic::RefUnwindSafe,
};

use super::{Compile, Context};

#[derive(Debug)]
pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not implemented")
    }
}

/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait Keyword: RefUnwindSafe + Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Keyword` is
    /// cloned and [`setup`] is called.
    ///
    /// If the keyword is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`evaluate`](`Self::evaluate`) should not
    /// be called for the given [`Schema`].
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        schema: &'v Value,
        structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;
    #[allow(unused_variables)]
    fn subschemas(&self, schema: &Value) -> Result<Vec<Pointer>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
    fn dialect(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<AbsoluteUri>, UriError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the method `is_pertinent_to` for a given `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaKeyword;
    ///
    /// let schema = serde_json::json!({
    ///     "$schema": "https://json-schema.org/draft/2020-12/schema
    /// });
    ///
    /// let is_pertinent_to = SchemaKeyword.is_pertinent_to(&schema);
    /// assert!(is_pertinent_to);
    ///
    /// let schema = serde_json::json!({"$schema": "https://json-schema.org/draft/2019-09/schema" });
    /// let is_pertinent_to = SchemaKeyword.is_pertinent_to(&schema);
    /// assert!(!is_pertinent_to);
    /// ```
    #[allow(unused_variables)]
    fn is_pertinent_to(&self, schema: &Value) -> Result<bool, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    #[allow(unused_variables)]
    fn references(
        &self,
        schema: &Value,
    ) -> Result<Result<Vec<Reference>, UriError>, Unimplemented> {
        Err(Unimplemented)
    }
}

clone_trait_object!(Keyword);

// impl Keyword for &dyn Keyword {
//     fn compile<'i>(
//         &mut self,
//         compile: &mut Compile<'i>,
//         schema: Schema<'i>,
//     ) -> Result<bool, CompileError> {
//         self.compile(compile, schema)
//     }

//     fn evaluate<'i, 'v>(
//         &'i self,
//         ctx: &'i mut Context,
//         schema: &'v Value,
//         structure: Structure,
//     ) -> Result<Option<output::Node<'v>>, EvaluateError> {
//         self.evaluate(ctx, schema, structure)
//     }
// }

#[cfg(test)]
mod tests {
    use crate::interrogator::state::State;

    #[test]
    fn test_get() {
        let mut state = State::new();
        let i: i32 = 1;
        state.insert(i);
        let x = state.get_mut::<i32>().unwrap();
        *x += 1;

        assert_eq!(state.get::<i32>(), Some(&2));
    }
}
