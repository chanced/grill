

use std::fmt::{self};
use std::ops::{ControlFlow};

use crate::error::{
    AnchorError, CompileError, EvaluateError, IdentifyError, RefError,
};
use crate::schema::{Anchor, Ref};
use crate::{
    cache::{Numbers, Values},
    schema::{
        Dialects, Schemas,
    },
    source::{Deserializers, Resolvers, Sources},
    uri::{AbsoluteUri},
};
use crate::{Schema, FALSE, TRUE};
use grill_uri::Uri;
use jsonptr::{Pointer, Token};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Value};
use slotmap::Key;

use std::fmt::Debug;
// Output determines the granularity of a [`Report`].
pub trait Output: Copy + Clone + fmt::Debug + Serialize + DeserializeOwned {
    fn verbose() -> Self;
}

/// The result of a keyword's evaluation.
pub enum Assessment<A, E> {
    Annotation(Option<A>),
    Error(Option<E>),
}

/// Type alias for the associated type `Report` of the given `Criterion` `C`.
pub type CriterionReport<'v, C, K> = <C as Criterion<K>>::Report<'v>;
pub type CriterionOwnedReport<C, K> = <C as Criterion<K>>::OwnedReport;
/// Type alias for the associated type `Output` of the `Report` associated with
/// the given `Criterion` `C`.
pub type CriterionReportOutput<'v, C, K> = <CriterionReport<'v, C, K> as Report<'v>>::Output;

pub trait Report<'v>: Clone + std::error::Error + Serialize + DeserializeOwned {
    type Error<'e>: 'e + Serialize + DeserializeOwned;
    type Annotation<'a>: 'a + Default + Serialize + DeserializeOwned;
    type Output: 'static + Output;
    type Owned: 'static
        + Report<
            'static,
            Error<'static> = Self::Error<'static>,
            Annotation<'static> = Self::Annotation<'static>,
            Output = Self::Output,
        >;
    fn into_owned(self) -> Self::Owned;

    fn new(
        output: Self::Output,
        absolute_keyword_location: &AbsoluteUri,
        keyword_location: Pointer,
        instance_location: Pointer,
        assessment: Assessment<Self::Annotation<'v>, Self::Error<'v>>,
    ) -> Self;

    fn is_valid(&self) -> bool;
    fn append(&mut self, nodes: impl Iterator<Item = Self>);
    fn push(&mut self, output: Self);
}

#[derive(Debug)]
pub struct NewContext<'i, 'v, 'r, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    pub report: &'r mut CriterionReport<'v, C, K>,
    pub eval_numbers: &'i mut Numbers,
    pub global_numbers: &'i Numbers,
    pub schemas: &'i Schemas<C, K>,
    pub sources: &'i Sources,
}

pub struct NewCompile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    /// AbsoluteUri of the Schema being compiled
    pub absolute_uri: &'i AbsoluteUri,
    /// Global cache of numbers
    pub global_numbers: &'i mut Numbers,
    /// Current collection of compiled schemas at the point of compilation
    pub schemas: &'i Schemas<C, K>,
    pub sources: &'i Sources,
    pub dialects: &'i Dialects<C, K>,
    pub resolvers: &'i Resolvers,
    pub deserializers: &'i Deserializers,
    pub values: &'i mut Values,
}

impl<'i, C, K> Debug for NewCompile<'i, C, K>
where
    C: Criterion<K>,
    K: 'static + Key,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewCompile")
            .field("absolute_uri", &self.absolute_uri)
            .field("global_numbers", &self.global_numbers)
            .field("schemas", &self.schemas)
            .field("sources", &self.sources)
            .field("dialects", &self.dialects)
            // .field("resolvers", &self.resolvers)
            .field("deserializers", &self.deserializers)
            .field("values", &self.values)
            .finish_non_exhaustive()
    }
}

pub trait Context<'i, C, K>: Debug {}
pub trait Compile<'i>: Debug {}

pub trait Criterion<K>: Sized + Clone + Debug
where
    K: 'static + Key,
{
    type Context<'i>: Context<'i, Self, K>
    where
        Self: 'i;
    type Compile<'i>: Compile<'i>
    where
        Self: 'i;

    type Keyword: Keyword<Self, K>;
    /// This should be the same type as `Report`. The additional associated type
    /// is to specify that `into_owned` returns the same type as `Report` but
    /// with an owned lifetime since there is no way to indicate that
    /// `into_owned` (of `Report`) should return `Self<'static>`.
    type OwnedReport: Report<'static>;
    type Report<'v>: Report<'v, Owned = <Self as Criterion<K>>::OwnedReport>;

    fn new_compile<'i>(&mut self, new_compile: NewCompile<'i, Self, K>) -> Self::Compile<'i>;
    fn new_context<'i, 'v, 'r>(
        &self,
        new_context: NewContext<'i, 'v, 'r, Self, K>,
    ) -> Self::Context<'i>;
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
pub trait Keyword<C, K>: 'static + Send + Sync + Clone + fmt::Debug
where
    C: Criterion<K>,
    K: 'static + Key,
{
    /// The [`Kind`] of the keyword. `Kind` can be either `Single`, which will
    /// be the name of the keyword or `Composite`, which will be a list of
    /// keywords.
    fn kind(&self) -> Kind;

    fn compile<'i, 'c>(
        &'i mut self,
        compile: &'c mut C::Compile<'i>,
        schema: Schema<'i, C, K>,
    ) -> Result<ControlFlow<()>, CompileError<C, K>>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    fn evaluate<'i, 'c, 'v>(
        &'i self,
        ctx: &'c mut C::Context<'i>,
        value: &'v Value,
    ) -> Result<Option<C::Report<'v>>, EvaluateError<K>>;

    /// Returns the paths to subschemas that this `Keyword` is aware of.
    fn subschemas(&self, schema: &Value) -> ControlFlow<(), Vec<Pointer>> {
        ControlFlow::Break(())
    }

    /// Returns a list of [`Anchor`]s which are handled by this `Keyword`
    fn anchors(&self, schema: &Value) -> ControlFlow<(), Result<Vec<Anchor>, AnchorError>> {
        ControlFlow::Break(())
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
    fn identify(&self, schema: &Value) -> ControlFlow<(), Result<Option<Uri>, IdentifyError>> {
        ControlFlow::Break(())
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema's dialect.
    fn dialect(
        &self,
        schema: &Value,
    ) -> ControlFlow<(), Result<Option<AbsoluteUri>, IdentifyError>> {
        ControlFlow::Break(())
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    fn refs(&self, schema: &Value) -> ControlFlow<(), Result<Vec<Ref>, RefError>> {
        ControlFlow::Break(())
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
            pub fn [< is_ $keyword:snake >](keyword: &dyn $crate::criterion::Keyword) -> bool {
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

// /// Generates an `enum` which contains implements [`Translate`](crate::output::Translate) for a given
// /// [`Error`](`crate::output::Error`).
// ///
// /// The variants are either a `fn` pointer or `Fn` closure wrapped in an `Arc`.
// ///
// /// Note: requires the [`inherent`](https://docs.rs/inherent/latest/inherent/) crate.
// #[macro_export]
// macro_rules! define_translate {
//     ($error:ident, $default:ident) => {
//         paste::paste!{
//             /// A function which can translate [`$error`].
//             #[derive(Clone)]
//             pub enum [< Translate $error >]{
//                 #[doc= "A closure `Fn` wrapped in an `Arc` that can translate [`" $error "`]."]
//                 Closure(
//                     ::std::sync::Arc<
//                         dyn Send + Sync + Fn(&mut ::std::fmt::Formatter, &$error) -> ::std::fmt::Result,
//                     >,
//                 ),
//                 #[doc = "A `fn` which can translate [`" $error "`]"]
//                 FnPtr(fn(&mut ::std::fmt::Formatter, &$error) -> std::fmt::Result),
//             }

//             #[::inherent::inherent]
//             impl grill_core::criterion::Translate<$error<'_>> for [< Translate $error>]{
//                 /// Runs the translation
//                 pub fn run(&self, f: &mut ::std::fmt::Formatter, v: &$error) -> ::std::fmt::Result {
//                     match self {
//                         Self::Closure(c) => c(f, v),
//                         Self::FnPtr(p) => p(f, v),
//                     }
//                 }
//             }
//             impl ::std::fmt::Debug for [< Translate $error >] {
//                 fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
//                     match self {
//                         Self::Closure(_) => f.debug_tuple("Closure").finish(),
//                         Self::FnPtr(_) => f.debug_tuple("Pointer").finish(),
//                     }
//                 }
//             }
//             impl std::default::Default for [< Translate $error >] {
//                 fn default() -> Self {
//                     Self::FnPtr($default)
//                 }
//             }
//         }
//     };
// }

// pub use define_translate;

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

impl fmt::Display for Unimplemented {
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
    /// logic that handles co-dependencies between the embedded keywords.
    ///
    /// The output of this keyword should be a transient
    /// [`Output`](`crate::Output`), with `is_transient` set to `true`.
    /// Depending on the specified `Output`, the [`Report`] may be expanded
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
