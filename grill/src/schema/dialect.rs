//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::{
    error::{
        AnchorError, DialectError, DialectExistsError, DialectsError, IdentifyError, Unimplemented,
        UriError,
    },
    keyword::Keyword,
    uri::AbsoluteUri,
    Src,
};
use jsonptr::Pointer;
use serde_json::{json, Value};
use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    convert::Into,
    fmt::Debug,
    hash::Hash,
    iter::IntoIterator,
    ops::Deref,
};

use super::{Anchor, Reference};

pub struct Builder {
    id: AbsoluteUri,
    metaschemas: Vec<(AbsoluteUri, Value)>,
    keywords: Vec<Box<dyn Keyword>>,
}

impl Builder {
    #[must_use]
    pub fn with_metaschema(mut self, id: AbsoluteUri, schema: Value) -> Self {
        self.metaschemas.push((id, schema));
        self
    }

    #[must_use]
    pub fn with_keyword(mut self, keyword: impl 'static + Keyword) -> Self {
        self.keywords.push(Box::new(keyword));
        self
    }
    pub fn build(mut self) -> Result<Dialect, DialectError> {
        Dialect::new(self.id, self.metaschemas, self.keywords)
    }
}

/// A set of keywords and semantics which are used to evaluate a [`Value`](serde_json::Value) against a
/// schema.
#[derive(Clone)]
pub struct Dialect {
    /// Identifier of the `Dialect`. A meta schema must be defined in
    /// `metaschemas` with this `id`.
    id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    metaschemas: HashMap<AbsoluteUri, Value>,
    /// Set of [`Keyword`]s defined by the dialect.
    keywords: Box<[Box<dyn Keyword>]>,
    is_pertinent_to_index: u16,
    identify_indexes: Box<[u16]>,
    dialect_indexes: Box<[u16]>,
    subschemas_indexes: Box<[u16]>,
    anchors_indexes: Box<[u16]>,
    references_indexes: Box<[u16]>,
}
impl std::fmt::Display for Dialect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl Dialect {
    /// Returns a new `Dialect` [`Builder`].
    #[must_use]
    pub fn builder(id: AbsoluteUri) -> Builder {
        Builder {
            id,
            metaschemas: Vec::new(),
            keywords: Vec::new(),
        }
    }

    /// Creates a new [`Dialect`].
    fn new(
        id: AbsoluteUri,
        metaschemas: Vec<(AbsoluteUri, Value)>,
        keywords: Vec<Box<dyn Keyword>>,
    ) -> Result<Self, DialectError> {
        let metaschemas: HashMap<AbsoluteUri, Value> = metaschemas.into_iter().collect();
        let keywords = keywords.into_boxed_slice();

        let identify_indexes = find_identify_indexes(&id, &keywords)?;
        let is_pertinent_to_index = find_is_pertinent_to_index(&id, &keywords)?;
        let dialect_indexes = find_dialect_indexes(&id, &keywords)?;
        let subschemas_indexes = find_impl_indexes(&keywords, Keyword::subschemas);
        let anchors_indexes = find_impl_indexes(&keywords, Keyword::anchors);
        let references_indexes = find_impl_indexes(&keywords, Keyword::references);

        if !metaschemas.contains_key(&id) {
            return Err(DialectError::DefaultNotFound(id.clone()));
        }

        Ok(Self {
            id,
            metaschemas,
            keywords,
            is_pertinent_to_index,
            identify_indexes,
            dialect_indexes,
            subschemas_indexes,
            anchors_indexes,
            references_indexes,
        })
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

        let mut primary = None;
        let mut secondary_uris = Vec::new();

        for idx in self.identify_indexes.iter().cloned() {
            let Some(id) = self.keywords[idx as usize].identify(schema).unwrap()? else { continue };
            if id.is_primary() && primary.is_none() {
                let uri = id.take_uri();
                base_uri = base_uri.resolve(&uri)?;
                primary = Some(base_uri.clone());
            } else {
                secondary_uris.push(id.take_uri());
            }
        }
        for uri in secondary_uris {
            uris.push(base_uri.resolve(&uri)?);
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
        self.subschemas_indexes
            .iter()
            .flat_map(|&idx| self.keywords[idx as usize].subschemas(source).unwrap())
            .map(|mut p| {
                let mut path = path.clone();
                path.append(&mut p);
                path
            })
            .collect()
    }

    pub fn references(&self, source: &Value) -> Result<Vec<Reference>, UriError> {
        let mut refs = Vec::new();
        for res in self
            .references_indexes
            .iter()
            .copied()
            .map(|idx| self.keywords[idx as usize].references(source).unwrap())
        {
            refs.append(&mut res?);
        }
        Ok(refs)
    }

    pub fn anchors(&self, source: &Value) -> Result<Vec<Anchor>, AnchorError> {
        let mut anchors = Vec::new();
        for res in self
            .references_indexes
            .iter()
            .copied()
            .map(|idx| self.keywords[idx as usize].anchors(source).unwrap())
        {
            anchors.append(&mut res?);
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
        self.keywords[self.is_pertinent_to_index as usize]
            .is_pertinent_to(schema)
            .unwrap()
    }

    #[must_use]
    /// Returns the [`AbsoluteUri`] of this `Dialect`.
    pub fn id(&self) -> &AbsoluteUri {
        &self.id
    }

    #[must_use]
    /// Returns the metaschemas of this `Dialect`.
    pub fn metaschemas(&self) -> &HashMap<AbsoluteUri, Value> {
        &self.metaschemas
    }

    #[must_use]
    /// Returns the [`Keyword`]s of this `Dialect`.
    pub fn keywords(&self) -> &[Box<dyn Keyword>] {
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
            .field("metaschemas", &self.metaschemas)
            .field("keywords", &self.keywords)
            .field("anchors_indexes", &self.anchors_indexes)
            .field("dialect_indexes", &self.dialect_indexes)
            .field("identify_indexes", &self.identify_indexes)
            .field("references_indexes", &self.references_indexes)
            .field("is_pertinent_to_index", &self.is_pertinent_to_index)
            .field("subschemas_indexes", &self.subschemas_indexes)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct Dialects<'i> {
    dialects: Cow<'i, [Cow<'static, Dialect>]>,
    default: usize,
}

impl<'d> Deref for Dialects<'d> {
    type Target = [Cow<'static, Dialect>];

    fn deref(&self) -> &Self::Target {
        self.dialects.as_ref()
    }
}

impl<'i> Dialects<'i> {
    pub fn new(
        dialects: Vec<Cow<Dialect>>,
        default: Option<&AbsoluteUri>,
    ) -> Result<Self, DialectsError> {
        if dialects.is_empty() {
            return Err(DialectsError::Empty);
        }
        let mut collected = Vec::with_capacity(dialects.len());
        let mut lookup: HashMap<AbsoluteUri, usize> = HashMap::with_capacity(dialects.len());
        for (i, dialect) in dialects.into_iter().enumerate() {
            if dialect.id.fragment().is_some() && dialect.id.fragment() != Some("") {
                return Err(DialectError::FragmentedId(dialect.id.clone()).into());
            }
            if lookup.contains_key(&dialect.id) {
                return Err(DialectsError::Duplicate(DialectExistsError {
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
            default,
        })
    }

    /// Returns the [`Dialect`](crate::dialect::Dialect).
    #[must_use]
    pub fn get(&self, id: &AbsoluteUri) -> Option<&Dialect> {
        self.dialects
            .iter()
            .find(|d| d.id == *id)
            .map(|d| d.deref())
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
            .map(|d| d.deref())
    }

    /// Appends a [`Dialect`].
    ///
    /// # Errors
    /// Returns the [`DialectExists`] if a `Dialect` already exists with the same `id`.
    pub fn push(&mut self, dialect: Cow<'static, Dialect>) -> Result<(), DialectExistsError> {
        if self.contains(&dialect.id) {
            return Err(DialectExistsError { id: dialect.id });
        }
        self.dialects.to_mut().push(dialect);
        Ok(())
    }

    /// Returns a new `Dialects` with the default `Dialect` set to the provided
    ///
    /// # Errors
    /// Returns the `Err(&'static str)` if the `Dialect` is not currently in this `Dialects`.
    pub fn with_default(self, default: &Dialect) -> Result<Dialects<'i>, &'static str> {
        let primary = self.get(&default.id).ok_or("dialect not found")?;
        Ok(Dialects {
            dialects: self.dialects,
            default: self.default,
        })
    }

    #[must_use]
    pub fn contains(&self, id: &AbsoluteUri) -> bool {
        self.dialects.iter().any(|d| &d.id == id)
    }

    /// Returns the [`Dialect`] that is pertinent to the schema or the default
    /// [`Dialect`] if the [`Dialect`] can not be determined from schema.
    #[must_use]
    pub fn pertinent_to_or_default(&self, schema: &Value) -> &Dialect {
        self.pertinent_to(schema).unwrap_or(self.primary())
    }

    #[must_use]
    pub fn pertinent_to_or_default_idx(&self, schema: &Value) -> usize {
        self.pertinent_to(schema)
            .map_or(self.default, |d| self.position(d).unwrap())
    }

    /// Returns an [`Iterator`] of [`&AbsoluteUri`](`crate::uri::AbsoluteUri`) for each metaschema in each [`Dialect`](`crate::dialect::Dialect`).
    pub fn source_ids(&self) -> impl Iterator<Item = &AbsoluteUri> {
        self.dialects.iter().map(|d| &d.id)
    }

    #[must_use]
    pub(crate) fn sources(&self) -> Vec<Src> {
        let mut result = Vec::with_capacity(self.dialects.len());
        for dialect in self.dialects.iter() {
            for metaschema in &dialect.metaschemas {
                result.push(Src::Value(metaschema.0.clone(), metaschema.1.clone()));
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
    pub fn iter(&'i self) -> std::slice::Iter<'i, Dialect> {
        self.dialects.iter()
    }

    /// Returns the primary [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn primary(&self) -> &Dialect {
        &self.dialects[self.default]
    }

    /// Sets the  [`Dialect`] to use when no other [`Dialect`] matches.
    ///
    /// # Panics
    /// Panics if the index is out of bounds.
    pub(crate) fn set_default_dialect_index(&mut self, index: usize) {
        assert!(index < self.dialects.len());
        self.default = index;
    }

    /// Returns the number of [`Dialect`]s.
    #[must_use]
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.dialects.len()
    }

    /// Returns the index of the given [`Dialect`] in the list of [`Dialect`]s.
    #[must_use]
    pub fn position(&self, dialect: &Dialect) -> Option<usize> {
        self.dialects.iter().position(|d| d.as_ref() == dialect)
    }

    #[must_use]
    pub fn get_by_index(&self, idx: usize) -> Option<&Dialect> {
        self.dialects.get(idx).map(AsRef::as_ref)
    }

    #[must_use]
    pub fn dialect_index_for(&self, schema: &Value) -> usize {
        let default = self.primary();
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

    fn find_primary(
        dialects: &[Cow<'static, Dialect>],
        lookup: &HashMap<AbsoluteUri, usize>,
        default: Option<&AbsoluteUri>,
    ) -> Result<usize, DialectError> {
        let uri = default.unwrap_or(&dialects[0].id);
        lookup
            .get(uri)
            .copied()
            .ok_or(DialectError::DefaultNotFound(uri.clone()))
    }

    #[must_use]
    pub fn as_borrowed(&'i self) -> Dialects<'i> {
        Dialects {
            dialects: Cow::Borrowed(self.dialects.as_ref()),
            lookup: Cow::Borrowed(self.lookup.as_ref()),
            default: self.default,
        }
    }
}

impl<'d> IntoIterator for &'d Dialects<'d> {
    type Item = &'d Dialect;

    type IntoIter = std::slice::Iter<'d, Dialect>;

    fn into_iter(self) -> Self::IntoIter {
        self.dialects.iter()
    }
}

fn find_impl_indexes<'a, F, T>(keywords: &'a [Box<dyn Keyword>], func: F) -> Box<[u16]>
where
    F: for<'b> Fn(&'a dyn Keyword, &'b Value) -> Result<T, Unimplemented>,
{
    let value = json!({});
    keywords
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (idx, keyword)| {
            if func(keyword.as_ref(), &value).is_ok() {
                #[allow(clippy::cast_possible_truncation)]
                acc.push(idx as u16);
            };
            acc
        })
        .into_boxed_slice()
}

fn find_is_pertinent_to_index(
    uri: &AbsoluteUri,
    keywords: &[Box<dyn Keyword>],
) -> Result<u16, DialectError> {
    find_impl_indexes(keywords, Keyword::is_pertinent_to)
        .first()
        .copied()
        .ok_or(DialectError::IsPertinentToNotImplemented(uri.clone()))
}

fn find_identify_indexes(
    uri: &AbsoluteUri,
    keywords: &[Box<dyn Keyword>],
) -> Result<Box<[u16]>, DialectError> {
    let indexes = find_impl_indexes(keywords, Keyword::identify);
    if indexes.is_empty() {
        return Err(DialectError::IdentifyNotImplemented(uri.clone()));
    }
    Ok(indexes)
}

fn find_dialect_indexes(
    uri: &AbsoluteUri,
    keywords: &[Box<dyn Keyword>],
) -> Result<Box<[u16]>, DialectError> {
    let indexes = find_impl_indexes(keywords, Keyword::dialect);
    if indexes.is_empty() {
        return Err(DialectError::DialectNotImplemented(uri.clone()));
    }
    Ok(indexes)
}

#[cfg(test)]
mod tests {}
