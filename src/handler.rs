use std::{error::Error, fmt};

use crate::{
    error::SetupError,
    output::{Annotation, Structure},
    Compiler, Schema, Scope,
};
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Handler {
    /// A synchronous handler.
    Sync(Box<dyn SyncHandler>),
    /// An asynchronous handler.
    Async(Box<dyn AsyncHandler>),
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
    async fn setup<'h, 'c, 's, 'p>(
        &mut self,
        compiler: &'c mut Compiler<'s>,
        schema: &'s Schema,
    ) -> Result<bool, SetupError>;

    /// Executes the handler logic for the given [`Schema`] and [`Value`].
    async fn evaluate<'h, 's, 'v>(
        &'h self,
        scope: &'s mut Scope,
        value: &'v Value,
        output_structure: Structure,
    ) -> Result<Option<Annotation<'v>>, Box<dyn Error>>;
}

clone_trait_object!(AsyncHandler);
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.

pub trait SyncHandler: Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    fn setup<'s>(
        &mut self,
        compiler: &mut Compiler<'s>,
        schema: &'s Schema,
    ) -> Result<bool, SetupError>;

    /// Executes the handler logic for the given [`Schema`] and [`Value`].
    fn evaluate<'v>(
        &self,
        scope: &mut Scope,
        value: &'v Value,
        output_structure: Structure,
    ) -> Result<Option<Annotation<'v>>, Box<dyn Error>>;
}
clone_trait_object!(SyncHandler);
