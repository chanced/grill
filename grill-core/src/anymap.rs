//! [`AnyMap`] was sourced from the [anymap
//! crate](https://crates.io/crates/anymap).
//!
//! The anymap crate is licensed under BlueOak-1.0.0 OR MIT OR Apache-2.0 The
//! reason this was lifted rather than using anymap directly is due to
//! `Downcast` not being exposed. unsafe code can be removed once dyn upcasting
//! is stable: <https://github.com/rust-lang/rust/issues/65991>

use std::{
    any::{Any, TypeId},
    collections::{hash_map, HashMap},
    hash::{BuildHasherDefault, Hasher},
};

use dyn_clone::{clone_trait_object, DynClone};

trait Item: Any + std::fmt::Debug + DynClone + Send + Sync {}
clone_trait_object!(Item);

impl<T> Item for T where T: 'static + Any + std::fmt::Debug + Send + Sync + DynClone {}
trait Downcast {
    unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T;

    unsafe fn downcast_mut_unchecked<T: 'static>(&mut self) -> &mut T;

    #[allow(clippy::unnecessary_box_returns)]
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

type Map = HashMap<TypeId, Box<dyn Item>, BuildHasherDefault<TypeIdHasher>>;

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
/// A collection containing zero or one values for any given type and
/// allowing convenient, type-safe access to those values.
///
/// sourced from [`anymap`](https://docs.rs/anymap/1.0.0-beta.2/anymap/index.html)
#[derive(Clone, Debug, Default)]
pub struct AnyMap {
    map: Map,
}
impl AnyMap {
    /// Creates a new empty `AnyMap` collection
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Gets the entry for the given type in the collection for in-place manipulation
    pub fn entry<T>(&mut self) -> Entry<'_, T>
    where
        T: 'static + Clone + Send + Sync,
    {
        self.map.entry(TypeId::of::<T>()).into()
    }

    /// Returns true if the collection contains a value of type `T`.
    #[must_use]
    pub fn contains<T>(&self) -> bool
    where
        T: 'static + Clone + Send + Sync,
    {
        self.map.contains_key(&TypeId::of::<T>())
    }

    /// Returns a reference to the value stored in the collection for the type
    /// `T`, if it exists.
    #[must_use]
    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any + std::fmt::Debug + Clone + Send + Sync,
    {
        let v = self.map.get(&TypeId::of::<T>());
        v.map(|v| unsafe { v.downcast_ref_unchecked::<T>() })
    }
    /// Returns a mutable reference to the value stored in the collection for
    /// the type `T`, if it exists.
    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static + Clone + Send + Sync,
    {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|v| unsafe { v.downcast_mut_unchecked() })
    }
    /// Sets the value stored in the collection for the type `T`. If the
    /// collection already had a value of type `T`, that value is returned.
    /// Otherwise, `None` is returned.
    pub fn insert<T>(&mut self, value: T) -> Option<T>
    where
        T: 'static + Clone + std::fmt::Debug + Send + Sync,
    {
        self.map
            .insert(TypeId::of::<T>(), Box::new(value))
            .map(|v| *unsafe { v.downcast_unchecked() })
    }
    /// Returns the number of items in the collection.
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if there are no items in the collection.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// A view into a single occupied location in an `AnyMap`.
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

/// A view into a single empty location in an `AnyMap`.
pub struct VacantEntry<'a, T> {
    inner: hash_map::VacantEntry<'a, TypeId, Box<dyn Item>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> VacantEntry<'a, T> {
    /// Sets the value of the entry with the VacantEntry’s key, and returns a
    /// mutable reference to it
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
/// A view into a single location in an `AnyMap`, which may be vacant or
/// occupied.
pub enum Entry<'a, T> {
    /// A view into a single occupied location in an `AnyMap`.
    Occupied(OccupiedEntry<'a, T>),
    /// A view into a single empty location in an `AnyMap`.
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

#[test]
fn test_anymap_get() {
    let mut state = AnyMap::new();
    let i: i32 = 1;
    state.insert(i);
    let x = state.get_mut::<i32>().unwrap();
    *x += 1;

    assert_eq!(state.get::<i32>(), Some(&2));
}
