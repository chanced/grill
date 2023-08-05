//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::{
    error::{DialectError, IdentifyError, LocateSchemasError, UriError},
    schema::{LocatedSchema, LocatedSchemas},
    uri::AbsoluteUri,
    Handler, Object, Source,
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

/// A set of keywords and semantics which are used to evaluate a [`Value`](serde_json::Value) against a
/// schema.
#[derive(Clone)]
pub struct Dialect {
    /// Identifier of the `Dialect`. A meta schema must be defined in
    /// `metaschemas` with this `id`.
    pub id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    pub metaschemas: HashMap<AbsoluteUri, Object>,
    /// Set of [`Handler`]s defined by the dialect.
    pub handlers: Vec<Handler>,
}
impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl Dialect {
    /// Creates a new [`Dialect`].
    pub fn new<S, M, I, H>(id: AbsoluteUri, meta_schemas: M, handlers: H) -> Self
    where
        S: Borrow<Metaschema>,
        M: IntoIterator<Item = S>,
        I: Into<Handler>,
        H: IntoIterator<Item = I>,
    {
        let metaschemas = meta_schemas
            .into_iter()
            .map(|m| {
                let m = m.borrow();
                (m.id.clone(), m.schema.clone())
            })
            .collect();
        let handlers = handlers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Handler>>();

        Self {
            id,
            metaschemas,
            handlers,
        }
    }

    /// Attempts to identify a `schema` based on the `Handler`s associated with
    /// this `Dialect`.
    pub fn identify(
        &self,
        base_uri: &AbsoluteUri,
        schema: &Value,
    ) -> Result<Option<AbsoluteUri>, IdentifyError> {
        let id = self.handlers[1].identify(schema)?;
        let Some(id) = id else { return Ok(None)};
        match id {
            crate::Uri::Url(url) => Ok(Some(AbsoluteUri::Url(url))),
            crate::Uri::Urn(urn) => Ok(Some(AbsoluteUri::Urn(urn))),
            crate::Uri::Relative(rel) => {
                let mut uri = base_uri.clone();
                uri.set_fragment(rel.fragment())?;
                uri.set_path_or_nss(rel.path()).map_err(UriError::from)?;
                uri.set_query(rel.query()).map_err(UriError::from)?;
                Ok(Some(uri))
            }
        }
    }

    /// Attempts to locate nested schemas within `value` by means of attached
    /// `Handler`s of this `Dialect`.
    ///
    /// Not all URIs may be identified for each schema. For example, the schema:
    /// ```json
    /// {
    ///     "$schema": "https://json-schema.org/draft/2020-12/schema",
    ///     "$id": "https://example.com/example.json",
    ///     "$defs": {
    ///         "foo": {
    ///             "$id": "https://example.com/foo.json",
    ///             "$defs": {
    ///                 "bar": {
    ///                     "$id": "https://example.com/bar.json",
    ///                     "$defs": {
    ///                         "$anchor": : "anchor",
    ///                     }
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    /// may only identify following URIs:
    /// - `https://example.com/example.json`
    /// - `https://example.com/foo.json`
    /// - `https://example.com/bar.json`
    /// - `https://example.com/bar.json#anchor`
    ///
    /// The remainder are resolved upon use.
    ///
    /// # Examples
    /// ## [JSON Schema 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#name-schema-identification-examp)
    ///
    /// ```json
    /// {
    ///     "$id": "https://example.com/root.json",
    ///     "$defs": {
    ///         "A": { "$anchor": "foo" },
    ///         "B": {
    ///             "$id": "other.json",
    ///             "$defs": {
    ///                 "X": { "$anchor": "bar" },
    ///                 "Y": {
    ///                     "$id": "t/inner.json",
    ///                     "$anchor": "bar"
    ///                 }
    ///             }
    ///         },
    ///         "C": {
    ///             "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// The schemas at the following URI-encoded JSON Pointers (relative to the root
    /// schema) have the following base URIs, and are identifiable by any listed
    /// URI:
    ///
    /// - `#` (document root) canonical (and base) URI
    ///   `https://example.com/root.json`
    ///     - canonical resource URI plus pointer fragment
    ///       `https://example.com/root.json#`
    /// - `#/$defs/A` base URI `https://example.com/root.json`
    ///     - canonical resource URI plus plain fragment
    ///       `https://example.com/root.json#foo`
    ///     - canonical resource URI plus pointer fragment
    ///       `https://example.com/root.json#/$defs/A`
    /// - `#/$defs/B` canonical (and base) URI `https://example.com/other.json`
    ///     - canonical resource URI plus pointer fragment
    ///       `https://example.com/other.json#`
    ///     - base URI of enclosing (root.json) resource plus fragment
    ///       `https://example.com/root.json#/$defs/B`
    /// - `#/$defs/B/$defs/X` base URI `https://example.com/other.json`
    ///     - canonical resource URI plus plain fragment
    ///       `https://example.com/other.json#bar`
    ///     - canonical resource URI plus pointer fragment
    ///       `https://example.com/other.json#/$defs/X`
    ///     - base URI of enclosing (root.json) resource plus fragment
    ///       `https://example.com/root.json#/$defs/B/$defs/X`
    /// - `#/$defs/B/$defs/Y` canonical (and base) URI
    ///   `https://example.com/t/inner.json`
    ///     - canonical URI plus plain fragment
    ///       `https://example.com/t/inner.json#bar`
    ///     - canonical URI plus pointer fragment
    ///       `https://example.com/t/inner.json#`
    ///     - base URI of enclosing (other.json) resource plus fragment
    ///       `https://example.com/other.json#/$defs/Y`
    ///     - base URI of enclosing (root.json) resource plus fragment
    ///       `https://example.com/root.json#/$defs/B/$defs/Y`
    /// - `#/$defs/C`
    ///     - canonical (and base) URI
    ///       `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f`
    ///     - canonical URI plus pointer fragment
    ///       `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f#`
    ///     - base URI of enclosing (root.json) resource plus fragment
    ///       `https://example.com/root.json#/$defs/C`
    ///
    /// ## [JSON Schema 07](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-8.2.4)
    /// ```json
    /// {
    ///     "$id": "http://example.com/root.json",
    ///     "definitions": {
    ///         "A": { "$id": "#foo" },
    ///         "B": {
    ///             "$id": "other.json",
    ///             "definitions": {
    ///                 "X": { "$id": "#bar" },
    ///                 "Y": { "$id": "t/inner.json" }
    ///             }
    ///         },
    ///         "C": {
    ///             "$id": "urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f"
    ///         }
    ///     }
    /// }
    /// ```
    /// -   `#` (document root)
    ///     -   `http://example.com/root.json`
    ///     -   `http://example.com/root.json#`
    /// -   `#/definitions/A`
    ///     -   `http://example.com/root.json#foo`
    ///     -   `http://example.com/root.json#/definitions/A`
    /// -   `#/definitions/B`
    ///     -   `http://example.com/other.json`
    ///         -   `http://example.com/other.json#`
    ///     -   `http://example.com/root.json#/definitions/B`
    /// -   `#/definitions/B/definitions/X`
    ///     -   `http://example.com/other.json#bar`
    ///         -   `http://example.com/other.json#/definitions/X`
    ///         -   `http://example.com/root.json#/definitions/B/definitions/X`
    /// -   `#/definitions/B/definitions/Y`
    ///     -   `http://example.com/t/inner.json`
    ///     -   `http://example.com/t/inner.json#`
    ///     -   `http://example.com/other.json#/definitions/Y`
    ///     -   `http://example.com/root.json#/definitions/B/definitions/Y`
    /// -   `#/definitions/C`
    ///     -   `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f`
    ///     -   `urn:uuid:ee564b8a-7a87-4125-8c96-e9f123d6766f#`
    ///     -   `http://example.com/root.json#/definitions/C`
    pub fn locate_schemas<'v>(
        &self,
        path: &Pointer,
        base_uri: &AbsoluteUri,
        value: &'v Value,
        dialects: &Dialects,
    ) -> Result<LocatedSchemas<'v>, LocateSchemasError> {
        if let Some(dialect) = dialects.pertinent_to(value) {
            if dialect != self {
                return dialect.locate_schemas(path, base_uri, value, dialects);
            }
        }
        let uri = self.identify(base_uri, value)?.unwrap_or(base_uri.clone());

        let mut located = vec![LocatedSchema {
            keyword: None,
            path: path.clone(),
            uri: uri.clone(),
            value,
        }];

        for handler in &self.handlers {
            located.append(&mut handler.schemas(path, &uri, value, dialects)?);
        }

        Ok(located.into())
    }

    /// Determines if the schema is pertinent to this `Dialect`.
    ///
    /// # Convention
    /// Exactly one `Handler` must implement `is_pertinent_to` for a given
    /// `Dialect`. It **must** be the **first** (index: `0`) `Handler` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Handlers`](`crate::dialect::Handlers`).
    #[must_use]
    pub fn is_pertinent_to(&self, schema: &Value) -> bool {
        self.handlers[0].is_pertinent_to(schema)
    }
}

impl PartialEq for Dialect {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Hash for Dialect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl Debug for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dialect")
            .field("id", &self.id)
            .field("meta_schemas", &self.metaschemas)
            .field("handlers", &self.handlers)
            .finish_non_exhaustive()
    }
}
#[derive(Debug, Clone)]
pub struct Dialects<'d> {
    dialects: Cow<'d, [Dialect]>,
    lookup: Cow<'d, HashMap<AbsoluteUri, usize>>,
    default: usize,
}
impl<'d> Dialects<'d> {
    pub fn new(dialects: Vec<Dialect>, default: Option<AbsoluteUri>) -> Result<Self, DialectError> {
        if dialects.is_empty() {
            return Err(DialectError::Empty);
        }
        let queue = Self::queue(dialects)?;
        let mut dialects = Vec::with_capacity(queue.len());
        let mut lookup: HashMap<AbsoluteUri, usize> = HashMap::with_capacity(queue.len());

        for dialect in queue.into_iter().rev() {
            if lookup.contains_key(&dialect.id) {
                return Err(DialectError::Duplicate(dialect));
            }
            let id = dialect.id.clone();
            dialects.push(dialect);
            lookup.insert(id, dialects.len() - 1);
        }

        let default = Self::find_default(&dialects, &lookup, default)?;
        Ok(Self {
            dialects: Cow::Owned(dialects),
            lookup: Cow::Owned(lookup),
            default,
        })
    }

    /// Returns the [`Dialect`](crate::dialect::Dialect).
    #[must_use]
    pub fn get(&self, id: &AbsoluteUri) -> Option<&Dialect> {
        self.lookup.get(id).map(|&index| &self.dialects[index])
    }
    /// Returns the [`Dialect`] that is determined pertinent to the schema based
    /// upon the first [`Handler`] in each
    /// [`Dialect`](`crate::dialect::Dialect`) or `None` if a [`Dialect`] cannot
    /// be confidently determined.
    #[must_use]
    pub fn pertinent_to(&self, schema: &Value) -> Option<&Dialect> {
        self.dialects
            .iter()
            .find(|&dialect| dialect.is_pertinent_to(schema))
    }

    /// Appends a [`Dialect`].
    ///
    /// # Errors
    /// Returns the `Dialect` if a `Dialect` already exists with the same `id`.
    pub(crate) fn push(&mut self, dialect: Dialect) -> Result<(), Dialect> {
        if self.lookup.contains_key(&dialect.id) {
            return Err(dialect);
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
    pub fn with_default<'o>(&'d self, default: &'o Dialect) -> Result<Dialects<'d>, &'o Dialect> {
        let default = self.lookup.get(&default.id).copied().ok_or(default)?;
        Ok(Dialects {
            dialects: Cow::Borrowed(self.dialects.as_ref()),
            default,
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
    pub fn pertinent_to_or_default(&self, schema: &Value) -> &Dialect {
        self.pertinent_to(schema).unwrap_or(self.default())
    }
    /// Returns an [`Iterator`] of [`&AbsoluteUri`](`crate::uri::AbsoluteUri`) for each metaschema in each [`Dialect`](`crate::dialect::Dialect`).
    pub fn source_ids(&self) -> impl Iterator<Item = &AbsoluteUri> {
        self.dialects.iter().map(|d| &d.id)
    }

    #[must_use]
    pub fn sources(&self) -> Vec<Source> {
        let mut result = Vec::with_capacity(self.dialects.len());
        for dialect in self.dialects.iter() {
            for metaschema in &dialect.metaschemas {
                result.push(Source::Value(
                    metaschema.0.clone(),
                    metaschema.1.clone().into(),
                ));
            }
        }
        result
    }

    /// Returns a slice of [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn as_slice(&self) -> &[Dialect] {
        &self.dialects
    }

    /// Returns the index (`usize`) of the default.
    /// [`Dialect`](`crate::dialect::Dialect`)
    #[must_use]
    pub fn default_index(&self) -> usize {
        self.default
    }
    /// Returns an [`Iterator`] over the [`Dialect`]s.
    pub fn iter(&'d self) -> std::slice::Iter<'d, Dialect> {
        self.dialects.iter()
    }

    /// Returns the default [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn default(&self) -> &Dialect {
        &self.dialects[self.default]
    }
    /// Sets the default [`Dialect`] to use when no other [`Dialect`] matches.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub fn set_default_dialect_index(&mut self, index: usize) {
        assert!(index < self.dialects.len());
        self.default = index;
    }

    /// Returns the number of [`Dialect`]s.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.dialects.len()
    }

    #[must_use]
    pub fn default_dialect(&self) -> &Dialect {
        &self.dialects[self.default]
    }
    /// Returns the index of the given [`Dialect`] in the list of [`Dialect`]s.
    #[must_use]
    pub fn position(&self, dialect: &Dialect) -> Option<usize> {
        self.dialects.iter().position(|d| d == dialect)
    }

    #[must_use]
    pub fn get_by_index(&self, idx: usize) -> Option<&Dialect> {
        self.dialects.get(idx)
    }

    #[must_use]
    pub fn dialect_index_for(&self, schema: &Value) -> usize {
        let default = self.default_dialect();
        if default.is_pertinent_to(schema) {
            return self.default;
        }
        for (idx, dialect) in self.dialects.iter().enumerate() {
            if dialect.id != default.id && dialect.is_pertinent_to(schema) {
                return idx;
            }
        }
        self.default
    }

    fn queue(dialects: Vec<Dialect>) -> Result<Vec<Dialect>, DialectError> {
        let mut queue = Vec::with_capacity(dialects.len());
        let mut indexed = HashSet::with_capacity(dialects.len());
        for dialect in dialects.into_iter().rev() {
            if let Some(fragment) = dialect.id.fragment() {
                if !fragment.is_empty() {
                    return Err(DialectError::FragmentedId(dialect.id.clone()));
                }
            }
            if indexed.contains(&dialect.id) {
                continue;
            }
            let id = dialect.id.clone();
            queue.push(dialect);
            indexed.insert(id);
        }
        Ok(queue)
    }

    fn find_default(
        dialects: &[Dialect],
        lookup: &HashMap<AbsoluteUri, usize>,
        default: Option<AbsoluteUri>,
    ) -> Result<usize, DialectError> {
        let uri = default.unwrap_or(dialects[0].id.clone());
        lookup
            .get(&uri)
            .copied()
            .ok_or(DialectError::DefaultNotFound(uri))
    }
}

impl<'d> IntoIterator for &'d Dialects<'d> {
    type Item = &'d Dialect;

    type IntoIter = std::slice::Iter<'d, Dialect>;

    fn into_iter(self) -> Self::IntoIter {
        self.dialects.iter()
    }
}

impl<'d> Deref for Dialects<'d> {
    type Target = [Dialect];

    fn deref(&self) -> &Self::Target {
        self.dialects.as_ref()
    }
}

pub struct Metaschema {
    pub id: AbsoluteUri,
    pub schema: Object,
}

impl Metaschema {
    #[must_use]
    pub fn new(id: AbsoluteUri, schema: Object) -> Self {
        Self { id, schema }
    }
}

#[cfg(test)]
mod tests {}
