use crate::{
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, UriError},
    output::{self, Structure},
    schema::{Anchor, Identifier, Reference},
    AbsoluteUri, Schema,
};
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;
use std::{fmt, panic::RefUnwindSafe};

use super::{Compile, Context};

/// A keyword for a given keyword in a JSON Schema Dialect.
#[derive(Debug, Clone)]
pub enum Keyword {
    /// A synchronous keyword.
    Sync(Box<dyn SyncKeyword>),
    /// An asynchronous keyword.
    Async(Box<dyn AsyncKeyword>),
}

impl Keyword {
    pub async fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        match self {
            Self::Sync(keyword) => keyword.compile(compile, schema),
            Self::Async(keyword) => keyword.compile(compile, schema).await,
        }
    }

    /// Returns `true` if the keyword is [`Sync`].
    ///
    /// [`Sync`]: Keyword::Sync
    #[must_use]
    pub fn is_sync(&self) -> bool {
        matches!(self, Self::Sync(..))
    }

    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn as_sync(&self) -> Option<&Box<dyn SyncKeyword>> {
        if let Self::Sync(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the keyword is [`Async`].
    ///
    /// [`Async`]: Keyword::Async
    #[must_use]
    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(..))
    }

    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn as_async(&self) -> Option<&Box<dyn AsyncKeyword>> {
        if let Self::Async(v) = self {
            Some(v)
        } else {
            None
        }
    }
    /// Attempts to identify the schema based on the [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the method `identify` for a given `Dialect`. It **must** be the
    /// **second** (index: `1`) `Keyword` in the [`Dialect`](`crate::dialect::Dialect`)'s `Keyword`s.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::keywords::IdKeyword;
    /// use serde_json::json;
    ///
    /// let id = IdKeyword.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".try_into().unwrap())));
    /// ```
    pub fn identify(&self, schema: &Value) -> Result<Option<Identifier>, IdentifyError> {
        match self {
            Keyword::Sync(keyword) => keyword.identify(schema),
            Keyword::Async(keyword) => keyword.identify(schema),
        }
    }
    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the method `is_pertinent_to` for a given `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::keywords::SchemaKeyword;
    ///
    /// let is_pertinent_to = SchemaKeyword.is_pertinent_to(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema"}));
    /// assert_eq!(is_pertinent_to, true);
    /// let is_pertinent_to = SchemaKeyword.is_pertinent_to(&json!({"$schema": "https://json-schema.org/draft/2019-09/schema"}));
    /// assert_eq!(is_pertinent_to, false);
    /// ```
    #[must_use]
    pub fn is_pertinent_to(&self, value: &Value) -> bool {
        match self {
            Keyword::Sync(keyword) => keyword.is_pertinent_to(value),
            Keyword::Async(keyword) => keyword.is_pertinent_to(value),
        }
    }

    /// Returns a list of JSON [`Pointer`]s for each embedded schema within
    /// `value` relevant to this `Keyword`.
    #[must_use]
    pub fn subschemas(&self, path: &Pointer, value: &Value) -> Vec<Pointer> {
        match self {
            Keyword::Sync(h) => h.subschemas(path, value),
            Keyword::Async(h) => h.subschemas(path, value),
        }
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    pub fn anchors(&self, schema: &Value) -> Result<Vec<Anchor>, AnchorError> {
        match self {
            Keyword::Sync(h) => h.anchors(schema),
            Keyword::Async(h) => h.anchors(schema),
        }
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    pub fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        match self {
            Keyword::Sync(h) => h.references(schema),
            Keyword::Async(h) => h.references(schema),
        }
    }
}

#[async_trait]
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait AsyncKeyword: IntoKeyword + RefUnwindSafe + Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Keyword` is
    /// cloned and [`setup`] is called.
    ///
    /// If the keyword is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
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
        schema: &'v Value,
        structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;
    #[allow(unused_variables)]
    fn subschemas(&self, path: &Pointer, schema: &Value) -> Vec<Pointer> {
        Vec::new()
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    #[allow(unused_variables)]
    fn anchors(&self, schema: &Value) -> Result<Vec<Anchor>, AnchorError> {
        Ok(Vec::new())
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
    fn identify(&self, schema: &Value) -> Result<Option<Identifier>, IdentifyError> {
        unimplemented!("identify must be implemented by at least one Keyword in a Dialect")
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
    fn dialect(&self, schema: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        unimplemented!("dialect is not implemented by this Keyword")
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
    fn is_pertinent_to(&self, schema: &Value) -> bool {
        unimplemented!("is_pertinent_to is not implemented by this Keyword")
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    #[allow(unused_variables)]
    fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        Ok(Vec::new())
    }
}

clone_trait_object!(AsyncKeyword);

/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait SyncKeyword: IntoKeyword + RefUnwindSafe + Send + Sync + DynClone + fmt::Debug {
    /// For each [`Schema`] compiled by the [`Interrogator`], this `Keyword` is
    /// cloned and [`setup`] is called.
    ///
    /// If the keyword is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError>;

    /// Evaluates the [`Value`] `value` and optionally returns an `Annotation`.
    ///
    /// Keywords should fail fast if the `structure` is
    /// [`Structure::Flag`](`crate::output::Structure::Flag`)
    fn evaluate<'v>(
        &self,
        scope: &mut Context,
        value: &'v Value,
        structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the method `identify` for a given
    /// `Dialect`. It **must** be the **second** (index: `1`) `Keyword` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Keywords`](`crate::dialect::Keywords`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::keywords::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    #[allow(unused_variables)]
    fn identify(&self, schema: &Value) -> Result<Option<Identifier>, IdentifyError> {
        unimplemented!("identify must be implemented by the second Keyword in a Dialect")
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
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let is_pertinent_to = SchemaKeyword.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(is_pertinent_to);
    ///
    /// let draft = "https://json-schema.org/draft/2019-09/schema";
    /// let is_pertinent_to = SchemaKeyword.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(!is_pertinent_to);
    /// ```
    #[allow(unused_variables)]
    fn is_pertinent_to(&self, schema: &Value) -> bool {
        unimplemented!("is_pertinent_to must be implemented by the first Keyword in a Dialect")
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    fn anchors(&self, schema: &Value) -> Result<Vec<Anchor>, AnchorError> {
        Ok(Vec::new())
    }

    /// Returns a list of [`LocatedSchema`] for each subschema in `value`.
    #[allow(unused_variables)]
    fn subschemas(&self, path: &Pointer, value: &Value) -> Vec<Pointer> {
        Vec::new()
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    #[allow(unused_variables)]
    fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        Ok(Vec::new())
    }
}
clone_trait_object!(SyncKeyword);

pub trait IntoKeyword {
    fn into_keyword(self) -> Keyword;
}

impl<T> IntoKeyword for T
where
    T: Into<Keyword>,
{
    fn into_keyword(self) -> Keyword {
        self.into()
    }
}

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
