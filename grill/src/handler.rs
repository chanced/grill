use crate::{
    dialect::Dialects,
    error::{CompileError, EvaluateError, IdentifyError, UriError},
    output::{self, Structure},
    schema::{LocatedSchema, Reference},
    AbsoluteUri, Uri,
};
use async_trait::async_trait;
use big_rational_str::ParseError as BigRatParseError;
use dyn_clone::{clone_trait_object, DynClone};
use inherent::inherent;
use jsonptr::Pointer;
use num_rational::BigRational;
use serde_json::{Number, Value};
use slotmap::SlotMap;
use std::{
    any::{Any, TypeId},
    collections::{hash_map, HashMap},
    fmt,
    hash::{BuildHasherDefault, Hasher},
};

/// A handler that performs logic for a given condition in a JSON Schema.
#[derive(Debug, Clone)]
pub enum Handler {
    /// A synchronous handler.
    Sync(Box<dyn SyncHandler>),
    /// An asynchronous handler.
    Async(Box<dyn AsyncHandler>),
}

impl Handler {
    /// Returns `true` if the handler is [`Sync`].
    ///
    /// [`Sync`]: Handler::Sync
    #[must_use]
    pub fn is_sync(&self) -> bool {
        matches!(self, Self::Sync(..))
    }
    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn as_sync(&self) -> Option<&Box<dyn SyncHandler>> {
        if let Self::Sync(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the handler is [`Async`].
    ///
    /// [`Async`]: Handler::Async
    #[must_use]
    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async(..))
    }

    #[must_use]
    #[allow(clippy::borrowed_box)]
    pub fn as_async(&self) -> Option<&Box<dyn AsyncHandler>> {
        if let Self::Async(v) = self {
            Some(v)
        } else {
            None
        }
    }
    /// Attempts to identify the schema based on the [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `identify` for a given `Dialect`. It **must** be the
    /// **second** (index: `1`) `Handler` in the [`Dialect`](`crate::dialect::Dialect`)'s [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::IdHandler;
    ///
    /// let id = IdHandler.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".try_into().unwrap())));
    /// ```
    pub fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        match self {
            Handler::Sync(handler) => handler.identify(schema),
            Handler::Async(handler) => handler.identify(schema),
        }
    }
    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `is_pertinent_to` for a given `Dialect`.
    /// It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::SchemaHandler;
    ///
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({"$schema": "https://json-schema.org/draft/2020-12/schema"}));
    /// assert_eq!(is_pertinent_to, true);
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({"$schema": "https://json-schema.org/draft/2019-09/schema"}));
    /// assert_eq!(is_pertinent_to, false);
    /// ```
    #[must_use]
    pub fn is_pertinent_to(&self, value: &Value) -> bool {
        match self {
            Handler::Sync(handler) => handler.is_pertinent_to(value),
            Handler::Async(handler) => handler.is_pertinent_to(value),
        }
    }
    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `dialect` for a given `Dialect`. It
    /// **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaHandler.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    pub fn dialect(&self, value: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        match self {
            Handler::Sync(handler) => handler.dialect(value),
            Handler::Async(handler) => handler.dialect(value),
        }
    }

    /// Returns a list of [`LocatedSchema`] for each embedded schema within
    /// `value`.
    pub fn schemas<'v>(
        &self,
        path: &Pointer,
        base_uri: &AbsoluteUri,
        value: &'v Value,
        dialects: &Dialects,
    ) -> Result<Vec<LocatedSchema<'v>>, IdentifyError> {
        match self {
            Handler::Sync(h) => h.schemas(path, base_uri, value, dialects),
            Handler::Async(h) => h.schemas(path, base_uri, value, dialects),
        }
    }

    /// Returns a list of [`Reference`](`crate::schema::Reference`)s to other
    /// schemas that `schema` depends on.
    pub fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        match self {
            Handler::Sync(h) => h.references(schema),
            Handler::Async(h) => h.references(schema),
        }
    }
}

#[async_trait]
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.
pub trait AsyncHandler: IntoHandler + Send + Sync + DynClone + fmt::Debug {
    /// For each `Schema` compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    async fn compile<'h, 'c, 's, 'p>(
        &mut self,
        compile: &'c mut Compile<'s>,
        schema: &'s Value,
    ) -> Result<bool, CompileError>;

    /// Executes the handler logic for the given [`Schema`] and [`Value`].
    async fn evaluate<'h, 's, 'v>(
        &'h self,
        scope: &'s mut Scope,
        value: &'v Value,
        structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    fn schemas<'v>(
        &self,
        path: &Pointer,
        base_uri: &AbsoluteUri,
        value: &'v Value,
        dialects: &Dialects,
    ) -> Result<Vec<LocatedSchema<'v>>, IdentifyError> {
        Ok(Vec::new())
    }
    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `identify` for a given `Dialect`.
    /// It **must** be the **second** (index: `1`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    #[allow(unused_variables)]
    fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        unimplemented!("identify must be implemented by the second Handler in a Dialect")
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the `dialect` method for a given
    /// `Dialect`. It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaHandler.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    #[allow(unused_variables)]
    fn dialect(&self, schema: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        unimplemented!("dialect must be implemented by the first Handler in a Dialect")
    }

    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `is_pertinent_to` for a given `Dialect`.
    /// It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(is_pertinent_to);
    ///
    /// let draft = "https://json-schema.org/draft/2019-09/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(!is_pertinent_to);
    /// ```
    #[allow(unused_variables)]
    fn is_pertinent_to(&self, schema: &Value) -> bool {
        unimplemented!("is_pertinent_to must be implemented by the first Handler in a Dialect")
    }

    /// Returns a list of [`Reference`](`crate::schema::Reference`)s to other
    /// schemas that `schema` depends on.
    #[allow(unused_variables)]
    fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        Ok(Vec::new())
    }
}

clone_trait_object!(AsyncHandler);
/// Handles the setup and execution of logic for a given keyword in a JSON Schema.

pub trait SyncHandler: IntoHandler + Send + Sync + DynClone + fmt::Debug {
    /// For each [`Schema`] compiled by the [`Interrogator`], this `Handler` is
    /// cloned and [`setup`] is called.
    ///
    /// If the handler is applicable to the given [`Schema`], it must return
    /// `true`. A return value of `false` indicates that [`execute`] should not
    /// be called for the given [`Schema`].
    fn compile<'s>(
        &mut self,
        compile: &mut Compile<'s>,
        schema: &'s Value,
    ) -> Result<bool, CompileError>;

    /// Evaluates the [`Value`] `value` and optionally returns an `Annotation`.
    ///
    /// Handlers should fail fast if the `structure` is
    /// [`Structure::Flag`](`crate::output::Structure::Flag`)
    fn evaluate<'v>(
        &self,
        scope: &mut Scope,
        value: &'v Value,
        _structure: Structure,
    ) -> Result<Option<output::Node<'v>>, EvaluateError>;

    /// Attempts to identify the schema based on the
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `identify` for a given
    /// `Dialect`. It **must** be the **second** (index: `1`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::handlers::Id;
    ///
    /// let id = Id.identify(&json!({"$id": "https://example.com/schema.json"}));
    /// assert_eq!(id, Ok(Some("https://example.com/schema.json".parse().unwrap())));
    /// ```
    #[allow(unused_variables)]
    fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        unimplemented!("identify must be implemented by the second Handler in a Dialect")
    }

    /// Attempts to retrieve the [`AbsoluteUri`](`crate::uri::AbsoluteUri`) of
    /// the schema.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `dialect` for a given `Dialect`. It
    /// **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let dialect = SchemaHandler.dialect(&json!({ "$schema": draft }));
    /// assert_eq!(dialect.as_str(), draft);
    /// ```
    #[allow(unused_variables)]
    fn dialect(&self, value: &Value) -> Result<Option<AbsoluteUri>, UriError> {
        unimplemented!("dialect must be implemented by the first Handler in a Dialect")
    }

    /// Determines if the schema is of a specific
    /// [`Dialect`](`crate::dialect::Dialect`).
    ///
    /// # Convention
    /// Exactly one `Handler` must implement the method `is_pertinent_to` for a
    /// given `Dialect`. It **must** be the **first** (index: `0`) `Handler` in
    /// the [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`)
    ///
    /// # Example
    /// ```rust
    /// use grill::json_schema::draft_2020_12::SchemaHandler;
    ///
    /// let draft = "https://json-schema.org/draft/2020-12/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(is_pertinent_to);
    ///
    /// let draft = "https://json-schema.org/draft/2019-09/schema";
    /// let is_pertinent_to = SchemaHandler.is_pertinent_to(&json!({ "$schema": draft }));
    /// assert!(!is_pertinent_to);
    /// ```
    #[allow(unused_variables)]
    fn is_pertinent_to(&self, schema: &Value) -> bool {
        unimplemented!("is_pertinent_to must be implemented by the first Handler in a Dialect")
    }

    /// Returns a list of [`LocatedSchema`] for each subschema in `value`.
    #[allow(unused_variables)]
    fn schemas<'v>(
        &self,
        path: &Pointer,
        base_uri: &AbsoluteUri,
        value: &'v Value,
        dialects: &Dialects,
    ) -> Result<Vec<LocatedSchema<'v>>, IdentifyError> {
        Ok(Vec::new())
    }

    /// Returns a list of [`Reference`](`crate::schema::Reference`)s to other
    /// schemas that `schema` depends on.
    #[allow(unused_variables)]
    fn references(&self, schema: &Value) -> Result<Vec<Reference>, UriError> {
        Ok(Vec::new())
    }
}
clone_trait_object!(SyncHandler);

pub trait IntoHandler {
    fn into_handler(self) -> Handler;
}

impl<T> IntoHandler for T
where
    T: Into<Handler>,
{
    fn into_handler(self) -> Handler {
        self.into()
    }
}

// AnyMap, TypIdHasher, and Downcast was sourced (and modified) from the anymap
// crate:
// https://github.com/chris-morgan/anymap/blob/2e9a570491664eea18ad61d98aa1c557d5e23e67/src/any.rs
// The anymap crate is licensed under BlueOak-1.0.0 OR MIT OR Apache-2.0
// The reason this was lifted rather than using anymap directly is due to `Downcast` not being exposed.
// unsafe code can be removed once dyn upcasting is stable: https://github.com/rust-lang/rust/issues/65991

trait Item: Any + std::fmt::Debug + DynClone + Send + Sync {}
clone_trait_object!(Item);

impl<T> Item for T where T: 'static + Any + std::fmt::Debug + Send + Sync + DynClone {}
trait Downcast {
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T;

    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T;

    unsafe fn downcast_unchecked<T: 'static>(self: Box<Self>) -> Box<T>;
}

#[allow(clippy::ptr_as_ptr)]
impl Downcast for dyn Item {
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
        &*(self as *const Self as *const T)
    }
    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T {
        &mut *(self as *mut Self as *mut T)
    }
    #[inline]
    unsafe fn downcast_unchecked<T: 'static>(self: Box<Self>) -> Box<T> {
        Box::from_raw(Box::into_raw(self) as *mut T)
    }
}

type AnyMap = HashMap<TypeId, Box<dyn Item>, BuildHasherDefault<TypeIdHasher>>;

#[derive(Default)]
struct TypeIdHasher(u64);
impl Hasher for TypeIdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct State {
    map: AnyMap,
}
impl State {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entry<T>(&mut self) -> Entry<'_, T>
    where
        T: 'static + Clone + Send + Sync,
    {
        self.map.entry(TypeId::of::<T>()).into()
    }

    #[must_use]
    pub fn contains<T>(&self) -> bool
    where
        T: 'static + Clone + Send + Sync,
    {
        self.map.contains_key(&TypeId::of::<T>())
    }

    #[must_use]
    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any + std::fmt::Debug + Clone + Send + Sync,
    {
        let v = self.map.get(&TypeId::of::<T>());
        v.map(|v| unsafe { v.downcast_ref_unchecked::<T>() })
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static + Clone + Send + Sync,
    {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|v| unsafe { v.downcast_mut_unchecked() })
    }

    pub fn insert<T>(&mut self, value: T) -> Option<T>
    where
        T: 'static + Clone + std::fmt::Debug + Send + Sync,
    {
        self.map
            .insert(TypeId::of::<T>(), Box::new(value))
            .map(|v| *unsafe { v.downcast_unchecked() })
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

pub struct OccupiedEntry<'a, T> {
    inner: hash_map::OccupiedEntry<'a, TypeId, Box<dyn Item>>,
    _marker: std::marker::PhantomData<T>,
}
impl<'a, T: 'static> OccupiedEntry<'a, T> {
    /// Gets a reference to the value in the entry.
    #[inline]
    #[must_use]
    pub fn get(&self) -> &T {
        unsafe { self.inner.get().downcast_ref_unchecked() }
    }

    /// Gets a mutable reference to the value in the entry
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { self.inner.get_mut().downcast_mut_unchecked() }
    }

    /// Converts the `OccupiedEntry` into a mutable reference to the value in the entry
    /// with a lifetime bound to the collection itself
    #[inline]
    #[must_use]
    pub fn into_mut(self) -> &'a mut T {
        unsafe { self.inner.into_mut().downcast_mut_unchecked() }
    }

    /// Sets the value of the entry, and returns the entry's old value
    #[inline]
    pub fn insert(&mut self, value: T) -> T
    where
        T: 'static + Clone + std::fmt::Debug + Send + Sync,
    {
        unsafe { *self.inner.insert(Box::new(value)).downcast_unchecked() }
    }

    /// Takes the value out of the entry, and returns it
    #[inline]
    #[must_use]
    pub fn remove(self) -> T {
        unsafe { *self.inner.remove().downcast_unchecked() }
    }
}

pub struct VacantEntry<'a, T> {
    inner: hash_map::VacantEntry<'a, TypeId, Box<dyn Item>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> VacantEntry<'a, T> {
    pub fn insert(self, value: T) -> &'a mut T
    where
        T: 'static + Clone + std::fmt::Debug + Send + Sync,
    {
        unsafe {
            self.inner
                .insert(Box::new(value))
                .downcast_mut_unchecked::<T>()
        }
    }
}
pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}

impl<'a, T> Entry<'a, T>
where
    T: 'static + Any + std::fmt::Debug + Send + Sync + Clone,
{
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    #[inline]
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if
    /// empty, and returns a mutable reference to the value in the entry.
    #[inline]
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting the default value if empty,
    /// and returns a mutable reference to the value in the entry.
    #[must_use]
    #[inline]
    pub fn or_default(self) -> &'a mut T
    where
        T: Default,
    {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(Default::default()),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any potential inserts
    /// into the map.
    #[must_use]
    #[inline]
    pub fn and_modify<F: FnOnce(&mut T)>(self, f: F) -> Self {
        match self {
            Entry::Occupied(mut inner) => {
                f(inner.get_mut());
                Entry::Occupied(inner)
            }
            Entry::Vacant(inner) => Entry::Vacant(inner),
        }
    }
}

impl<'a, T> From<hash_map::Entry<'a, TypeId, Box<dyn Item>>> for Entry<'a, T> {
    fn from(value: hash_map::Entry<'a, TypeId, Box<dyn Item>>) -> Self {
        match value {
            hash_map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry {
                inner: entry,
                _marker: std::marker::PhantomData,
            }),
            hash_map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                inner: entry,
                _marker: std::marker::PhantomData,
            }),
        }
    }
}

use crate::{location::Locate, output::Node, Location, SchemaKey};
/// Contains state and location information needed to perform an
/// [`evaluation`](`crate::Interrogator::evaluate`).
pub struct Scope<'s> {
    pub state: &'s mut State,
    location: Location,
    number: Option<BigRational>,
}

#[inherent]
impl Locate for Scope<'_> {
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }
}

impl<'s> Scope<'s> {
    pub fn new(
        location: Location,
        state: &'s mut State,
        _schemas: SlotMap<SchemaKey, Value>,
    ) -> Self {
        Self {
            state,
            location,
            number: None,
        }
    }
    #[must_use]
    pub fn annotate<'v>(&self, keyword: &'static str, value: &'v Value) -> Node<'v> {
        let mut location = self.location.clone();
        location.push_keyword_location(keyword);
        Node::new(location, value)
    }

    /// # Errors
    /// Returns a [`ParseError`](`big_rational_str::ParseError`) if `number` cannot be parsed as a [`BigRational`].
    #[allow(clippy::missing_panics_doc)]
    pub fn number(&mut self, number: &Number) -> Result<&BigRational, BigRatParseError> {
        let n = &mut self.number;
        if let Some(number) = n {
            Ok(number)
        } else {
            let number = big_rational_str::str_to_big_rational(&number.to_string())?;
            n.replace(number);
            Ok(n.as_ref().unwrap())
        }
    }

    /// Returns a new, nested [`Scope`], where `instance` should be the name of
    /// field or index within the value being evaluated and `keyword` is the
    /// keyword being executed.
    ///
    /// # Errors
    /// Returns a [`jsonptr::Error`](`jsonptr::Error`) if the
    /// `absolute_keyword_location`'s pointer is malformed.
    pub fn nested(
        &mut self,
        _instance: &str,
        _keyword: &str,
        _absolute_keyword_location: Option<String>,
    ) -> Result<Scope, jsonptr::MalformedPointerError> {
        // let mut keyword_location = self.keyword_location().clone();
        // keyword_location.push_back(keyword.into());
        // let absolute_keyword_location =
        //     if let Some(absolute_keyword_location) = absolute_keyword_location {
        //         absolute_keyword_location
        //     } else {
        //         let v = self.location.absolute_keyword_location.clone();
        //         let (uri, ptr) = v.split_once('#').unwrap_or((&v, ""));
        //         let mut ptr: Pointer = Pointer::try_from(ptr)?;
        //         ptr.push_back(keyword.into());
        //         format!("{uri}#{ptr}")
        //     };
        // let mut instance_location = self.instance_location().clone();
        // instance_location.push_back(instance.into());
        // Ok(Scope {
        //     location: Location {
        //         keyword_location,
        //         absolute_keyword_location,
        //         instance_location,
        //     },
        //     state: self.state,
        //     number: None,
        // })
        todo!()
    }
}

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Compile<'s> {
    marker: PhantomData<&'s String>, // location: Location,
                                     // anchors: Vec<(String, Anchor<'s>)>,
                                     // schemas: HashMap<Keyword<'s>, Subschema<'s>>,
                                     // numbers: HashMap<Keyword<'s>, &'s Number>,
                                     // references: HashMap<Keyword<'s>, &'s str>,
}

// impl<'s> Compile<'s> {
//     #[must_use]
//     pub fn new(location: Location) -> Self {
//         Self {
//             location,
//             anchors: Vec::new(),
//             schemas: HashMap::default(),
//             numbers: HashMap::default(),
//             references: HashMap::default(),
//         }
//     }

//     pub fn anchor(&mut self, anchor: Anchor<'s>) {
//         self.anchors
//             .push((self.location.absolute_keyword_location.clone(), anchor));
//     }
//     pub fn schema(&mut self, keyword: Keyword<'s>, schema: Subschema<'s>) {
//         self.schemas.insert(keyword, schema);
//     }
//     pub fn reference(&mut self, keyword: Keyword<'s>, reference: &'s str) {
//         self.references.insert(keyword, reference);
//     }

//     /// # Errors
//     pub fn number<'x>(&'x mut self, keyword: Keyword<'s>, number: &'s Number) {
//         self.numbers.entry(keyword).or_insert_with(|| number);
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut state = State::new();
        let i: i32 = 1;
        state.insert(i);
        let x = state.get_mut::<i32>().unwrap();
        *x += 1;

        assert_eq!(state.get::<i32>(), Some(&2));
    }
}
