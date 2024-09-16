use crate::{
    compile,
    report::{self, Location},
    schema::{self, dialect::{Dialect, Dialects}},
    EvaluateError, IntoOwned, JsonSchema, Output,
};
use grill_core::{
    lang, state::{State, Transaction}, DefaultKey, Key, Resolve
};
use grill_uri::AbsoluteUri;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap, error::Error as StdError, fmt::{Debug, Display}
};

/// Return types [`Keyword`] trait.
pub mod found;


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
    type CompileError<R> = super::CompileError<Self::Report<'static>, R> where R: 'static + Resolve;

    type EvaluateError = EvaluateError<K>;

    type Evaluate<'int, 'val, 'req> = schema::keyword::Evaluate<'int, 'val, 'req, Self, K>;

    type Compile<'int, 'txn, 'res, R> = schema::keyword::Compile<'int, 'txn, 'res, R, Self, K> 
        where R: 'static + Resolve + Sized, 
        'int: 'txn,
        Self: 'txn + 'int;

    type Keyword = schema::keyword::Keyword;

    type Keywords<'int> = schema::keyword::Keywords<'int>;

    type Annotation<'v> = report::Annotation<'v>;

    type Error<'v> = report::Error<'v>;

    type Report<'v> = report::Report<Self::Annotation<'v>, Self::Error<'v>>;

    async fn init_compile<'int, 'txn, 'res, R>(
        &'int mut self,
        ctx: lang::Compile<'int, 'txn, 'res, JsonSchema<K, Self>, R, K>,
    ) -> Result<Self::Compile<'int, 'txn, 'res, R>, Self::CompileError<R>>
    where
        'int: 'txn,
        R: 'static + Resolve + Send + Sync,
    {
        Ok(schema::keyword::Compile {
            dialects: &self.dialects,
            resolve: ctx.resolve,
            targets: ctx.targets,
            txn: ctx.txn,
            must_validate: ctx.must_validate
        })
    }

    fn init_evaluate<'int, 'val, 'req>(
        &'int self,
        ctx: lang::Evaluate<'int, 'val, 'req, JsonSchema<K, Self>, K>,
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
    type CompileError<R>: 'static + CompileError<Self, K, R>
    where
        R: 'static + Resolve;

    type EvaluateError: Send + StdError + From<EvaluateError<K>>;

    /// Context type supplied to `evaluate`.
    type Evaluate<'int, 'val, 'req>: Evaluate<'int, 'val, Self, K>
    where
        Self: 'int + 'val + 'req,
        K: 'static + Send;

    /// Context type supplied to `compile`.
    type Compile<'int, 'txn, 'res, R>: Compile<'int, 'txn, 'res, R, Self, K> + Send
    where
        Self: 'txn + 'int + 'res,
        'int: 'txn,
        R: 'static + Resolve + Send + Sync,
        K: Key + 'static;

    type Keyword: 'static + Keyword<Self, K> + Send + Sync;

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
    async fn init_compile<'int, 'txn, 'res, R>(
        &'int mut self,
        compile: lang::Compile<'int, 'txn, 'res, JsonSchema<K, Self>, R, K>,
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
    fn init_evaluate<'int,  'val, 'req>(
        &'int self,
        eval: lang::Evaluate<'int,  'val, 'req, JsonSchema<K, Self>, K>,
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


/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                     Init                                     ║
║                                    ¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub struct Init<'int, S, K>
where
    K: 'static + Key + Send + Sync,
    S: 'static + Specification<K>,
{
    pub state: &'int mut State<JsonSchema<K, S>, K>,
    pub dialects: &'int mut Dialects<S, K>,
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                 CompileError                                 ║
║                                ¯¯¯¯¯¯¯¯¯¯¯¯¯¯                                ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait CompileError<S, K, R>:
    'static + Send + StdError + From<compile::CompileError<S::Report<'static>, R>>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve,
{
    fn is_recoverable(&self) -> bool;
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
    fn targets(&mut self) -> &[AbsoluteUri];

    fn txn(&mut self) -> &mut Transaction<'_, 'int, JsonSchema<K, S>, K>;

    fn dialects(&self) -> &Dialects<S, K>;

    fn resolve(&self) -> &'res R;

    /// Indicates whether or not the schemas should be validated.
    ///
    /// This is needed for compiling meta-schemas. A value of `false`
    fn should_validate(&self) -> bool;
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
║                                   Keyword                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait implemented by types which are capable of evaluating one or more
/// keywords of a JSON Schema specification.
#[trait_variant::make(Send)]
pub trait Keyword<S, K>: Send + Debug + Clone + PartialEq + Eq
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    /// Compiles the keyword.
    async fn compile<R>(
        &self,
        compile: S::Compile<'_, '_, '_, R>,
    ) -> Option<Result<(), S::CompileError<R>>>
    where
        R: 'static + Resolve + Send + Sync;

    /// Evaluates the keyword.
    ///
    /// ## Errors
    /// returns the [`Specification`]'s
    /// [`EvaluateError`](`Specification::EvaluateError`) if an error occurs while validating.
    /// Failing to validate is not an error.
    fn evaluate(
        &self,
        eval: S::Evaluate<'_, '_, '_>,
    ) -> Result<(), S::EvaluateError>;

    /// Returns the string URI for the referenced schema this keyword is capable
    /// of handling, if present.
    fn reference(&self, _schema: &Value) -> Option<found::Reference> {
        None
    }

    /// Returns the name of the anchor this keyword is capable of handling, if
    /// present.
    fn anchor(&self, _schema: &Value) -> Option<found::Anchor> {
        None
    }
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
