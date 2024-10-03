use crate::{
dialect::Dialects, report::{self, Location},  EvaluateError, IntoOwned, JsonSchema, Output
};
use grill_core::{
    lang::context, state::State, DefaultKey, Key, Resolve
};
use serde::{Deserialize, Serialize};
use std::{
     error::Error as StdError, fmt::{Debug, Display}
};

pub mod keyword;

pub use keyword::Keyword;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Standard                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Standard JSON Schema [`Specification`].
#[derive(Clone, Debug)]
pub struct Standard<K = DefaultKey>
where
    K: 'static + Key + Send + Sync,
{
    dialects: Dialects<Self, K>,
}


impl<K> Standard<K> where K: 'static + Key + Send + Sync {
    pub fn new(dialects: Dialects<Self, K>) ->Standard<K> {
        Self {
            dialects,
        }
    }
}

impl<K> Specification<K> for Standard<K>
where
    K: 'static + Key + Send + Sync,
{
    type CompileError<R> = super::Error<Self::Report<'static>, R> where R: 'static + Resolve;
    type EvaluateError = EvaluateError<K>;
    type Evaluate<'int, 'val, 'req> = crate::keyword::context::Evaluate<'int, 'val, 'req, Self, K>;
    type Compile<'int, 'txn, 'res, R> = crate::keyword::context::Compile<'int, 'txn, 'res, R, Self, K> 
        where R: 'static + Resolve + Sized, 
        'int: 'txn,
        Self: 'txn + 'int;
    type Keyword = crate::keyword::Keyword;
    type Keywords<'int> = crate::keyword::Keywords<'int>;
    type Annotation<'v> = report::Annotation<'v>;
    type Error<'v> = report::Error<'v>;
    type Report<'v> = report::Report<Self::Annotation<'v>, Self::Error<'v>>;

    async fn compile<'int, 'txn, 'res, R>(
        &'int mut self,
        ctx: context::Compile<'int, 'txn, 'res, JsonSchema<K, Self>, R, K>,
    ) -> Result<Self::Compile<'int, 'txn, 'res, R>, Self::CompileError<R>>
    where
        'int: 'txn,
        R: 'static + Resolve + Send + Sync,
    {
        Ok(crate::keyword::context::Compile {
            dialects: &self.dialects,
            interrogator: ctx
        })
    }

    fn evaluate<'int, 'val, 'req>(
        &'int self,
        ctx: context::Evaluate<'int, 'val, 'req, JsonSchema<K, Self>, K>,
    ) -> Result<Self::Evaluate<'int, 'val, 'req>, Self::EvaluateError> {
        println!("init_evaluate");
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                Specification                                 ║
║                               ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait implemented by types which are capable of evaluating a specification
/// of JSON Schema.
#[trait_variant::make(Send)]
pub trait Specification<K>: 'static + Sized + Debug + Clone + Send + Sync
where
    K: 'static + Key + Send + Sync,
{
    /// The error type that can be returned when compiling a schema.
    type CompileError<R>: 'static + CompileError<R, Self, K>
    where
        R: 'static + Resolve;

    type EvaluateError: Send + StdError + From<EvaluateError<K>>;

    /// Context type supplied to `evaluate`.
    type Evaluate<'int, 'val, 'req>: Evaluate<'int, 'val, Self, K>
    where
        Self: 'int + 'val + 'req,
        K: 'static + Send;

    /// Context type supplied to `compile`.
    type Compile<'int, 'txn, 'res, R>:  Compile<'int, 'txn, 'res, R, Self, K> + Send
    where
        Self: 'txn + 'int + 'res,
        'int: 'txn,
        K: Key + 'static,
        R: 'static + Resolve + Send + Sync;

    type Keyword: 'static + keyword::Keyword<Self, K> + Send + Sync;

    /// A type which can hold a slice [`Self::Keyword`]s, returned from
    /// [`JsonSchema::keywords`].
    ///
    /// The purpose of allowing for this to be typed is so that convience
    /// accessor methods, can be accessed.
    type Keywords<'int>: From<&'int [Self::Keyword]> + IntoIterator<Item = &'int Self::Keyword>
    where
        Self: 'int,
        Self::Keyword: 'static;

    /// The annotation type to be used in `Self::Report`.
    ///
    // Even if an annotation is not used for a keyword, it is helpful to have
    /// unit struct as an annotation for analysis pre-serialization.
    ///
    ///[`ShouldSerialize`] is used by the `Report` to determine which annotations
    /// should be serialized.
    type Annotation<'v>: Serialize + ShouldSerialize;

    /// The error type to be used in `Self::Report`.
    /// 
    type Error<'v>: Send + IntoOwned + Display;

    type Report<'v>: Report<'v, Self::Annotation<'v>, Self::Error<'v>>;

    /// Initializes the specification.
    ///
    /// ## Errors
    /// Returns [`Self::InitError`] if an error occurs while initializing the
    /// specification.
    #[allow(unused_variables)]
    fn init(&mut self, state: &mut State<JsonSchema<K, Self>, K>) {}

    /// Initializes a `Self::Compile` context
    ///
    /// ## Errors
    /// Returns [`Self::CompileError`] if an error occurs while initializing the
    /// context.
    ///
    /// [`Self::CompileError`]: Specification::CompileError
    async fn compile<'int, 'txn, 'res, R>(
        &'int mut self,
        compile: context::Compile<'int, 'txn, 'res, JsonSchema<K, Self>, R, K>,
    ) -> Result<Self::Compile<'int, 'txn, 'res, R>, Self::CompileError<R>>
    where
        'int: 'txn,
        R: 'static + Resolve + Send + Sync;

    /// Initializes a `Self::Evaluate` context
    ///
    /// ## Errors
    /// Returns [`Self::EvaluateError`] if an error occurs while initializing the
    /// value
    ///
    /// [Self::EvaluateError]: Specification::EvaluateError
    fn evaluate<'int,  'val, 'req>(
        &'int self,
        eval: context::Evaluate<'int,  'val, 'req, JsonSchema<K, Self>, K>,
    ) -> Result<Self::Evaluate<'int, 'val, 'req>, Self::EvaluateError>;
}




/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                               ShouldSerialize                                ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                               ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait ShouldSerialize {
    fn should_serialize(&self) -> bool;
}




/// Context for [`Keyword::evaluate`].
pub trait Evaluate<'int, 'val, S, K>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    fn dialects(&self) -> &Dialects<S, K>;

    fn assess(
        &mut self,
    ) -> &mut <S::Report<'val> as Report<'val, S::Annotation<'val>, S::Error<'val>>>::Assess<'val>;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Report                                    ║
║                                   ¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// The result of evaluating a JSON Schema.
pub trait Report<'val, A, E>:
    for<'de> Deserialize<'de> + Serialize + Display + Debug + Send + std::error::Error
{
    /// A type which should allow for modification of a unit within the report.
    type Assess<'rpt>: Assess<'rpt, A, E>
    where
        Self: 'rpt;
    /// Creates a new report.
    fn new(output: Output, location: Location) -> Self;
    /// Returns `true` if the report is valid.
    fn is_valid(&self) -> bool;
    /// Retrieves or creates a unit within the `Report` at the given location
    /// and returns `Self::Assess`, which should be capable of mutating the unit
    /// in place.
    fn assess(&mut self, location: Location) -> Self::Assess<'_>;
}


/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Assess                                    ║
║                                   ¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait Assess<'rpt, A, E> {
    fn annotate(&mut self, annotation: A) -> Option<A>;
    fn fail(&mut self, error: E);
}


/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Error                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait CompileError<R, S, K>:
    'static + Send + StdError + From<crate::compile::Error<S::Report<'static>, R>>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve,
{
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Compile                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[trait_variant::make(Send)]
/// Context for [`Keyword::compile`].
pub trait Compile<'int, 'txn, 'res, R, S, K>: Send + Sync
where
    R: 'static + Resolve + Send,
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    fn interrogator(
        &mut self,
    ) -> &mut grill_core::lang::context::Compile<'int, 'txn, 'res, JsonSchema<K, S>, R, K>;
    
    fn language(&mut self) -> &mut crate::keyword::context::Compile<'int, 'txn, 'res, R, S, K>;
}


