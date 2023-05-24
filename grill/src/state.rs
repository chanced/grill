use dyn_clone::{clone_trait_object, DynClone};
use std::{
    any::{Any, TypeId},
    collections::{hash_map, HashMap},
    hash::{BuildHasherDefault, Hasher},
};

// AnyMap and TypeIdHasher were sourced (and modified) from hyperium's http crate:
// https://github.com/hyperium/http/blob/20633e59339e753990bb734d7c73adba6ccff4ed/src/extensions.rs

type AnyMap = HashMap<TypeId, Box<dyn Item>, BuildHasherDefault<TypeIdHasher>>;

trait Item: Any + std::fmt::Debug + DynClone + Send + Sync {}
clone_trait_object!(Item);

impl<T> Item for T where T: std::fmt::Debug + Any + Send + Sync + Clone {}

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

/// Downcasts an opaque value to a value of a concrete type.
/// # Safety
/// It is imperative that the caller ensures that the value is of type `T`
fn downcast<T>(v: Box<dyn Any>) -> T {
    // SAFETY: v has been indexed by it's TypeId in a HashMap, reaching this point requires that the value is of type T
    unsafe { *v.downcast().unwrap_unchecked() }
}

/// Downcasts an opaque reference to a reference of a concrete type.
/// # Safety
/// It is imperative that the caller ensures that the value is of type `T`

/// Downcasts an opaque reference to a reference of a concrete type.
/// # Safety
/// It is imperative that the caller ensures that the value is of type `T`
fn downcast_ref<T>(v: &(dyn Any)) -> &T {
    // SAFETY: v has been indexed by it's TypeId in a HashMap, reaching this point requires that the value is of type T
    unsafe { v.downcast_ref().unwrap_unchecked() }
}
/// Downcasts a mutable opaque reference to a mutable reference of a concrete type.
///
/// # Safety
/// It is imperative that the caller ensures that the value is of type `T`
fn downcast_mut<T>(v: &mut (dyn Any)) -> &mut T {
    // SAFETY: v has been indexed by it's TypeId in a HashMap, reaching this point requires that the value is of type T
    unsafe { v.downcast_mut().unwrap_unchecked() }
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
        T: Clone + Send + Sync,
    {
        self.map.entry(TypeId::of::<T>()).into()
    }

    #[must_use]
    pub fn contains<T>(&self) -> bool
    where
        T: Clone + Send + Sync,
    {
        self.map.contains_key(&TypeId::of::<T>())
    }
    fn x(x: &Box<dyn Any>) {}

    #[must_use]
    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any + Clone + Send + Sync,
    {
        self.map
            .get(&TypeId::of::<T>())
            .map(|v| downcast_ref::<T>(v))
    }

    pub fn get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Clone + Send + Sync,
    {
        self.map
            .get_mut(&TypeId::of::<T>())
            .map(|v| downcast_mut::<T>(v))
    }

    pub fn insert<T>(&mut self, value: T) -> Option<T>
    where
        T: Clone + std::fmt::Debug + Send + Sync,
    {
        self.map
            .insert(TypeId::of::<T>(), Box::new(value))
            .map(|v| downcast(v))
    }

    // pub fn remove<T>(&mut self) -> Option<T>
    // where
    //     T: 'static + Clone + Send + Sync,
    // {
    //     self.map.remove::<T>()
    // }
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

pub enum Entry<'a, T> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}
// impl<'a> From<hash_map::Entry<'a, TypeId, Box<dyn Item>> for Entry<'a, T> {}
impl<'a, T> From<hash_map::Entry<'a, TypeId, Box<dyn Item>>> for Entry<'a, T> {
    fn from(value: hash_map::Entry<'a, TypeId, Box<dyn Item>>) -> Self {
        match value {
            hash_map::Entry::Occupied(entry) => Entry::Occupied(OccupiedEntry {
                entry,
                _marker: std::marker::PhantomData,
            }),
            hash_map::Entry::Vacant(entry) => Entry::Vacant(VacantEntry {
                entry,
                _marker: std::marker::PhantomData,
            }),
        }
    }
}

impl<'a, T> Entry<'a, T> {
    #[inline]
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(inner) => inner.or_insert(default),
            Entry::Vacant(inner) => inner.or_insert(default),
        }
    }

    #[inline]
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            Entry::Occupied(inner) => inner.into_mut(),
            Entry::Vacant(inner) => inner.insert(default()),
        }
    }

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

pub struct OccupiedEntry<'a, T> {
    entry: hash_map::OccupiedEntry<'a, TypeId, Box<dyn Item>>,
    _marker: std::marker::PhantomData<T>,
}
impl<'a, T> OccupiedEntry<'a, T> {
    fn or_insert(&self, value: T) -> &mut T {
        todo!()
    }

    fn into_mut(&self) -> &mut T {
        todo!()
    }

    fn get_mut(&self) -> &mut T {
        todo!()
    }
}

pub struct VacantEntry<'a, T> {
    entry: hash_map::VacantEntry<'a, TypeId, Box<dyn Item>>,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T> VacantEntry<'a, T> {
    fn or_insert(&self, default: T) -> &mut T {
        todo!()
    }

    fn insert(&self, default: T) -> &mut T {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let mut state = State::new();
        state.insert(1);
        assert_eq!(state.get::<i32>(), Some(&1));
    }
}
