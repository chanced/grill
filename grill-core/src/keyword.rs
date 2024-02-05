/// # Cache collections for [`Value`](`serde_json::Value`)s and [`BigRational`]s.
pub mod cache;

use num_rational::BigRational;

use self::cache::{Numbers, Values};

use crate::{
    anymap::AnyMap,
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, NumberError, RefError},
    output::{self, Annotation, AnnotationOrError, BoxedError, Translator},
    schema::{Anchor, Ref, Schemas},
    source::Sources,
    AbsoluteUri, Key, Output, Schema, Structure, Uri,
};
use jsonptr::{Pointer, Token};
use serde_json::{Number, Value};
use std::{
    any::Any,
    collections::HashSet,
    fmt::{self, Display},
    sync::Arc,
};

/// A static reference to [`Value::Bool`] with the value `true`
pub const TRUE: &Value = &Value::Bool(true);
/// A static reference to [`Value::Bool`] with the value `false`
pub const FALSE: &Value = &Value::Bool(false);

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
pub trait Keyword: Send + Sync + Clone + fmt::Debug {
    type Context;
    type Output;

    /// The [`Kind`] of the keyword. `Kind` can be either `Single`, which will
    /// be the name of the keyword or `Composite`, which will be a list of
    /// keywords.
    fn kind(&self) -> Kind;

    /// Each `Schema` compiled by the [`Interrogator`](`crate::Interrogator`)
    /// that has a [`Dialect`](`crate::schema::Dialect`) containing a fresh copy
    /// of this `Keyword` will call `setup` with the `Schema` and `Compile`
    /// context.
    ///
    /// If the keyword is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that
    /// [`evaluate`](`Keyword::evaluate`) should not be called for the given
    /// [`Schema`].
    fn compile<'i>(
        &mut self,
        compile: &mut Compile<'i>,
        schema: Schema<'i>,
    ) -> Result<bool, CompileError>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Self::Context,
        value: &'v Value,
    ) -> Result<Option<output::Output<'v>>, EvaluateError>;

    /// Sets the default `Translate` if available in the [`Translator`]
    fn set_translate(&mut self, translator: &Translator) -> Result<(), Unimplemented> {
        Err(Unimplemented)
    }
    /// Returns the paths to subschemas that this `Keyword` is aware of.
    fn subschemas(&self, schema: &Value) -> Result<Vec<Pointer>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    fn anchors(&self, schema: &Value) -> Result<Result<Vec<Anchor>, AnchorError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Attempts to identify the schema based on the [`Dialect`](`crate::schema::Dialect`).
    ///
    /// # Convention
    /// At least `Keyword` must implement the method `identify` for a given
    /// `Dialect`.
    ///
    /// # Example
    /// ```rust
    /// use serde_json::json;
    /// use grill::{ uri::AbsoluteUri, keyword::Keyword, json_schema::keyword::id::Id };
    ///
    /// let id_keyword = Id::new("$id", false);
    /// let id = id_keyword.identify(&json!({"$id": "https://example.com/schema.json" }))
    ///     .unwrap()  // unwraps `Result<Result<Option<Identifier>, IdentifyError>, Unimplemented>`
    ///     .unwrap()  // unwraps `Result<Option<Identifier>, Identifier>`
    ///     .unwrap(); // unwraps `Option<Identifier>`
    /// assert_eq!(&id, &AbsoluteUri::parse("https://example.com/schema.json").unwrap());
    /// ```
    ///
    fn identify(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<Uri>, IdentifyError>, Unimplemented> {
        Err(Unimplemented)
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement the `dialect` method for a given
    /// `Dialect`.
    ///()
    /// # Example
    /// ```
    /// use serde_json::json;
    /// use grill::keyword::Keyword as _;
    /// use std::borrow::Cow;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let schema = json!({ "$schema": draft });
    /// let schema_keyword = grill::json_schema::keyword::schema::Schema::new("$schema", false);
    /// let dialect = schema_keyword.dialect(&schema).unwrap().unwrap().unwrap();
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

/// Returns a static reference to [`Value::Bool`] with the given value.
#[must_use]
pub const fn boolean(value: bool) -> &'static Value {
    if value {
        TRUE
    } else {
        FALSE
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

/// Contains global state, evaluation level state, schemas, and location
/// information needed to [`evaluate`](`crate::Interrogator::evaluate`) a
/// schema.
pub struct Context<'i> {
    pub(crate) absolute_keyword_location: &'i AbsoluteUri,
    pub(crate) keyword_location: Pointer,
    pub(crate) instance_location: Pointer,
    pub(crate) structure: Structure,
    pub(crate) schemas: &'i Schemas,
    pub(crate) sources: &'i Sources,
    pub(crate) global_numbers: &'i Numbers,
    pub(crate) eval_numbers: &'i mut Numbers,
}

impl<'s> Context<'s> {
    /// Evaluates `value` against the schema with the given `key` for the
    /// `keyword` and produces an [`Output`]
    pub fn evaluate<'v>(
        &mut self,
        key: Key,
        instance: Option<&str>,
        keyword: &Pointer,
        value: &'v Value,
    ) -> Result<Output<'v>, EvaluateError> {
        if self.absolute_keyword_location().host().as_deref() != Some("json-schema.org") {
            // println!("{}", self.absolute_keyword_location());
            // println!("{}", serde_json::to_string_pretty(&value).unwrap());
        }
        let mut instance_location = self.instance_location.clone();
        if let Some(instance) = instance {
            instance_location.push_back(instance.into());
        }
        self.evaluated.insert(instance_location.to_string());
        let mut keyword_location = self.keyword_location.clone();
        keyword_location.append(keyword);
        self.schemas.evaluate(
            self.structure,
            key,
            value,
            instance_location,
            keyword_location,
            self.sources,
            self.evaluated,
            self.global_state,
            self.eval_state,
            self.global_numbers,
            self.eval_numbers,
        )
    }
    /// Either returns a reference to a previously parsed [`BigRational`] or
    /// parses, stores the [`BigRational`] as an [`Arc`] (per eval) and returns
    /// a reference to the [`BigRational`].
    ///
    /// # Errors
    /// Returns [`NumberError`] if the number fails to parse
    pub fn number_ref(&mut self, number: &Number) -> Result<&BigRational, NumberError> {
        if let Some(n) = self.global_numbers.get_ref(number) {
            return Ok(n);
        }
        self.eval_numbers.get_or_insert_ref(number)
    }
    /// Either returns a [`Arc`] to a previously parsed [`BigRational`] or
    /// parses, stores (per eval) and returns an [`Arc`] to the [`BigRational`].
    ///
    /// # Errors
    /// Returns [`NumberError`] if the number fails to parse
    pub fn number_arc(&mut self, number: &Number) -> Result<Arc<BigRational>, NumberError> {
        if let Some(n) = self.global_numbers.get_arc(number) {
            return Ok(n);
        }
        self.eval_numbers.get_or_insert_arc(number)
    }
    #[must_use]
    pub fn absolute_keyword_location(&self) -> &AbsoluteUri {
        self.absolute_keyword_location
    }

    /// Evaluates `value` against the schema with the given `key` but does not
    /// mark the instance as evaluated.
    ///
    /// This is intended for use with `if` but may be used
    /// in other cases.
    pub fn probe<'v>(
        &mut self,
        key: Key,
        instance: Option<&str>,
        keyword: &Pointer,
        value: &'v Value,
    ) -> Result<Output<'v>, EvaluateError> {
        let mut instance_location = self.instance_location.clone();
        if let Some(instance) = instance {
            instance_location.push_back(instance.into());
        }
        let mut keyword_location = self.keyword_location.clone();
        keyword_location.append(keyword);
        self.schemas.evaluate(
            self.structure,
            key,
            value,
            instance_location,
            keyword_location,
            self.sources,
            self.evaluated,
            self.global_state,
            self.eval_state,
            self.global_numbers,
            self.eval_numbers,
        )
    }

    /// Mutable reference to the eval local state [`AnyMap`].
    ///
    /// This does not include the [`global_state`](`Context::global_state`).
    #[must_use]
    pub fn global_state(&self) -> &AnyMap {
        self.global_state
    }

    /// Mutable reference to the eval local state [`AnyMap`].
    ///
    /// This does not include the [`global_state`](`Context::global_state`).
    pub fn eval_state(&mut self) -> &mut AnyMap {
        self.eval_state
    }

    /// creates a valid [`Output`] with the given `keyword` and `annotation`
    #[must_use]
    pub fn annotate<'v>(
        &mut self,
        keyword: Option<&'static str>,
        annotation: Option<Annotation<'v>>,
    ) -> Output<'v> {
        self.create_output(keyword, Ok(annotation), false)
    }

    /// Creates an invalid [`Output`] with the given `keyword` and `error`
    pub fn error<'v>(
        &mut self,
        keyword: Option<&'static str>,
        error: Option<BoxedError<'v>>,
    ) -> Output<'v> {
        self.create_output(keyword, Err(error), false)
    }

    /// Creates a transient [`Output`] with the given `keyword` and `nodes`
    ///
    /// A transient `Output` is one which is not included in the final output
    /// but accumulates errors and annotations, which are then flattened into a
    /// series of `Output`s which are included in the final output without
    /// having their conjunction considered.
    ///
    /// Essentially, a transient `Output` is a pseudo node which has its state
    /// determined by the `Keyword` rather than the result of it's children.
    ///
    /// The transient `Output` is removed from the final output, promoting the
    /// `nodes` to the same level as the transient `Output`.
    pub fn transient<'v>(
        &mut self,
        is_valid: bool,
        nodes: impl IntoIterator<Item = Output<'v>>,
    ) -> Output<'v> {
        let op = if is_valid { Ok(None) } else { Err(None) };
        let mut output = self.create_output(None, op, true);
        output.append(nodes.into_iter());
        output.set_valid(is_valid);
        output
    }

    fn create_output<'v>(
        &mut self,
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
                .fragment_decoded_lossy()
                .unwrap_or_default()
                .parse::<Pointer>()
            {
                ptr.push_back(tok);
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

    /// Returns a mutable reference to the [`HashSet`] of evaluated instances.
    #[must_use]
    pub fn evaluated_mut(&mut self) -> &mut HashSet<String> {
        self.evaluated
    }

    /// Returns an immutable reference to the [`HashSet`] of evaluated instances
    #[must_use]
    pub fn evaluated(&self) -> &HashSet<String> {
        self.evaluated
    }

    /// Returns `true` if the instance location has been evaluated.
    #[must_use]
    pub fn has_evaluated(&self, instance: &str) -> bool {
        let mut instance_location = self.instance_location.clone();
        instance_location.push_back(instance.into());
        self.evaluated.contains(&self.instance_location.to_string())
    }

    /// Returns `true` if enabling short-circuiting was successful or if it
    /// was previously set to `true`.
    pub fn enable_short_circuiting(&mut self) -> bool {
        if let Some(should_short_circuit) = self.should_short_circuit {
            should_short_circuit
        } else {
            self.should_short_circuit = Some(true);
            true
        }
    }
    /// Disables short-circuiting
    pub fn disable_short_circuiting(&mut self) {
        self.should_short_circuit = Some(false);
    }

    /// Returns `true` if the evaluation should short-circuit, as determined
    /// by the [`ShortCircuit`](grill_json_schema::keyword::short_circuit::ShortCircuit) keyword handler
    #[must_use]
    pub fn should_short_circuit(&self) -> bool {
        self.should_short_circuit.unwrap_or(false)
    }

    /// Returns the desired [`Structure`] of the evaluation
    #[must_use]
    pub fn structure(&self) -> Structure {
        self.structure
    }
}
/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                          static_pointer_fn!                           ║
║                         ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Generates a static function which returns a [`Pointer`] to the given path.
/// # Example
/// ```no_run
/// static_pointer_fn!(pub if "/if");
/// assert_eq!(if_pointer(), &Pointer::new(["if"]));
/// ```
///
#[macro_export]
macro_rules! static_pointer_fn {
    ($vis:vis $ident:ident $path:literal) => {
        paste::paste! {
            #[doc = "Returns a static [`Pointer`] to \"" $path "\""]
            pub fn [< $ident _pointer >]() -> &'static jsonptr::Pointer {
                use ::once_cell::sync::Lazy;
                static POINTER: Lazy<jsonptr::Pointer> = Lazy::new(|| jsonptr::Pointer::parse($path).unwrap());
                &POINTER
            }
        }
    };
}

pub use static_pointer_fn;

/// Generates an `as_<Keyword>` and `is_<Keyword>` fn for the given `Keyword` type.
#[macro_export]
macro_rules! keyword_fns {
    ($keyword:ident) => {
        paste::paste! {
            #[doc= "Attempts to downcast `keyword` as `" $keyword "`"]
            pub fn [< as_ $keyword:snake >](keyword: &dyn ::std::any::Any) -> Option<&$keyword> {
                keyword.downcast_ref::<$keyword>()
            }
            #[doc= "Returns `true` if `keyword` is an instance of `" $keyword "`"]
            pub fn [< is_ $keyword:snake >](keyword: &dyn $crate::keyword::Keyword) -> bool {
                ::std::any::TypeId::of::<$keyword>() == keyword.type_id()
            }

        }
    };
}

pub use keyword_fns;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           define_translate!                           ║
║                          ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Generates an `enum` which contains implements [`Translate`](crate::output::Translate) for a given
/// [`Error`](`crate::output::Error`).
///
/// The variants are either a `fn` pointer or `Fn` closure wrapped in an `Arc`.
///
/// Note: requires the [`inherent`](https://docs.rs/inherent/latest/inherent/) crate.
#[macro_export]
macro_rules! define_translate {
    ($error:ident, $default:ident) => {
        paste::paste!{
            /// A function which can translate [`$error`].
            #[derive(Clone)]
            pub enum [< Translate $error >]{
                #[doc= "A closure `Fn` wrapped in an `Arc` that can translate [`" $error "`]."]
                Closure(
                    ::std::sync::Arc<
                        dyn Send + Sync + Fn(&mut ::std::fmt::Formatter, &$error) -> ::std::fmt::Result,
                    >,
                ),
                #[doc = "A `fn` which can translate [`" $error "`]"]
                FnPtr(fn(&mut ::std::fmt::Formatter, &$error) -> std::fmt::Result),
            }

            #[::inherent::inherent]
            impl grill_core::output::Translate<$error<'_>> for [< Translate $error>]{
                /// Runs the translation
                pub fn run(&self, f: &mut ::std::fmt::Formatter, v: &$error) -> ::std::fmt::Result {
                    match self {
                        Self::Closure(c) => c(f, v),
                        Self::FnPtr(p) => p(f, v),
                    }
                }
            }
            impl ::std::fmt::Debug for [< Translate $error >] {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    match self {
                        Self::Closure(_) => f.debug_tuple("Closure").finish(),
                        Self::FnPtr(_) => f.debug_tuple("Pointer").finish(),
                    }
                }
            }
            impl std::default::Default for [< Translate $error >] {
                fn default() -> Self {
                    Self::FnPtr($default)
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

/// Context for compilation of the [`Keyword`]
#[derive(Debug)]
pub struct Compile<'i, GlobalState> {
    pub(crate) absolute_uri: &'i AbsoluteUri,
    pub(crate) schemas: &'i Schemas,
    pub(crate) numbers: &'i mut Numbers,
    pub(crate) value_cache: &'i mut Values,
}

impl<'i> Compile<'i, GlobalState> {
    #[must_use]
    /// The [`AbsoluteUri`] of the [`Schema`]
    pub fn absolute_uri(&self) -> &AbsoluteUri {
        self.absolute_uri
    }

    /// Parses a [`Number`] into a [`BigRational`], stores it and returns an
    /// `Arc` to it.
    ///
    /// # Errors
    /// Returns `NumberError` if the number fails to parse
    pub fn number(&mut self, num: &Number) -> Result<Arc<BigRational>, NumberError> {
        self.numbers.get_or_insert_arc(num)
    }
    /// Caches a [`Value`] and returns an `Arc` to it.
    pub fn value(&mut self, value: &Value) -> Arc<Value> {
        self.value_cache.value(value)
    }

    /// Returns a mutable reference to the global state [`AnyMap`].
    #[must_use]
    pub fn global_state_mut(&mut self) -> &mut GlobalState {
        self.global_state
    }

    /// Resolves a schema `Key` by URI
    ///
    /// # Errors
    /// - `CompileError::SchemaNotFound` if the schema is not found
    /// - `CompileError::UriParsingFailed` if the URI is invalid
    pub fn schema(&self, uri: &str) -> Result<Key, CompileError> {
        let uri: Uri = uri.parse()?;
        let uri = self.absolute_uri.with_fragment(None)?.resolve(&uri)?;
        self.schemas
            .get_key(&uri)
            .ok_or(CompileError::SchemaNotFound(uri))
    }

    /// Returns the [`Key`] of a schema at the specified `path` relative to
    /// the current schema.
    ///
    /// # Errors
    /// Returns a [`CompileError`] if the schema cannot be found.
    pub fn subschema(&self, path: &Pointer) -> Result<Key, CompileError> {
        let mut uri = self.absolute_uri().clone();

        if let Some(fragment) = uri.fragment_decoded_lossy() {
            let mut ptr = fragment.parse::<Pointer>()?;
            ptr.append(path);
            uri.set_fragment(Some(&ptr))?;
        } else {
            uri.set_fragment(Some(path))?;
        }
        self.schemas
            .get_key(&uri)
            .ok_or(CompileError::SchemaNotFound(uri))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                            paths_of_object                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Returns a [`Vec`] of [`Pointer`]s to the fields of the object at `field`.
#[must_use]
pub fn paths_of_object(field: &'static str, object: &Value) -> Vec<Pointer> {
    let Some(Value::Object(props)) = object.get(field) else {
        return Vec::new();
    };
    let base = Pointer::new([field]);
    props
        .keys()
        .map(|k| {
            let mut ptr = base.clone();
            ptr.push_back(Token::from(k));
            ptr
        })
        .collect()
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

/// Indicates that the specific method of [`Keyword`] is not implemented.
///
/// This enables the `Dialect` to reduce the list of `Keyword`s to call
/// for any given method down to those which are pertinent.
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

/// Indicates the type of [`Keyword`] and the keyword(s) which it handles.
#[derive(Clone, Debug, PartialEq, Eq, Copy)]
pub enum Kind {
    /// The [`Keyword`] is singular, evaluating the logic of a specific
    /// JSON Schema Keyword.
    Keyword(&'static str),
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
        if let Kind::Keyword(s) = self {
            s == other
        } else {
            false
        }
    }
}

impl From<&'static str> for Kind {
    fn from(s: &'static str) -> Self {
        Kind::Keyword(s)
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
║                                 Tests                                 ║
║                                 ¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
