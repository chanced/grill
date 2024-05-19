use std::error::Error;
use std::fmt::{self};
use std::ops::ControlFlow;

use crate::error::{
    AnchorError, CompileError, DialectError, EvaluateError, IdentifyError, RefsError,
};
use crate::schema::Anchor;
use crate::{
    cache::{Numbers, Values},
    schema::{Dialects, Schemas},
    source::{Deserializers, Resolvers, Sources},
};
use crate::{Schema, FALSE, TRUE};
use grill_uri::{AbsoluteUri, Uri};
use jsonptr::{Pointer, Token};
use serde::Deserialize;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use slotmap::Key;

use std::fmt::Debug;
// Output determines the granularity of a [`Report`].
pub trait Output: Copy + Clone + fmt::Debug + Serialize + DeserializeOwned {
    fn verbose() -> Self;
}

pub trait Report<'v>: Clone + Error + Serialize + Deserialize<'v> {
    type Output: 'static + Output;
    type Owned: 'static + Report<'static, Output = Self::Output>;
    fn into_owned(self) -> Self::Owned;

    fn new<'i, L, K>(output: Self::Output, schema: &Schema<'i, L, K>) -> Self
    where
        L: Language<K>,
        K: 'static + Key;

    fn is_valid(&self) -> bool;
}

#[derive(Debug)]
pub struct NewContext<'i, 'v, 'r, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    pub eval_numbers: &'i mut Numbers,
    pub global_numbers: &'i Numbers,
    pub schemas: &'i Schemas<L, K>,
    pub sources: &'i Sources,
    pub dialects: &'i Dialects<L, K>,
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
}

pub struct NewCompile<'i, L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// AbsoluteUri of the Schema being compiled
    pub absolute_uri: &'i AbsoluteUri,
    /// Global cache of numbers
    pub global_numbers: &'i mut Numbers,
    /// Current collection of compiled schemas at the point of compilation
    pub schemas: &'i Schemas<L, K>,
    pub sources: &'i Sources,
    pub dialects: &'i Dialects<L, K>,
    pub resolvers: &'i Resolvers,
    pub deserializers: &'i Deserializers,
    pub values: &'i mut Values,
}

impl<'i, L, K> Debug for NewCompile<'i, L, K>
where
    L: Language<K>,
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

pub trait Context<'i, 'v, 'r, L, K>: Debug {}
pub trait Compile<'i>: Debug {}

pub trait Language<K>: Sized + Clone + Debug
where
    K: 'static + Key,
{
    type Context<'i, 'v, 'r>: Context<'i, 'v, 'r, Self, K>
    where
        Self: 'i,
        'v: 'r;

    type Compile<'i>: Compile<'i>
    where
        Self: 'i;

    type Keyword: Keyword<Self, K>;
    /// This should be the same type as `Report`. The additional associated type
    /// is to specify that `into_owned` returns the same type as `Report` but
    /// with an owned lifetime since there is no way to indicate that
    /// `into_owned` (of `Report`) should return `Self<'static>`.
    type OwnedReport: Report<'static>;
    type Report<'v>: Report<'v, Owned = <Self as Language<K>>::OwnedReport>;

    fn new_compile<'i>(&mut self, new_compile: NewCompile<'i, Self, K>) -> Self::Compile<'i>;
    fn new_context<'i, 'v, 'r>(
        &self,
        new_context: NewContext<'i, 'v, 'r, Self, K>,
    ) -> Self::Context<'i, 'v, 'r>;
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
pub trait Keyword<L, K>: 'static + Send + Sync + Clone + fmt::Debug
where
    L: Language<K>,
    K: 'static + Key,
{
    fn compile<'i, 'c>(
        &'i mut self,
        compile: &'c mut L::Compile<'i>,
        schema: Schema<'i, L, K>,
    ) -> Result<ControlFlow<()>, CompileError<L, K>>;

    /// Executes the keyword logic for the given [`Schema`] and [`Value`].
    fn evaluate<'i, 'c, 'v, 'r>(
        &'i self,
        ctx: &'c mut L::Context<'i, 'v, 'r>,
        value: &'v Value,
    ) -> Result<(), EvaluateError<K>>;

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
    ) -> ControlFlow<(), Result<Option<AbsoluteUri>, DialectError>> {
        ControlFlow::Break(())
    }

    /// Returns a list of [`Ref`](`crate::schema::Ref`)s to other
    /// schemas that `schema` depends on.
    fn refs(&self, schema: &Value) -> ControlFlow<(), Result<Vec<Ref>, RefsError>> {
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
║                            paths_of_object                            ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                           ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// Returns a [`Vec`] of [`Pointer`]s to the fields of the object at `field`.
///
/// TODO: This needs to be updated - it should accept `&Map<String, Value> without the `field` parameter
#[must_use]
#[deprecated(
    note = "This needs to be updated - it should accept `&Map<String, Value> without the `field` parameter"
)]
pub fn paths_of_obj_fields(field: &'static str, object: &Value) -> Vec<Pointer> {
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

/// A reference to a schema, returned from [`Keyword::refs`]. This is used to
/// resolve the reference to the actual schema.
///
/// See [`Reference`] for use that is not implementing the [`Keyword`] trait.
#[derive(Debug, Clone)]
pub struct Ref {
    /// the parsed [`Uri`] value.
    pub uri: Uri,
    /// the keyword of the reference (i.e. $ref)
    pub keyword: &'static str,
}
