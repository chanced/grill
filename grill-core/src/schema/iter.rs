//! Linear [`Iterator`]s of [`Schema`]s from [`Key`]s.
//!
use either::Either;

use super::Schemas;
use crate::{error::UnknownKeyError, source::Sources, Key, Schema};

/// An [`Iterator`] over [`Schema`]s from an `Iterator` of [`Key`]s.
///
/// Each [`Item`](Iterator::Item) is a `Result<Schema, UnknownKeyError>`, to
/// safeguard against the circumstance of a [`Key`] belonging to a different
/// [`Interrogator`](`crate::Interrogator`). If this is not a concern, use
/// [`unchecked`](`Iter::unchecked`) which unwraps all `Result`s.
///
pub struct Iter<'i, Keyword> {
    sources: &'i Sources,
    schemas: &'i Schemas<Keyword>,
    inner: Either<std::slice::Iter<'i, Key>, std::vec::IntoIter<Key>>,
}

impl<'i, Keyword> Iter<'i, Keyword> {
    pub(crate) fn new(
        keys: &'i [Key],
        schemas: &'i Schemas<Keyword>,
        sources: &'i Sources,
    ) -> Self {
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
    pub fn unchecked(self) -> IterUnchecked<'i, Keyword> {
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
impl<'i, K> Iterator for Iter<'i, K> {
    type Item = Result<Schema<'i, K>, UnknownKeyError>;

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
pub struct IterUnchecked<'i, Keyword> {
    inner: Iter<'i, Keyword>,
}

impl<'i, K> Iterator for IterUnchecked<'i, K> {
    type Item = Schema<'i, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(std::result::Result::unwrap)
    }
}
