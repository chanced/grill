use crate::{
    error::{AnchorError, CompileError, EvaluateError, IdentifyError, RefError},
    schema::{Anchor, Ref},
    uri::{AbsoluteUri, Uri},
    Schema,
};
use jsonptr::{Pointer, Token};
use serde_json::Value;
use std::fmt::{self, Display};

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
pub trait Keyword<L, Key>: Send + Sync + Clone + fmt::Debug
where
    L: crate::Language<Key>,
    Key: crate::Key,
{
    type Context;
    type Compile;
    type Output: crate::Output;

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
        compile: &mut Self::Compile,
        schema: Schema<'i, Self>,
    ) -> Result<bool, CompileError<L, Key>>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Self::Context,
        value: &'v Value,
    ) -> Result<Option<Self::Output>, EvaluateError<Key>>;

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
