//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::error::{AnchorError, DialectExistsError, UriError};
use crate::keyword::Keyword;
use crate::schema::Metaschema;

use crate::{
    error::{DialectError, IdentifyError},
    uri::AbsoluteUri,
    Object, Src,
};
use jsonptr::Pointer;
use serde_json::{json, Value};
use std::panic::{self, UnwindSafe};
use std::{
    borrow::{Borrow, Cow},
    collections::{HashMap, HashSet},
    convert::Into,
    fmt::Debug,
    hash::Hash,
    iter::IntoIterator,
    ops::Deref,
};

use super::{Anchor, Identifier, Reference};

/// A set of keywords and semantics which are used to evaluate a [`Value`](serde_json::Value) against a
/// schema.
#[derive(Clone)]
pub struct Dialect {
    /// Identifier of the `Dialect`. A meta schema must be defined in
    /// `metaschemas` with this `id`.
    id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    metaschemas: HashMap<AbsoluteUri, Object>,
    /// Set of [`Keyword`]s defined by the dialect.
    keywords: Vec<Keyword>,
    is_pertinent_to_index: usize,
    identify_indexes: Vec<usize>,
}
impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl Dialect {
    /// Creates a new [`Dialect`].
    pub fn new<S, M, I, H>(
        id: AbsoluteUri,
        metaschemas: M,
        keywords: H,
    ) -> Result<Self, DialectError>
    where
        S: Borrow<Metaschema>,
        M: IntoIterator<Item = S>,
        I: Into<Keyword>,
        H: IntoIterator<Item = I>,
    {
        let metaschemas: HashMap<AbsoluteUri, Object> = metaschemas
            .into_iter()
            .map(|m| {
                let m = m.borrow();
                (m.id.clone(), m.schema.clone())
            })
            .collect();
        let keywords = keywords
            .into_iter()
            .map(Into::into)
            .collect::<Vec<Keyword>>();
        let Some(is_pertinent_to_index) = Self::find_is_pertinent_to_index(&keywords) else { return Err(DialectError::IsPertinentToNotImplemented(id.clone())); };
        let identify_indexes = Self::find_identify_index(&keywords);
        if identify_indexes.is_empty() {
            return Err(DialectError::IdentifyNotImplemented(id.clone()));
        }
        if !metaschemas.contains_key(&id) {
            return Err(DialectError::DefaultNotFound(id.clone()));
        }
        Ok(Self {
            id,
            metaschemas,
            keywords,
            is_pertinent_to_index,
            identify_indexes,
        })
    }

    fn find_is_pertinent_to_index(keywords: &[Keyword]) -> Option<usize> {
        keywords
            .iter()
            .enumerate()
            .find(|(_, keyword)| is_implemented(|schema| keyword.is_pertinent_to(schema)))
            .map(|(idx, _)| idx)
    }

    fn find_identify_index(keywords: &[Keyword]) -> Vec<usize> {
        let mut result = Vec::new();
        for (idx, keyword) in keywords.iter().enumerate() {
            #[allow(clippy::blocks_in_if_conditions)]
            if is_implemented(|schema| {
                let _ = keyword.identify(schema);
            }) {
                result.push(idx);
            }
        }
        result
    }
    /// Attempts to identify a `schema` based on the [`Keyword`]s associated with
    /// this `Dialect`, returning the primary (if any) and all [`AbsoluteUri`]s
    /// the [`Schema`](`crate::schema::Schema`) can be referenced by.
    ///
    /// # Convention
    /// The second (index: `1`) `Keyword` must implement `identify` and, if
    /// able, return the primary identifier.
    ///
    /// Secondary identifiers are determined by
    /// [`Keyword`](crate::keyword::Keyword)s index `2` and greater.
    pub fn identify(
        &self,
        mut base_uri: AbsoluteUri,
        path: &Pointer,
        schema: &Value,
    ) -> Result<(Option<AbsoluteUri>, Vec<AbsoluteUri>), IdentifyError> {
        let mut uris = Vec::new();
        base_uri.set_fragment(Some(path))?;
        uris.push(base_uri.clone());
        // attempt to find a primary id
        let mut primary = None;
        for idx in &self.identify_indexes {
            if let Some(Identifier::Primary(id)) = self.keywords[*idx].identify(schema)? {
                let uri = base_uri.resolve(&id)?;
                primary = Some(uri);
                break;
            }
        }
        // if a primary identifier was found, use it as the base_uri for
        // subschemas. Otherwise, use the base_uri provided by the caller
        // with the fragment set to the json pointer `path`.
        let base_uri = primary.as_ref().unwrap_or(&base_uri);
        for idx in &self.identify_indexes {
            let keyword = &self.keywords[*idx];
            if let Some(id) = keyword.identify(schema)? {
                let uri = base_uri.resolve(id.uri())?;
                uris.push(uri);
            }
        }
        Ok((primary, uris))
    }

    #[must_use]
    pub fn primary_metaschema_id(&self) -> &AbsoluteUri {
        &self.id
    }

    /// Attempts to locate nested schemas within `source` by calling
    /// [`Keyword::subschemas`](`crate::Keyword::subschemas`) for each attached
    /// `Keyword` of this `Dialect`.
    ///
    #[must_use]
    pub fn subschemas(&self, path: &Pointer, source: &Value) -> HashSet<Pointer> {
        let mut locations = HashSet::new();
        for keyword in &self.keywords {
            locations.extend(keyword.subschemas(path, source));
        }
        locations
    }

    pub fn references(&self, source: &Value) -> Result<Vec<Reference>, UriError> {
        let mut refs = Vec::new();
        for keyword in &self.keywords {
            refs.append(&mut keyword.references(source)?);
        }
        Ok(refs)
    }

    pub fn anchors(&self, source: &Value) -> Result<Vec<Anchor>, AnchorError> {
        let mut anchors = Vec::new();
        for keyword in &self.keywords {
            anchors.append(&mut keyword.anchors(source)?);
        }
        Ok(anchors)
    }

    /// Determines if the schema is pertinent to this `Dialect`.
    ///
    /// # Convention
    /// Exactly one `Keyword` must implement `is_pertinent_to` for a given
    ///
    /// `Dialect`. It **must** be the **first** (index: `0`) `Keyword` in the
    /// [`Dialect`](`crate::dialect::Dialect`)'s
    /// [`Keywords`](`crate::dialect::Keywords`).
    #[must_use]
    pub fn is_pertinent_to(&self, schema: &Value) -> bool {
        self.keywords[self.is_pertinent_to_index].is_pertinent_to(schema)
    }

    #[must_use]
    /// Returns the [`AbsoluteUri`] of this `Dialect`.
    pub fn id(&self) -> &AbsoluteUri {
        &self.id
    }

    #[must_use]
    /// Returns the metaschemas of this `Dialect`.
    pub fn metaschemas(&self) -> &HashMap<AbsoluteUri, Object> {
        &self.metaschemas
    }

    #[must_use]
    /// Returns the [`Keyword`]s of this `Dialect`.
    pub fn keywords(&self) -> &[Keyword] {
        self.keywords.as_ref()
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
            .field("keywords", &self.keywords)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Dialects<'d> {
    dialects: Cow<'d, [Dialect]>,
    lookup: Cow<'d, HashMap<AbsoluteUri, usize>>,
    primary: usize,
}

impl<'d> Deref for Dialects<'d> {
    type Target = [Dialect];

    fn deref(&self) -> &Self::Target {
        self.dialects.as_ref()
    }
}

impl<'d> Dialects<'d> {
    pub fn new(
        dialects: Vec<Dialect>,
        default: Option<&AbsoluteUri>,
    ) -> Result<Self, DialectError> {
        if dialects.is_empty() {
            return Err(DialectError::Empty);
        }
        let mut collected: Vec<Dialect> = Vec::with_capacity(dialects.len());
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
    pub fn get(&self, id: &AbsoluteUri) -> Option<&Dialect> {
        self.lookup.get(id).map(|&index| &self.dialects[index])
    }
    /// Returns the [`Dialect`] that is determined pertinent to the schema based
    /// upon the first [`Keyword`] in each
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
    /// Returns the [`DialectExists`] if a `Dialect` already exists with the same `id`.
    pub(crate) fn push(&mut self, dialect: Dialect) -> Result<(), DialectExistsError> {
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
    pub fn with_default<'o>(&'d self, default: &'o Dialect) -> Result<Dialects<'d>, &'o Dialect> {
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
    pub fn pertinent_to_or_default(&self, schema: &Value) -> &Dialect {
        self.pertinent_to(schema).unwrap_or(self.primary_dialect())
    }
    /// Returns an [`Iterator`] of [`&AbsoluteUri`](`crate::uri::AbsoluteUri`) for each metaschema in each [`Dialect`](`crate::dialect::Dialect`).
    pub fn source_ids(&self) -> impl Iterator<Item = &AbsoluteUri> {
        self.dialects.iter().map(|d| &d.id)
    }

    #[must_use]
    pub fn sources(&self) -> Vec<Src> {
        let mut result = Vec::with_capacity(self.dialects.len());
        for dialect in self.dialects.iter() {
            for metaschema in &dialect.metaschemas {
                result.push(Src::Value(
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
        self.primary
    }
    /// Returns an [`Iterator`] over the [`Dialect`]s.
    pub fn iter(&'d self) -> std::slice::Iter<'d, Dialect> {
        self.dialects.iter()
    }

    /// Returns the primary [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn primary_dialect(&self) -> &Dialect {
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
    pub fn primary(&self) -> &Dialect {
        &self.dialects[self.primary]
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
        dialects: &[Dialect],
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

impl<'d> IntoIterator for &'d Dialects<'d> {
    type Item = &'d Dialect;

    type IntoIter = std::slice::Iter<'d, Dialect>;

    fn into_iter(self) -> Self::IntoIter {
        self.dialects.iter()
    }
}

fn is_implemented<M, R>(method: M) -> bool
where
    R: std::panic::UnwindSafe,
    M: UnwindSafe + FnOnce(&Value) -> R,
{
    let empty = json!({});
    let result = panic::catch_unwind(|| {
        let _ = method(&empty);
    });
    result.is_ok()
}
#[cfg(test)]
mod tests {}
