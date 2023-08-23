use crate::{
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, UriError},
    output::{self, Structure},
    schema::{Anchor, Reference},
    AbsoluteUri, Schema, Uri,
};
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;
use std::fmt;

use super::{Compile, Scope};

/// A handler that performs logic for a given condition in a JSON Schema.
#[derive(Debug, Clone)]
pub enum Handler {
    /// A synchronous handler.
    Sync(Box<dyn SyncHandler>),
    /// An asynchronous handler.
    Async(Box<dyn AsyncHandler>),
}

impl Handler {
    /// Returns `true` if the handler is [`Sync`].
    ///
    /// [`Sync`]: Handler::Sync
    #[must_use]
    pub fn is_sync(&self) -> bool {
        matches!(self, Self::Sync(..))
    }
    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn as_sync(&self) -> Option<&Box<dyn SyncHandler>> {
        if let Self::Sync(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the handler is [`Async`].
    ///
    /// [`Async`]: Handler::Async
    #[must_use]
    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(..))
    }

    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn as_async(&self) -> Option<&Box<dyn AsyncHandler>> {
        if let Self::Async(v) = self {
            Some(v)
        } else {
            None
        }
    }
    /// Attempts to identify the schema based on the [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `identify` for a given `Dialect`. It **must** be the
    /// **second** (index: `1`) `Handler` in the [`Dialect`](`crate::dialect::Dialect`)'s `Handler`s.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::IdHandler;
    /// use serde_json::json;
    ///
    /// let id = IdHandler.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".try_into().unwrap())));
    /// ```
    pub fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        match self {
            Handler::Sync(handler) => handler.identify(schema),
            Handler::Async(handler) => handler.identify(schema),
        }
    }
    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `is_pertinent_to` for a given `Dialect`.
    /// It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::SchemaHandler;
    ///
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema"}));
    /// assert_eq!(is_pertinent_to, true);
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({"$schema": "https://json-schema.org/draft/2019-09/schema"}));
    /// assert_eq!(is_pertinent_to, false);
    /// ```
    #[must_use]
    pub fn is_pertinent_to(&self, value: &Value) -> bool {
        match self {
            Handler::Sync(handler) => handler.is_pertinent_to(value),
            Handler::Async(handler) => handler.is_pertinent_to(value),
        }
    }
    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `dialect` for a given `Dialect`. It
    /// **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaHandler.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    pub fn dialect(&self, value: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        match self {
            Handler::Sync(handler) => handler.dialect(value),
            Handler::Async(handler) => handler.dialect(value),
        }
    }

    /// Returns a list of JSON [`Pointer`]s for each embedded schema within
    /// `value` relevant to this `Handler`.
    #[must_use]
    pub fn subschemas(&self, path: &Pointer, value: &Value) -> Vec<Pointer> {
        match self {
            Handler::Sync(h) => h.subschemas(path, value),
            Handler::Async(h) => h.subschemas(path, value),
        }
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Handler`
    pub fn anchors(&self, schema: &Value) -> Result<Vec<Anchor>, AnchorError> {
        match self {
            Handler::Sync(h) => h.anchors(schema),
            Handler::Async(h) => h.anchors(schema),
        }
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    pub fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        match self {
            Handler::Sync(h) => h.references(schema),
            Handler::Async(h) => h.references(schema),
        }
    }
}

#[async_trait]
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait AsyncHandler: IntoHandler + Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    async fn compile<'i>(
        &'i mut self,
        compile: &'i mut Compile<'i>,
        schema: &'i Value,
    ) -> Result<bool, CompileError>;

    /// Executes the handler logic for the given [`Schema`] and [`Value`].
    async fn evaluate<'i, 'v>(
        &'i self,
        scope: &'i mut Scope,
        schema: &'v Value,
        structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    fn subschemas(&self, path: &Pointer, schema: &Value) -> Vec<Pointer> {
        Vec::new()
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Handler`
    fn anchors(&self, schema: &Value) -> Result<Vec<Anchor>, AnchorError> {
        Ok(Vec::new())
    }

    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `identify` for a given `Dialect`.
    /// It **must** be the **second** (index: `1`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    #[allow(unused_variables)]
    fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        unimplemented!("identify must be implemented by the second Handler in a Dialect")
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the `dialect` method for a given
    /// `Dialect`. It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaHandler.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    #[allow(unused_variables)]
    fn dialect(&self, schema: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        unimplemented!("dialect must be implemented by the first Handler in a Dialect")
    }

    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `is_pertinent_to` for a given `Dialect`.
    /// It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(is_pertinent_to);
    ///
    /// let draft = "https://json-schema.org/draft/2019-09/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(!is_pertinent_to);
    /// ```
    #[allow(unused_variables)]
    fn is_pertinent_to(&self, schema: &Value) -> bool {
        unimplemented!("is_pertinent_to must be implemented by the first Handler in a Dialect")
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    #[allow(unused_variables)]
    fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        Ok(Vec::new())
    }
}

clone_trait_object!(AsyncHandler);

/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait SyncHandler: IntoHandler + Send + Sync + DynClone + fmt::Debug {
    /// For each [`Schema`] compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    fn compile<'i>(
        &'i mut self,
        compile: &'i mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError>;

    /// Evaluates the [`Value`] `value` and optionally returns an `Annotation`.
    ///
    /// Handlers should fail fast if the `structure` is
    /// [`Structure::Flag`](`crate::output::Structure::Flag`)
    fn evaluate<'i, 'v>(
        &'i self,
        scope: &'i mut Scope,
        value: &'v Value,
        _structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `identify` for a given
    /// `Dialect`. It **must** be the **second** (index: `1`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    #[allow(unused_variables)]
    fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        unimplemented!("identify must be implemented by the second Handler in a Dialect")
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `dialect` for a given `Dialect`. It
    /// **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaHandler.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    #[allow(unused_variables)]
    fn dialect(&self, value: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        unimplemented!("dialect must be implemented by the first Handler in a Dialect")
    }

    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `is_pertinent_to` for a
    /// given `Dialect`. It **must** be the **first** (index: `0`) `Handler` in
    /// the [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(is_pertinent_to);
    ///
    /// let draft = "https://json-schema.org/draft/2019-09/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(!is_pertinent_to);
    /// ```
    #[allow(unused_variables)]
    fn is_pertinent_to(&self, schema: &Value) -> bool {
        unimplemented!("is_pertinent_to must be implemented by the first Handler in a Dialect")
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Handler`
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
clone_trait_object!(SyncHandler);

pub trait IntoHandler {
    fn into_handler(self) -> Handler;
}

impl<T> IntoHandler for T
where
    T: Into<Handler>,
{
    fn into_handler(self) -> Handler {
        self.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::handler::State;

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
