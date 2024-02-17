//! Linear [`Iterator`]s of [`Schema`]s from [`Key`]s.
//!
use either::Either;
use slotmap::Key;

use super::Schemas;
use crate::{error::UnknownKeyError, source::Sources, Language, Schema};

/// An [`Iterator`] over [`Schema`]s from an `Iterator` of [`Key`]s.
///
/// Each [`Item`](Iterator::Item) is a `Result<Schema, UnknownKeyError>`, to
/// safeguard against the circumstance of a [`Key`] belonging to a different
/// [`Interrogator`](`crate::Interrogator`). If this is not a concern, use
/// [`unchecked`](`Iter::unchecked`) which unwraps all `Result`s.
///
pub struct Iter<'i, L: Language<K>, K: Key> {
    sources: &'i Sources,
    schemas: &'i Schemas<L, K>,
    inner: Either<std::slice::Iter<'i, K>, std::vec::IntoIter<K>>,
}

impl<'i, L: Language<K>, K: Key> Iter<'i, L, K> {
    pub(crate) fn new(keys: &'i [K], schemas: &'i Schemas<L, K>, sources: &'i Sources) -> Self {
        Self {
            sources,
            schemas,
            inner: Either::Left(keys.iter()),
        }
    }
    /// Converts this `Iter` into an `IterUnchecked`, thus unwrapping all
    /// `Result`s.
    ///
    /// # Safety
    /// Do not use this unless you are certain all `Key`s are associated with
    /// the [`Interrogator`] from which this is originated.
    #[must_use]
    pub fn unchecked(self) -> IterUnchecked<'i, L, K> {
        IterUnchecked { inner: self }
    }

    // pub(crate) fn from_vec(keys: Vec<Key>, schemas: &'i Schemas, sources: &'i Sources) -> Self {
    //     Self {
    //         sources,
    //         schemas,
    //         inner: Either::Right(keys.into_iter()),
    //     }
    // }
}
impl<'i, L: Language<K>, K: Key> Iterator for Iter<'i, L, K> {
    type Item = Result<Schema<'i, L, K>, UnknownKeyError<K>>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = match self.inner.as_mut() {
            Either::Left(iter) => *iter.next()?,
            Either::Right(iter) => iter.next()?,
        };
        Some(self.schemas.get(key, self.sources))
    }
}
/// An unchecked [`Iterator`] over [`Schema`]s from an `Iterator` of [`Key`]s.
///
/// # Panics
/// This will panic if any of the [`Key`]s are not associated with the same
/// [`Interrogator`](`crate::Interrogator`).
pub struct IterUnchecked<'i, L: Language<K>, K: Key> {
    inner: Iter<'i, L, K>,
}

impl<'i, L: Language<K>, K: Key> Iterator for IterUnchecked<'i, L, K> {
    type Item = Schema<'i, L, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(std::result::Result::unwrap)
    }
}
