use crate::Specification;
use grill_core::Resolve;
use jsonptr::PointerBuf;
use serde_json::Value;
use slotmap::Key;

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
pub trait Keyword<S, K>: Send + std::fmt::Debug + Clone + PartialEq + Eq
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
    fn evaluate(&self, eval: S::Evaluate<'_, '_, '_>) -> Result<(), S::EvaluateError>;

    /// Returns the string URI for the referenced schema this keyword is capable
    /// of handling, if present.
    fn reference(&self, _schema: &Value) -> Option<Found<String>> {
        None
    }

    /// Returns the name of the anchor this keyword is capable of handling, if
    /// present.
    fn anchor(&self, _schema: &Value) -> Option<Found<String>> {
        None
    }

    fn embedded_schemas(&self, _schema: &Value) -> Vec<Found<PointerBuf>> {
        Vec::new()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Found                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found<T> {
    pub keyword: &'static str,
    pub value: T,
}
