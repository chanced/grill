use ahash::AHashMap;
use num_rational::BigRational;

pub mod cache;

use self::cache::{Numbers, Values};

use crate::{
    anymap::AnyMap,
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, NumberError, RefError},
    output::{self, Annotation, AnnotationOrError, Error, Translator},
    schema::{Anchor, Identifier, Ref, Schemas},
    source::Sources,
    AbsoluteUri, Key, Output, Schema, Structure, Uri,
};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::{Pointer, Token};
use serde_json::{Number, Value};
use std::{
    fmt::{self, Display},
    sync::Arc,
};

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           define_translate!                           ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Compile                                ║
║                                ¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug)]
pub struct Compile<'i> {
    pub(crate) base_uri: &'i AbsoluteUri,
    pub(crate) schemas: &'i Schemas,
    pub(crate) numbers: &'i mut Numbers,
    pub(crate) value_cache: &'i mut Values,
    pub(crate) state: &'i mut AnyMap,
}

impl<'i> Compile<'i> {
    /// Parses a [`Number`] into a [`BigRational`], stores it and returns an
    /// `Arc` to it.
    ///
    /// # Errors
    /// Returns `NumberError` if the number fails to parse
    pub fn number(&mut self, num: &Number) -> Result<Arc<BigRational>, NumberError> {
        self.numbers.number(num)
    }
    /// Caches a [`Value`] and returns an `Arc` to it.
    pub fn value(&mut self, value: &Value) -> Arc<Value> {
        self.value_cache.value(value)
    }

    /// Returns an immutable reference to the global state [`AnyMap`].
    #[must_use]
    pub fn state(&self) -> &AnyMap {
        self.state
    }

    /// Returns a mutable reference to the global state [`AnyMap`].
    #[must_use]
    pub fn state_mut(&mut self) -> &mut AnyMap {
        self.state
    }

    /// Resolves a schema `Key` by URI
    ///
    /// # Errors
    /// - `CompileError::SchemaNotFound` if the schema is not found
    /// - `CompileError::UriParsingFailed` if the URI is invalid
    pub fn schema(&self, uri: &str) -> Result<Key, CompileError> {
        let uri: Uri = uri.parse()?;
        let uri = self.base_uri.resolve(&uri)?;
        self.schemas
            .get_key(&uri)
            .ok_or(CompileError::SchemaNotFound(uri))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Context                                ║
║                                ¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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
    pub(crate) evaluated: &'i mut Evaluated,
}

impl<'s> Context<'s> {
    pub fn evalute<'v>(
        &mut self,
        key: Key,
        instance: Option<&str>,
        keyword: &str,
        value: &'v Value,
    ) -> Result<Output<'v>, EvaluateError> {
        let mut instance_location = self.instance_location.clone();
        if let Some(instance) = instance {
            instance_location.push_back(instance.into());
        }
        let mut keyword_location = self.keyword_location.clone();
        keyword_location.push_back(keyword.into());
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
    pub fn annotate<'v>(
        &mut self,
        keyword: &'static str,
        annotation: Option<Annotation<'v>>,
    ) -> Output<'v> {
        self.evaluated.insert(&self.instance_location);
        self.output(Some(keyword), Ok(annotation), false)
    }

    pub fn error<'v, E>(&self, keyword: &'static str, error: E) -> Output<'v>
    where
        E: 'v + Error<'v>,
    {
        self.output(Some(keyword), Err(Some(Box::new(error))), false)
    }
    pub fn transient<'v>(
        &self,
        is_valid: bool,
        nodes: impl IntoIterator<Item = Output<'v>>,
    ) -> Output<'v> {
        let op = if is_valid { Ok(None) } else { Err(None) };
        let mut output = self.output(None, op, true);
        output.append(nodes.into_iter());
        output
    }

    fn output<'v>(
        &self,
        keyword: Option<&'static str>,
        annotation_or_error: AnnotationOrError<'v>,
        is_transient: bool,
    ) -> Output<'v> {
        let mut keyword_location = self.keyword_location.clone();
        let mut absolute_keyword_location = self.absolute_keyword_location.clone();

        if let Some(keyword) = keyword {
            let tok: Token = keyword.into();
            keyword_location.push_back(tok.clone());
            if let Ok(mut ptr) = absolute_keyword_location
                .fragment()
                .unwrap_or_default()
                .parse::<Pointer>()
            {
                ptr.push_back(tok);
                // TODO: this probably needs
                absolute_keyword_location.set_fragment(Some(&ptr)).unwrap();
            }
        }

        Output::new(
            self.structure,
            absolute_keyword_location,
            keyword_location,
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                             Unimplemented                             ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug)]
pub struct Unimplemented;

impl Display for Unimplemented {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not implemented")
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Kind                                  ║
║                                 ¯¯¯¯                                  ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Keyword                                ║
║                                ¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Evaluated                               ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A collection of evaluated paths in the form of a trie of JSON pointers.
///
/// # Example
/// ```
/// # use grill::keyword::Evaluated;
#[derive(Debug, Clone)]
pub struct Evaluated {
    children: AHashMap<String, Evaluated>,
}
impl Evaluated {
    #[must_use]
    pub fn new() -> Self {
        Self {
            children: AHashMap::new(),
        }
    }
    pub fn insert(&mut self, ptr: &Pointer) {
        let mut props = self;
        for tok in ptr.split('/') {
            props = props.children.entry(tok.to_string()).or_default();
        }
    }

    #[must_use]
    pub fn contains(&self, ptr: &Pointer) -> bool {
        let mut node = self;
        for tok in ptr.split('/') {
            if node.children.contains_key(tok) {
                node = node.children.get(tok).unwrap();
                continue;
            }
            return false;
        }
        true
    }
}

impl Default for Evaluated {
    fn default() -> Self {
        Self::new()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Tests                                 ║
║                                 ¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_evaluated_insert() {
        let ptr = Pointer::new(["a", "b", "c"]);
        let mut props = Evaluated::default();
        props.insert(&ptr);
        dbg!(&props);
        assert!(props.contains(&ptr));
        assert!(!props.contains(&Pointer::new(["a", "b", "d"])));
    }
}
