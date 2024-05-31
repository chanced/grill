use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use grill_core::{
    big::BigRational,
    lang::{source::Source, Init, Numbers, Sources, Values},
    Key, Resolve,
};
use grill_uri::AbsoluteUri;
use inherent::inherent;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::{
    report::{Annotation, Error, Location},
    schema::CompiledSchema,
    CompileError, EvaluateError, IntoOwned, Output,
};

pub trait ShouldSerialize {
    fn should_serialize(&self) -> bool;
}

/// A trait implemented by types which are capable of evaluating a specification
/// of JSON Schema.
pub trait Specification<K: Key>: Sized + Debug + Clone {
    /// The error type that can be returned when initializing the dialect.
    type InitError;

    /// The error type that can be returned when compiling a schema.
    type CompileError: for<'v> From<CompileError<<Self::Error<'v> as IntoOwned>::Owned>>;

    type EvaluateError: for<'v> From<EvaluateError<K>>;

    /// The context type supplied to `evaluate`.
    type Evaluate: Evaluate<K>;

    type Compile: Compile<K>;

    type Keyword: Keyword<Self, K>;

    /// The annotation type to be used in `Self::Report`.
    ///
    /// Even if an annotation is not used for a keyword, it is helpful to have
    /// unit struct as an annotation for analysis pre-serialization.
    ///
    ///[`ShouldSerialize`] is used by the `Report` to determine which annotations
    /// should be serialized.
    type Annotation<'v>: Serialize + ShouldSerialize;

    /// The error type to be used in `Self::Report`.
    type Error<'v>: IntoOwned + Display;

    type Report<'v>: Report<'v, Self::Annotation<'v>, Self::Error<'v>>;

    /// Initializes the specification.
    fn init(&mut self, init: Init<'_, CompiledSchema<Self, K>, K>) -> Result<(), Self::InitError> {
        Ok(())
    }

    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, CompiledSchema<Self, K>, R, K>,
    ) -> Result<Self::Compile, Self::CompileError>;

    fn evaluate<'i, 'v>(
        &'i self,
        eval: grill_core::lang::Evaluate<'i, 'v, CompiledSchema<Self, K>, Output, K>,
    ) -> Result<Self::Evaluate, Self::EvaluateError>;
}

pub mod alias {
    use super::Specification;
    pub type InitError<S, K> = <S as Specification<K>>::InitError;
    pub type CompileError<S, K> = <S as Specification<K>>::CompileError;
    pub type EvaluateError<S, K> = <S as Specification<K>>::EvaluateError;
    pub type Evaluate<S, K> = <S as Specification<K>>::Evaluate;
    pub type Compile<S, K> = <S as Specification<K>>::Compile;
    pub type Report<'v, S, K> = <S as Specification<K>>::Report<'v>;
    pub type Annotation<'v, S, K> = <S as Specification<K>>::Annotation<'v>;
    pub type Error<'v, S, K> = <S as Specification<K>>::Error<'v>;
    pub type EvaluateResult<'v, S, K> = Result<Report<'v, S, K>, EvaluateError<S, K>>;
}

/// Context for [`Keyword::compile`].
pub trait Compile<K>
where
    K: Key,
{
    /// Retrieves a schema from the store by [`AbsoluteUri`].
    fn schema(&self, uri: &AbsoluteUri) -> Option<K>;

    /// Returns a mutable reference to [`Numbers`] cache.
    fn numbers(&mut self) -> &mut Numbers;

    /// Parses a JSON number into an [`Arc<BigRational>`](`BigRational`) and
    /// stores it in the [`Numbers`] cache if it is not already present.
    /// Otherwise, the existing [`BigRational`] is returned.
    fn number(&mut self, number: &Number) -> Result<Arc<BigRational>, grill_core::big::ParseError>;

    /// Returns a mutable reference to [`Values`] cache.
    fn values(&mut self) -> &mut Values;

    /// If `value` is already in the [`Values`] cache, the existing
    /// `Arc<Value>` is cloned and returned. Otherwise, `value` is inserted
    /// as an `Arc<Value>`, cloned, and returned.
    fn value(&mut self, value: &Value) -> Arc<Value>;

    /// Returns a reference to [`Sources`].
    fn sources(&self) -> &Sources;

    /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a
    /// [`Link`](grill_core::lang::source::Link) exists.
    fn source(&self, uri: &AbsoluteUri) -> Option<Source>;
}

/// Context for [`Keyword::evaluate`].
pub trait Evaluate<K: Key> {
    fn schema(&self, uri: &AbsoluteUri) -> Option<K>;
}

pub trait Keyword<S, K>: Send + Debug + Clone + PartialEq + Eq
where
    S: Specification<K>,
    K: Key,
{
    fn compile<'i>(
        &self,
        compile: alias::Compile<S, K>,
    ) -> Option<Result<(), alias::CompileError<S, K>>>;

    fn evaluate<'v>(&self, eval: alias::Evaluate<S, K>) -> Result<(), alias::EvaluateError<S, K>>;
}

pub trait Report<'v, A, E>: for<'de> Deserialize<'de> + Serialize + Display + Debug + Send {
    /// A type which should allow for modification of a unit within the report.
    type Assess<'r>: Assess<'r, A, E>
    where
        Self: 'r;

    fn new(output: Output, location: Location) -> Self;
    fn instance_location(&self) -> Option<&jsonptr::Pointer>;
    fn keyword_location(&self) -> Option<&jsonptr::Pointer>;
    fn absolute_keyword_location(&self) -> Option<&AbsoluteUri>;
    fn location(&self) -> Option<&Location>;
    fn is_valid(&self) -> bool;
    fn assess(&mut self, location: Location) -> Self::Assess<'_>;
}

pub trait Assess<'r, A, E> {
    fn annotate(&mut self, annotation: A) -> Option<A>;
    fn fail(&mut self, error: E);
}
