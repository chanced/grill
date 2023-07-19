use std::fmt;

use crate::{
    dialect::{Dialects, LocatedSchema},
    error::{CompileError, EvaluateError, IdentifyError, LocateSchemasError},
    keyword::SchemaKeyword,
    output::{self, Structure},
    AbsoluteUri, Compile, Scope, Uri,
};

use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;

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
    /// Returns a list of [`SchemaKeyword`](`crate::keyword::SchemaKeyword`)
    /// which this `Handler` processes.
    #[must_use]
    pub fn schema_keywords(&self) -> Option<&'static [SchemaKeyword<'static>]> {
        match self {
            Handler::Sync(h) => h.schema_keywords(),
            Handler::Async(h) => h.schema_keywords(),
        }
    }

    pub fn identify_schema(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        match self {
            Handler::Sync(h) => h.identify_schema(schema),
            Handler::Async(h) => h.identify_schema(schema),
        }
    }

    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &mut AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        match self {
            Handler::Sync(h) => h.locate_schemas(path, value, dialects, base_uri),
            Handler::Async(h) => h.locate_schemas(path, value, dialects, base_uri),
        }
    }
}

#[async_trait]
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait AsyncHandler: Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    async fn compile<'h, 'c, 's, 'p>(
        &mut self,
        compile: &'c mut Compile<'s>,
        schema: &'s Value,
    ) -> Result<bool, CompileError>;

    /// Executes the handler logic for the given [`Schema`] and [`Value`].
    async fn evaluate<'h, 's, 'v>(
        &'h self,
        scope: &'s mut Scope,
        value: &'v Value,
        _structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    /// Returns a list of [`SchemaKeyword`](`crate::keyword::SchemaKeyword`)
    /// which this `Handler` processes.
    fn schema_keywords(&self) -> Option<&'static [SchemaKeyword<'static>]> {
        None
    }

    fn identify_schema(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        Ok(None)
    }
    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &mut AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        todo!()
    }
}

clone_trait_object!(AsyncHandler);
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.

pub trait SyncHandler: Send + Sync + DynClone + fmt::Debug {
    /// For each [`Schema`] compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    fn compile<'s>(
        &mut self,
        compile: &mut Compile<'s>,
        schema: &'s Value,
    ) -> Result<bool, CompileError>;

    /// Evaluates the [`Value`] `value` and optionally returns an `Annotation`.
    ///
    /// Handlers should fail fast if the `structure` is [`Structure::Flag`](`crate::output::Structure::Flag`)
    fn evaluate<'v>(
        &self,
        scope: &mut Scope,
        value: &'v Value,
        _structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    /// Returns a `Vec` of [`SchemaKeyword`](`crate::keyword::SchemaKeyword`),
    /// that is [`Keyword`](`crate::keyword::Keyword`) which contain one or more
    /// schemas, which this `Handler` processes.
    fn schema_keywords(&self) -> Option<&'static [SchemaKeyword<'static>]> {
        // TODO: remove this
        None
    }

    fn identify_schema(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        Ok(None)
    }
    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &mut AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        todo!()
    }
}
clone_trait_object!(SyncHandler);
