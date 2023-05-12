use crate::{errors::InternalError, Annotation, Interrogator, Schema, Scope, SetupError};
use async_fn_traits::{AsyncFn2, AsyncFn3};
use async_trait::async_trait;
use jsonptr::Pointer;
use serde_json::Value;

#[async_trait]
pub trait Handler<E> {
    async fn setup(
        &self,
        interrogator: &Interrogator,
        schema: &Schema,
        ptr: &Pointer,
    ) -> Result<Option<E>, SetupError>;
}

#[async_trait]
pub trait Execute: Send + Sync {
    async fn execute(&self, scope: &mut Scope, value: &Value) -> Result<Annotation, InternalError>;
}

#[async_trait]
impl<F> Execute for F
where
    F: for<'s, 'v> AsyncFn2<&'s mut Scope, &'v Value, Output = Result<Annotation, InternalError>>
        + Send
        + Sync,

    for<'s, 'v> <F as AsyncFn2<&'s mut Scope, &'v Value>>::OutputFuture: Send + Sync,
{
    async fn execute(&self, scope: &mut Scope, value: &Value) -> Result<Annotation, InternalError> {
        self(scope, value).await
    }
}

#[async_trait]
impl<H, E> Handler<E> for H
where
    E: Execute + Send + Sync,
    H: for<'i, 's, 'p> AsyncFn3<
            &'i Interrogator,
            &'s Schema,
            &'p Pointer,
            Output = Result<Option<E>, SetupError>,
        > + Sync
        + Send,
    for<'i, 's, 'p> <H as AsyncFn3<&'i Interrogator, &'s Schema, &'p Pointer>>::OutputFuture:
        Send + Sync,
{
    async fn setup(
        &self,
        interrogator: &Interrogator,
        schema: &Schema,
        ptr: &Pointer,
    ) -> Result<Option<E>, SetupError> {
        self(interrogator, schema, ptr).await
    }
}

#[cfg(test)]
mod tests {
    use crate::annotation::Detail;

    use super::*;
    async fn run<H: Handler<E>, E: Execute>(handler: H) {
        let i = Interrogator {};
        let schema = Schema::Bool(true);
        let mut scope = Scope::default();
        let ptr = Pointer::default();
        let value = Value::Bool(true);
        let exec = handler.setup(&i, &schema, &ptr).await.unwrap();
        let _ = exec.unwrap().execute(&mut scope, &value).await.unwrap();
    }

    async fn run_spike<'s, 'v>(
        _scope: &'s mut Scope,
        _value: &'v Value,
    ) -> Result<Annotation, crate::errors::InternalError> {
        println!("inside run");
        Ok(Annotation::Valid(Detail::default()))
    }

    async fn setup_spike<'i, 's, 'p>(
        _interrogator: &'i Interrogator,
        _schema: &'s Schema,
        _ptr: &'p Pointer,
    ) -> Result<Option<impl Execute>, SetupError> {
        println!("inside setup");
        if true {
            Ok(Some(run_spike))
        } else {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_spike() {
        run(setup_spike).await;
    }
}
