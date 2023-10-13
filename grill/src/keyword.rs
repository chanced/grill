mod compile;
pub use compile::Compile;

pub mod cache;

use crate::{
    anymap::AnyMap,
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, RefError},
    output::{self, Annotation, AnnotationOrError, Error, Translator},
    schema::{Anchor, Identifier, Ref, Schemas},
    source::Sources,
    AbsoluteUri, Key, Output, Schema, Structure,
};
pub use cache::{Numbers, Values};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;
use std::fmt::{self, Display};

#[macro_export]
macro_rules! define_translate {
    ($error:ident) => {
        #[derive(Clone)]
        pub enum Translate {
            Closure(
                ::std::sync::Arc<
                    dyn Send + Sync + Fn(&mut ::std::fmt::Formatter, &$error) -> ::std::fmt::Result,
                >,
            ),
            Pointer(fn(&mut ::std::fmt::Formatter, &$error) -> std::fmt::Result),
        }
        impl Translate {
            pub fn run(&self, f: &mut ::std::fmt::Formatter, v: &$error) -> ::std::fmt::Result {
                match self {
                    Translate::Closure(c) => c(f, v),
                    Translate::Pointer(p) => p(f, v),
                }
            }
        }
        impl ::std::fmt::Debug for Translate {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    Self::Closure(_) => f.debug_tuple("Closure").finish(),
                    Self::Pointer(_) => f.debug_tuple("Pointer").finish(),
                }
            }
        }
    };
}

pub use define_translate;

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
    pub fn evalute<'v>(
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
        self.schemas.evaluate(
            self.structure,
            key,
            value,
            instance_location,
            keyword_location,
            self.sources,
            self.global_state,
            self.eval_state,
        )
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

#[derive(Debug)]
pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not implemented")
    }
}

#[derive(Clone, Debug, Copy)]
pub enum Kind {
    /// The [`Keyword`] is singular, evaluating the logic of a specific
    /// JSON Schema Keyword.
    Single(&'static str),
    /// The [`Keyword`] is a composite of multiple keywords with additional
    /// logic which handles co-dependencies between the embedded keywords.
    ///
    /// The output of this keyword should be a transient
    /// [`Output`](`crate::Output`), with `is_transient` set to `true`.
    /// Depending on the specified `Structure`, the [`Output`] may be expanded
    /// into multiple nodes.
    Composite(&'static [&'static str]),
}

impl PartialEq<&str> for Kind {
    fn eq(&self, other: &&str) -> bool {
        if let Kind::Single(s) = self {
            s == other
        } else {
            false
        }
    }
}

impl From<&'static str> for Kind {
    fn from(s: &'static str) -> Self {
        Kind::Single(s)
    }
}
impl From<&'static [&'static str]> for Kind {
    fn from(s: &'static [&'static str]) -> Self {
        Kind::Composite(s)
    }
}

/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
#[allow(unused_variables)]
pub trait Keyword: Send + Sync + DynClone + fmt::Debug {
    /// The [`Kind`] of the keyword. `Kind` can be either `Single`, which will
    /// be the name of the keyword or `Composite`, which will be a list of
    /// keywords.
    fn kind(&self) -> Kind;

    /// For each `Schema` compiled by the [`Interrogator`], this `Keyword` is
    /// cloned and [`setup`] is called.
    ///
    /// If the keyword is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`evaluate`](`Self::evaluate`) should not
    /// be called for the given [`Schema`].
    fn setup<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        value: &'v Value,
    ) -> Result<Option<output::Output<'v>>, EvaluateError>;

    /// Sets the default `Translate` if available in the [`Translator`]
    fn set_translate(&mut self, translator: &Translator) -> Result<(), Unimplemented> {
        Err(Unimplemented)
    }

    fn subschemas(&self, schema: &Value) -> Result<Vec<Pointer>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    fn anchors(&self, schema: &Value) -> Result<Result<Vec<Anchor>, AnchorError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// At least `Keyword` must implement the method `identify` for a given `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::keywords::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json" }));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    fn identify(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<Identifier>, IdentifyError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the `dialect` method for a given
    /// `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaKeyword;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaKeyword.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    fn dialect(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<AbsoluteUri>, IdentifyError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    fn refs(&self, schema: &Value) -> Result<Result<Vec<Ref>, RefError>, Unimplemented> {
        Err(Unimplemented)
    }
}

clone_trait_object!(Keyword);

#[cfg(test)]
mod tests {
    use crate::anymap::AnyMap;

    #[test]
    fn test_get() {
        let mut state = AnyMap::new();
        let i: i32 = 1;
        state.insert(i);
        let x = state.get_mut::<i32>().unwrap();
        *x += 1;

        assert_eq!(state.get::<i32>(), Some(&2));
    }
}
