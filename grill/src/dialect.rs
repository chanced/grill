//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

mod identify_schema;
mod is_schema;
mod locate_schemas;

pub use identify_schema::IdentifySchema;
pub use is_schema::IsSchema;
use jsonptr::Pointer;
pub use locate_schemas::{LocateSchemas, LocatedSchema};

use crate::{
    error::{
        BuildError, DefaultDialectNotFoundError, DuplicateDialectError, EmptyDialectsError,
        FragmentedDialectIdError, IdentifyError, LocateSchemasError, NewDialectsError,
    },
    uri::AbsoluteUri,
    Handler, Metaschema, Object, Source, Uri,
};
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

/// A `Dialect` is a set of keywords and semantics that can be used to evaluate
/// a value against a schema.
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

impl Dialect {
    /// Creates a new [`Dialect`].
    pub fn new<S, M, I, H>(id: impl Borrow<AbsoluteUri>, meta_schemas: M, handlers: H) -> Self
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
        let id = id.borrow().clone();

        Self {
            id,
            metaschemas,
            handlers,
        }
    }

    pub(crate) fn id_and_handlers(&self) -> (AbsoluteUri, Vec<Handler>) {
        (self.id.clone(), self.handlers.clone())
    }

    /// Attempts to identify a `schema` based on the `Handler`s associated with
    /// this `Dialect`.
    pub fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        for handler in &self.handlers {
            if let Some(uri) = handler.identify_schema(schema)? {
                return Ok(Some(uri));
            }
        }
        Ok(None)
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
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        todo!()
    }

    #[must_use]
    pub fn matches(&self, schema: &Value) -> bool {
        todo!()
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
pub struct Dialects {
    dialects: Vec<Dialect>,
    default: usize,
    lookup: HashMap<AbsoluteUri, usize>,
}
impl<'d> IntoIterator for &'d Dialects {
    type Item = &'d Dialect;

    type IntoIter = std::slice::Iter<'d, Dialect>;

    fn into_iter(self) -> Self::IntoIter {
        self.dialects.iter()
    }
}

impl Deref for Dialects {
    type Target = [Dialect];

    fn deref(&self) -> &Self::Target {
        self.dialects.as_ref()
    }
}

impl Dialects {
    fn queue(dialects: Vec<Dialect>) -> Result<Vec<Dialect>, NewDialectsError> {
        let mut queue = Vec::with_capacity(dialects.len());
        let mut indexed = HashSet::with_capacity(dialects.len());
        for dialect in dialects.into_iter().rev() {
            if let Some(fragment) = dialect.id.fragment() {
                if !fragment.is_empty() {
                    return Err(FragmentedDialectIdError {
                        id: dialect.id.clone(),
                    }
                    .into());
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
    fn index(
        dialects: Vec<Dialect>,
    ) -> Result<(Vec<Dialect>, HashMap<AbsoluteUri, usize>), NewDialectsError> {
        let queue = Self::queue(dialects)?;
        let mut dialects = Vec::with_capacity(queue.len());
        let mut lookup: HashMap<AbsoluteUri, usize> = HashMap::with_capacity(queue.len());
        for dialect in queue.into_iter().rev() {
            if lookup.contains_key(&dialect.id) {
                return Err(DuplicateDialectError::new(dialect).into());
            }
            let id = dialect.id.clone();
            dialects.push(dialect);
            lookup.insert(id, dialects.len() - 1);
        }
        Ok((dialects, lookup))
    }
    fn find_default(
        dialects: &[Dialect],
        lookup: &HashMap<AbsoluteUri, usize>,
        default: Option<AbsoluteUri>,
    ) -> Result<usize, DefaultDialectNotFoundError> {
        let uri = default.unwrap_or(dialects[0].id.clone());
        lookup
            .get(&uri)
            .copied()
            .ok_or(DefaultDialectNotFoundError { uri })
    }

    pub fn new(
        dialects: Vec<Dialect>,
        default: Option<AbsoluteUri>,
    ) -> Result<Self, NewDialectsError> {
        if dialects.is_empty() {
            return Err(EmptyDialectsError.into());
        }
        let (dialects, lookup) = Self::index(dialects)?;
        let default = Self::find_default(&dialects, &lookup, default)?;
        Ok(Self {
            dialects,
            default,
            lookup,
        })
    }

    #[must_use]
    pub fn sources(&self) -> Vec<Source> {
        let mut result = Vec::with_capacity(self.dialects.len());
        for dialect in &self.dialects {
            for metaschema in &dialect.metaschemas {
                result.push(Source::Value(
                    metaschema.0.clone(),
                    metaschema.1.clone().into(),
                ));
            }
        }
        result
    }

    // #[must_use]
    // pub fn new(dialects: impl IntoIterator<Item = Dialect>, default_dialect: &Dialect) -> Self {
    //     // todo: do this in one pass
    //     let dialects = dialects.into_iter().collect::<Vec<_>>();
    //     let default_dialect = dialects
    //         .iter()
    //         .position(|d| d == default_dialect)
    //         .expect("default_dialect must be in list of dialects");
    //     Self {
    //         dialects,
    //         default: default_dialect,
    //     }
    // }

    #[must_use]
    pub fn as_slice(&self) -> &[Dialect] {
        &self.dialects
    }
    #[must_use]
    pub fn default_dialect_index(&self) -> usize {
        self.default
    }

    /// Sets the default [`Dialect`] to use when no other [`Dialect`] matches.
    ///
    /// # Panics
    /// Panics if the default dialect is not in the list of dialects.
    pub fn set_default_dialect_index(&mut self, dialect: usize) {
        assert!(dialect < self.dialects.len());
        self.default = dialect;
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
    pub fn get(&self, idx: usize) -> Option<&Dialect> {
        self.dialects.get(idx)
    }
    #[must_use]
    pub fn dialect_index_for(&self, schema: &Value) -> usize {
        let default = self.default_dialect();
        if default.matches(schema) {
            return self.default;
        }
        for (idx, dialect) in self.dialects.iter().enumerate() {
            if dialect.id != default.id && dialect.matches(schema) {
                return idx;
            }
        }
        self.default
    }
}

#[cfg(test)]
mod tests {}
