use jsonptr::Pointer;
use serde_json::Value;

use crate::{
    anymap::AnyMap,
    error::EvaluateError,
    output::{Annotation, AnnotationOrError, Error},
    schema::Schemas,
    source::Sources,
    AbsoluteUri, Key, Output, Structure,
};

/// Contains global and evaluation level [`State`], schemas, and location
/// information needed to [`evaluate`](`crate::Interrogator::evaluate`) a
/// schema.
pub struct Context<'i> {
    pub(crate) absolute_keyword_location: &'i AbsoluteUri,
    pub(crate) keyword_location: Pointer,
    pub(crate) instance_location: Pointer,
    pub(crate) structure: Structure,
    /// global state of the interrogator
    pub(crate) global_state: &'i AnyMap,
    pub(crate) eval_state: &'i mut AnyMap,
    pub(crate) schemas: &'i Schemas,
    pub(crate) sources: &'i Sources,
}

impl<'s> Context<'s> {
    pub async fn evalute<'v>(
        &mut self,
        key: Key,
        instance: &str,
        value: &'v Value,
    ) -> Result<Output<'v>, EvaluateError> {
        let token = jsonptr::Token::from(instance);
        let mut instance_location = self.instance_location.clone();
        instance_location.push_back(token.clone());
        let mut keyword_location = self.keyword_location.clone();
        keyword_location.push_back(token);
        self.schemas
            .evaluate(
                self.structure,
                key,
                value,
                instance_location,
                keyword_location,
                self.sources,
                self.global_state,
                self.eval_state,
            )
            .await
    }

    #[must_use]
    pub fn global_state(&self) -> &AnyMap {
        self.global_state
    }

    pub fn eval_state(&mut self) -> &AnyMap {
        self.eval_state
    }

    #[must_use]
    pub fn annotate<'v>(&self, annotation: Option<Annotation<'v>>) -> Output<'v> {
        self.output(Ok(annotation), false)
    }

    pub fn error<'v, E: 'v + Error<'v>>(&self, error: E) -> Output<'v> {
        self.output(Err(Some(Box::new(error))), false)
    }
    pub fn transient<'v>(
        &self,
        is_valid: bool,
        nodes: impl IntoIterator<Item = Output<'v>>,
    ) -> Output<'v> {
        let op = if is_valid { Ok(None) } else { Err(None) };
        let mut output = self.output(op, true);
        output.append(nodes.into_iter());
        output
    }

    fn output<'v>(
        &self,
        annotation_or_error: AnnotationOrError<'v>,
        is_transient: bool,
    ) -> Output<'v> {
        Output::new(
            self.structure,
            self.absolute_keyword_location.clone(),
            self.keyword_location.clone(),
            self.instance_location.clone(),
            annotation_or_error,
            is_transient,
        )
    }
    /// Returns `true` if the evaluation should short-circuit, i.e. if the
    /// [`Structure`] is [`Flag`](`crate::Structure::Flag`).
    #[must_use]
    pub fn should_short_circuit(&self) -> bool {
        self.structure.is_flag()
    }
}

#[cfg(test)]
mod tests {}
