//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::error::{DialectExistsError, UriError};
use crate::handler::Handler;
use crate::schema::Metaschema;
use crate::SchemaKey;

use crate::{
    error::{DialectError, IdentifyError},
    uri::AbsoluteUri,
    Object, SrcValue,
};
use jsonptr::Pointer;
use serde_json::Value;
use std::{
    borrow::{Borrow, Cow},
    collections::{HashMap, HashSet},
    convert::Into,
    fmt::Debug,
    hash::Hash,
    iter::IntoIterator,
    ops::Deref,
};
use tap::TapOptional;

use super::Reference;

/// A set of keywords and semantics which are used to evaluate a [`Value`](serde_json::Value) against a
/// schema.
#[derive(Clone)]
pub struct Dialect<Key = SchemaKey>
where
    Key: 'static + slotmap::Key,
{
    /// Identifier of the `Dialect`. A meta schema must be defined in
    /// `metaschemas` with this `id`.
    pub id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    pub metaschemas: HashMap<AbsoluteUri, Object>,
    /// Set of [`Handler`]s defined by the dialect.
    pub handlers: Vec<Handler<Key>>,
}
impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl<Key> Dialect<Key>
where
    Key: 'static + slotmap::Key,
{
    /// Creates a new [`Dialect`].
    pub fn new<S, M, I, H>(
        id: AbsoluteUri,
        metaschemas: M,
        handlers: H,
    ) -> Result<Self, DialectError>
    where
        S: Borrow<Metaschema>,
        M: IntoIterator<Item = S>,
        I: Into<Handler<Key>>,
        H: IntoIterator<Item = I>,
    {
        let metaschemas: HashMap<AbsoluteUri, Object> = metaschemas
            .into_iter()
            .map(|m| {
                let m = m.borrow();
                (m.id.clone(), m.schema.clone())
            })
            .collect();
        let handlers = handlers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Handler<Key>>>();
        if !metaschemas.contains_key(&id) {
            return Err(DialectError::DefaultNotFound(id.clone()));
        }
        Ok(Self {
            id,
            metaschemas,
            handlers,
        })
    }

    /// Attempts to identify a `schema` based on the [`Handler`]s associated with
    /// this `Dialect`, returning the primary (if any) and all [`AbsoluteUri`]s
    /// the [`Schema`](`crate::schema::Schema`) can be referenced by.
    ///
    /// # Convention
    /// The second (index: `1`) `Handler` must implement `identify` and, if
    /// able, return the primary identifier.
    ///
    /// Secondary identifiers are determined by
    /// [`Handler`](crate::handler::Handler)s index `2` and greater.
    pub fn identify(
        &self,
        mut base_uri: AbsoluteUri,
        path: &Pointer,
        schema: &Value,
    ) -> Result<(Option<AbsoluteUri>, Vec<AbsoluteUri>), IdentifyError> {
        let mut uris = Vec::new();

        // use the second handler to identify the primary identifier, if any
        let primary = self.handlers[1]
            .identify(schema)?
            .map(|uri| base_uri.resolve(&uri))
            .transpose()?
            .tap_some(|id| uris.push(id.clone()));

        base_uri.set_fragment(Some(path))?;
        uris.insert(0, base_uri.clone());

        // if a primary identifier was found, use it as the base_uri for
        // subschemas. Otherwise, use the base_uri provided by the caller
        // with the fragment set to the json pointer `path`.
        let base_uri = primary.as_ref().unwrap_or(&base_uri);

        for handler in &self.handlers[2..] {
            if let Some(id) = handler.identify(schema)? {
                uris.push(base_uri.resolve(&id)?);
            }
        }
        Ok((primary, uris))
    }
    #[must_use]
    pub fn primary_metaschema_id(&self) -> &AbsoluteUri {
        &self.id
    }
    /// Attempts to locate nested schemas within `source` by calling
    /// [`Handler::subschemas`](`crate::Handler::subschemas`) for each attached
    /// `Handler` of this `Dialect`.
    ///
    #[must_use]
    pub fn subschemas(&self, path: &Pointer, source: &Value) -> HashSet<Pointer> {
        let mut locations = HashSet::new();
        for handler in &self.handlers {
            locations.extend(handler.subschemas(path, source));
        }
        locations
    }

    pub fn references(&self, source: &Value) -> Result<Vec<Reference<Key>>, UriError> {
        let mut refs = Vec::new();
        for handler in &self.handlers {
            refs.append(&mut handler.references(source)?);
        }
        Ok(refs)
    }

    /// Determines if the schema is pertinent to this `Dialect`.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement `is_pertinent_to` for a given
    ///
    /// `Dialect`. It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`).
    #[must_use]
    pub fn is_pertinent_to(&self, schema: &Value) -> bool {
        self.handlers[0].is_pertinent_to(schema)
    }
}

impl<Key> PartialEq for Dialect<Key>
where
    Key: 'static + slotmap::Key,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<Key> Hash for Dialect<Key>
where
    Key: 'static + slotmap::Key,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<Key> Debug for Dialect<Key>
where
    Key: 'static + slotmap::Key,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dialect")
            .field("id", &self.id)
            .field("meta_schemas", &self.metaschemas)
            .field("handlers", &self.handlers)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Dialects<'d, Key = SchemaKey>
where
    Key: 'static + slotmap::Key,
{
    dialects: Cow<'d, [Dialect<Key>]>,
    lookup: Cow<'d, HashMap<AbsoluteUri, usize>>,
    primary: usize,
}

impl<'d, Key> Deref for Dialects<'d, Key>
where
    Key: 'static + slotmap::Key,
{
    type Target = [Dialect<Key>];

    fn deref(&self) -> &Self::Target {
        self.dialects.as_ref()
    }
}

impl<'d, Key> Dialects<'d, Key>
where
    Key: 'static + slotmap::Key,
{
    pub fn new(
        dialects: Vec<Dialect<Key>>,
        default: Option<&AbsoluteUri>,
    ) -> Result<Self, DialectError> {
        if dialects.is_empty() {
            return Err(DialectError::Empty);
        }
        let mut collected: Vec<Dialect<Key>> = Vec::with_capacity(dialects.len());
        let mut lookup: HashMap<AbsoluteUri, usize> = HashMap::with_capacity(dialects.len());
        for (i, dialect) in dialects.into_iter().enumerate() {
            if dialect.id.fragment().is_some() && dialect.id.fragment() != Some("") {
                return Err(DialectError::FragmentedId(dialect.id.clone()));
            }
            if lookup.contains_key(&dialect.id) {
                return Err(DialectError::Duplicate(DialectExistsError {
                    id: dialect.id,
                }));
            }
            let id = dialect.id.clone();
            collected.push(dialect);
            lookup.insert(id, i);
        }
        let default = Self::find_primary(&collected, &lookup, default)?;
        Ok(Self {
            dialects: Cow::Owned(collected),
            lookup: Cow::Owned(lookup),
            primary: default,
        })
    }

    /// Returns the [`Dialect`](crate::dialect::Dialect).
    #[must_use]
    pub fn get(&self, id: &AbsoluteUri) -> Option<&Dialect<Key>> {
        self.lookup.get(id).map(|&index| &self.dialects[index])
    }
    /// Returns the [`Dialect`] that is determined pertinent to the schema based
    /// upon the first [`Handler`] in each
    /// [`Dialect`](`crate::dialect::Dialect`) or `None` if a [`Dialect`] cannot
    /// be confidently determined.
    #[must_use]
    pub fn pertinent_to(&self, schema: &Value) -> Option<&Dialect<Key>> {
        self.dialects
            .iter()
            .find(|&dialect| dialect.is_pertinent_to(schema))
    }

    /// Appends a [`Dialect`].
    ///
    /// # Errors
    /// Returns the [`DialectExists`] if a `Dialect` already exists with the same `id`.
    pub(crate) fn push(&mut self, dialect: Dialect<Key>) -> Result<(), DialectExistsError> {
        if self.lookup.contains_key(&dialect.id) {
            return Err(DialectExistsError { id: dialect.id });
        }
        self.lookup
            .to_mut()
            .insert(dialect.id.clone(), self.dialects.len());
        self.dialects.to_mut().push(dialect);
        Ok(())
    }

    /// Returns a new `Dialects` with the default `Dialect` set to the provided
    ///
    /// # Errors
    /// Returns the `&Dialect` if the `Dialect` is not currently in this `Dialects`.
    pub fn with_default<'o>(
        &'d self,
        default: &'o Dialect,
    ) -> Result<Dialects<'d, Key>, &'o Dialect> {
        let default = self.lookup.get(&default.id).copied().ok_or(default)?;
        Ok(Dialects {
            dialects: Cow::Borrowed(self.dialects.as_ref()),
            primary: default,
            lookup: Cow::Borrowed(self.lookup.as_ref()),
        })
    }

    #[must_use]
    pub fn contains(&self, id: &AbsoluteUri) -> bool {
        self.lookup.contains_key(id)
    }

    /// Returns the [`Dialect`] that is pertinent to the schema or the default
    /// [`Dialect`] if the [`Dialect`] can not be determined from schema.
    #[must_use]
    pub fn pertinent_to_or_default(&self, schema: &Value) -> &Dialect<Key> {
        self.pertinent_to(schema).unwrap_or(self.primary_dialect())
    }
    /// Returns an [`Iterator`] of [`&AbsoluteUri`](`crate::uri::AbsoluteUri`) for each metaschema in each [`Dialect`](`crate::dialect::Dialect`).
    pub fn source_ids(&self) -> impl Iterator<Item = &AbsoluteUri> {
        self.dialects.iter().map(|d| &d.id)
    }

    #[must_use]
    pub fn sources(&self) -> Vec<SrcValue> {
        let mut result = Vec::with_capacity(self.dialects.len());
        for dialect in self.dialects.iter() {
            for metaschema in &dialect.metaschemas {
                result.push(SrcValue::Value(
                    metaschema.0.clone(),
                    metaschema.1.clone().into(),
                ));
            }
        }
        result
    }

    /// Returns a slice of [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn as_slice(&self) -> &[Dialect<Key>] {
        &self.dialects
    }

    /// Returns the index (`usize`) of the default.
    /// [`Dialect`](`crate::dialect::Dialect`)
    #[must_use]
    pub fn default_index(&self) -> usize {
        self.primary
    }
    /// Returns an [`Iterator`] over the [`Dialect`]s.
    pub fn iter(&'d self) -> std::slice::Iter<'d, Dialect<Key>> {
        self.dialects.iter()
    }

    /// Returns the primary [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn primary_dialect(&self) -> &Dialect<Key> {
        &self.dialects[self.primary]
    }

    /// Sets the primary [`Dialect`] to use when no other [`Dialect`] matches.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set_primary_dialect_index(&mut self, index: usize) {
        assert!(index < self.dialects.len());
        self.primary = index;
    }

    /// Returns the number of [`Dialect`]s.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.dialects.len()
    }

    #[must_use]
    pub fn primary(&self) -> &Dialect<Key> {
        &self.dialects[self.primary]
    }
    /// Returns the index of the given [`Dialect`] in the list of [`Dialect`]s.
    #[must_use]
    pub fn position(&self, dialect: &Dialect<Key>) -> Option<usize> {
        self.dialects.iter().position(|d| d == dialect)
    }

    #[must_use]
    pub fn get_by_index(&self, idx: usize) -> Option<&Dialect<Key>> {
        self.dialects.get(idx)
    }

    #[must_use]
    pub fn dialect_index_for(&self, schema: &Value) -> usize {
        let default = self.primary_dialect();
        if default.is_pertinent_to(schema) {
            return self.primary;
        }
        for (idx, dialect) in self.dialects.iter().enumerate() {
            if dialect.id != default.id && dialect.is_pertinent_to(schema) {
                return idx;
            }
        }
        self.primary
    }

    fn find_primary(
        dialects: &[Dialect<Key>],
        lookup: &HashMap<AbsoluteUri, usize>,
        default: Option<&AbsoluteUri>,
    ) -> Result<usize, DialectError> {
        let uri = default.unwrap_or(&dialects[0].id);
        lookup
            .get(uri)
            .copied()
            .ok_or(DialectError::DefaultNotFound(uri.clone()))
    }
}

impl<'d, Key> IntoIterator for &'d Dialects<'d, Key>
where
    Key: 'static + slotmap::Key,
{
    type Item = &'d Dialect<Key>;

    type IntoIter = std::slice::Iter<'d, Dialect<Key>>;

    fn into_iter(self) -> Self::IntoIter {
        self.dialects.iter()
    }
}

#[cfg(test)]
mod tests {}
