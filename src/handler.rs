use std::{error::Error, fmt};

use crate::{
    error::SetupError,
    output::{Annotation, Structure},
    Interrogator, Schema, Scope,
};
use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;

#[async_trait]
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait Handler: Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    async fn setup<'h, 'i, 's, 'p>(
        &'h mut self,
        interrogator: &'i Interrogator,
        schema: &'s Schema,
        ptr: &'p Pointer,
    ) -> Result<bool, SetupError>;

    /// Executes the handler logic for the given [`Schema`] and [`Value`].
    async fn execute<'h, 's, 'v>(
        &'h self,
        scope: &'s mut Scope,
        value: &'v Value,
        output_structure: Structure,
    ) -> Result<Annotation, Box<dyn Error>>;
}

clone_trait_object!(Handler);

// #[async_trait]
// pub trait Execute: Send + Sync {
//     async fn execute(&self, scope: &mut Scope, value: &Value)
//         -> Result<Annotation, Box<dyn Error>>;
// }

// #[async_trait]
// impl<F> Execute for F
// where
//     F: for<'s, 'v> AsyncFn2<&'s mut Scope, &'v Value, Output = Result<Annotation, Box<dyn Error>>>
//         + Send
//         + Sync,

//     for<'s, 'v> <F as AsyncFn2<&'s mut Scope, &'v Value>>::OutputFuture: Send + Sync,
// {
//     async fn execute(
//         &self,
//         scope: &mut Scope,
//         value: &Value,
//     ) -> Result<Annotation, Box<dyn Error>> {
//         self(scope, value).await
//     }
// }

// #[async_trait]
// impl<H, E> Handler<E> for H
// where
//     E: Execute + Send + Sync,
//     H: for<'i, 's, 'p> AsyncFn3<
//             &'i Interrogator,
//             &'s Schema,
//             &'p Pointer,
//             Output = Result<Option<E>, SetupError>,
//         > + Sync
//         + Send,
//     for<'i, 's, 'p> <H as AsyncFn3<&'i Interrogator, &'s Schema, &'p Pointer>>::OutputFuture:
//         Send + Sync,
// {
//     async fn setup(
//         &self,
//         interrogator: &Interrogator,
//         schema: &Schema,
//         ptr: &Pointer,
//     ) -> Result<Option<E>, SetupError> {
//         self(interrogator, schema, ptr).await
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::{annotation::Detail, error::SetupError};

//     use super::*;
//     async fn run<H: Handler<E>, E: Execute>(handler: H) {
//         let i = Interrogator {};
//         let schema = Schema::Bool(true);
//         let mut scope = Scope::default();
//         let ptr = Pointer::default();
//         let value = Value::Bool(true);
//         let exec = handler.setup(&i, &schema, &ptr).await.unwrap();
//         let _ = exec.unwrap().execute(&mut scope, &value).await.unwrap();
//     }

//     async fn run_spike<'s, 'v>(
//         _scope: &'s mut Scope,
//         _value: &'v Value,
//     ) -> Result<Annotation, Box<dyn std::error::Error>> {
//         println!("inside run");
//         Ok(Annotation::Valid(Detail::default()))
//     }

//     async fn setup_spike<'i, 's, 'p>(
//         _interrogator: &'i Interrogator,
//         _schema: &'s Schema,
//         _ptr: &'p Pointer,
//     ) -> Result<Option<impl Execute>, SetupError> {
//         println!("inside setup");
//         if true {
//             Ok(Some(run_spike))
//         } else {
//             Ok(None)
//         }
//     }

//     #[tokio::test]
//     async fn test_spike() {
//         run(setup_spike).await;
//     }
// }
