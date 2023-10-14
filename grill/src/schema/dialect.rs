//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::{
    error::{
        AnchorError, DialectError, DialectExistsError, DialectsError, IdentifyError, RefError,
    },
    keyword::{Keyword, Unimplemented},
    uri::AbsoluteUri,
    Src,
};
use ahash::{AHashMap, AHashSet};
use jsonptr::Pointer;
use serde_json::{json, Value};
use std::{borrow::Cow, convert::Into, fmt::Debug, hash::Hash, iter::IntoIterator, ops::Deref};

use super::{Anchor, Ref};

pub struct Builder {
    id: AbsoluteUri,
    metaschemas: Vec<(AbsoluteUri, Cow<'static, Value>)>,
    keywords: Vec<Box<dyn Keyword>>,
}

impl Builder {
    #[must_use]
    pub fn metaschema(mut self, id: AbsoluteUri, schema: Cow<'static, Value>) -> Self {
        self.metaschemas.push((id, schema));
        self
    }

    #[must_use]
    pub fn keyword(mut self, keyword: impl 'static + Keyword) -> Self {
        self.keywords.push(Box::new(keyword));
        self
    }
    pub fn build(self) -> Result<Dialect, DialectError> {
        Dialect::new(self.id, self.metaschemas, self.keywords)
    }
}

/// A set of keywords and semantics which are used to evaluate a [`Value`] against a
/// schema.
#[derive(Clone)]
pub struct Dialect {
    /// Identifier of the `Dialect`. A meta schema must be defined in
    /// `metaschemas` with this `id`.
    id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    metaschemas: AHashMap<AbsoluteUri, Cow<'static, Value>>,
    /// Set of [`Keyword`]s defined by the dialect.
    keywords: Box<[Box<dyn Keyword>]>,
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
        metaschemas: Vec<(AbsoluteUri, Cow<'static, Value>)>,
        keywords: Vec<Box<dyn Keyword>>,
    ) -> Result<Self, DialectError> {
        let metaschemas: AHashMap<AbsoluteUri, Cow<'static, Value>> =
            metaschemas.into_iter().collect();
        let keywords = keywords.into_boxed_slice();
        let identify_indexes = find_identify_indexes(&id, &keywords)?;
        let dialect_indexes = find_dialect_indexes(&id, &keywords)?;
        let subschemas_indexes = find_impl_indexes(&keywords, Keyword::subschemas);
        let anchors_indexes = find_impl_indexes(&keywords, Keyword::anchors);
        let references_indexes = find_impl_indexes(&keywords, Keyword::refs);
        if !metaschemas.contains_key(&id) {
            return Err(DialectError::DefaultNotFound(id));
        }
        Ok(Self {
            id,
            metaschemas,
            keywords,
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
    /// [`Keyword`]s index `2` and greater.
    pub fn identify(
        &self,
        mut base_uri: AbsoluteUri,
        path: &Pointer,
        schema: &Value,
    ) -> Result<(Option<AbsoluteUri>, Vec<AbsoluteUri>), IdentifyError> {
        let mut uris = Vec::new();
        if path.is_empty() {
            base_uri.set_fragment(None)?;
        } else {
            base_uri.set_fragment(Some(path))?;
        }
        uris.push(base_uri.clone());

        let mut primary = None;
        let mut secondary_uris = Vec::new();

        for idx in self.identify_indexes.iter().copied() {
            let Some(id) = self.keywords[idx as usize].identify(schema).unwrap()? else {
                continue;
            };
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
    pub fn subschemas(&self, path: &Pointer, src: &Value) -> AHashSet<Pointer> {
        self.subschemas_indexes
            .iter()
            .flat_map(|&idx| self.keywords[idx as usize].subschemas(src).unwrap())
            .map(|p| {
                let mut path = path.clone();
                path.append(&p);
                path
            })
            .collect()
    }

    pub fn refs(&self, source: &Value) -> Result<Vec<Ref>, RefError> {
        let mut refs = Vec::new();
        for res in self
            .references_indexes
            .iter()
            .copied()
            .map(|idx| self.keywords[idx as usize].refs(source).unwrap())
        {
            refs.append(&mut res?);
        }
        Ok(refs)
    }

    pub fn anchors(&self, source: &Value) -> Result<Vec<Anchor>, AnchorError> {
        let mut anchors = Vec::new();
        for res in self
            .anchors_indexes
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
        for idx in &*self.dialect_indexes {
            let idx = *idx as usize;
            let dialect = self.keywords[idx].dialect(schema).unwrap();
            if dialect.is_err() {
                continue;
            }
            let dialect = dialect.unwrap();
            if dialect.is_none() {
                continue;
            }
            let dialect = dialect.unwrap();
            if dialect == self.id {
                return true;
            }

            let both_have_similar_fragments = (self.id.is_fragment_empty_or_none()
                && dialect.is_fragment_empty_or_none())
                || self.id.fragment() == dialect.fragment();

            let is_http_or_https = |scheme: &str| scheme == "https" || scheme == "http";
            let has_http_or_https_scheme = |uri: &AbsoluteUri| is_http_or_https(uri.scheme());
            let both_are_http_or_https =
                has_http_or_https_scheme(&self.id) && has_http_or_https_scheme(&dialect);

            let both_are_urls = dialect.is_url() && self.id.is_url();
            let both_have_same_paths = dialect.path_or_nss() == self.id.path_or_nss();
            let both_have_same_queries = dialect.query() == self.id.query();
            let both_have_same_namespaces =
                dialect.authority_or_namespace() == self.id.authority_or_namespace();

            if both_are_urls
                && both_are_http_or_https
                && both_have_same_paths
                && both_have_same_queries
                && both_have_same_namespaces
                && both_have_similar_fragments
            {
                return true;
            }
        }
        false
    }

    #[must_use]
    /// Returns the [`AbsoluteUri`] of this `Dialect`.
    pub fn id(&self) -> &AbsoluteUri {
        &self.id
    }

    #[must_use]
    /// Returns the metaschemas of this `Dialect`.
    pub fn metaschemas(&self) -> &AHashMap<AbsoluteUri, Cow<'static, Value>> {
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
            .field("subschemas_indexes", &self.subschemas_indexes)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone)]
pub struct Dialects {
    dialects: Vec<Dialect>,
    default: usize,
}

impl Deref for Dialects {
    type Target = [Dialect];

    fn deref(&self) -> &Self::Target {
        &self.dialects
    }
}

impl Dialects {
    pub fn new(
        dialects: Vec<Dialect>,
        default: Option<AbsoluteUri>,
    ) -> Result<Self, DialectsError> {
        if dialects.is_empty() {
            return Err(DialectsError::Empty);
        }
        let mut collected = Vec::with_capacity(dialects.len());
        let mut lookup: AHashMap<AbsoluteUri, usize> = AHashMap::with_capacity(dialects.len());
        for (i, dialect) in dialects.into_iter().enumerate() {
            if dialect.id.fragment().is_some() && dialect.id.fragment() != Some("") {
                return Err(DialectError::FragmentedId(dialect.id.clone()).into());
            }
            if lookup.contains_key(&dialect.id) {
                return Err(DialectsError::Duplicate(DialectExistsError {
                    id: dialect.id.clone(),
                }));
            }
            let id = dialect.id.clone();
            collected.push(dialect);
            lookup.insert(id, i);
        }
        let default = Self::find_primary(&collected, &lookup, default.as_ref())?;
        Ok(Self {
            dialects: collected,
            default,
        })
    }

    /// Returns the [`Dialect`](crate::dialect::Dialect).
    #[must_use]
    pub fn get(&self, id: &AbsoluteUri) -> Option<&Dialect> {
        self.index_of(id).map(|idx| &self.dialects[idx])
    }

    fn index_of(&self, id: &AbsoluteUri) -> Option<usize> {
        self.dialects
            .iter()
            .enumerate()
            .find(|(_, d)| d.id == *id)
            .map(|(idx, _)| idx)
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

    #[must_use]
    pub fn pertinent_to_idx(&self, schema: &Value) -> Option<usize> {
        self.dialects
            .iter()
            .enumerate()
            .find(|(_, dialect)| dialect.is_pertinent_to(schema))
            .map(|(idx, _)| idx)
    }

    /// Appends a [`Dialect`].
    ///
    /// # Errors
    /// Returns the [`DialectExists`] if a `Dialect` already exists with the same `id`.
    pub fn push(&mut self, dialect: Dialect) -> Result<(), DialectExistsError> {
        if self.contains(&dialect.id) {
            return Err(DialectExistsError {
                id: dialect.id.clone(),
            });
        }
        self.dialects.push(dialect);
        Ok(())
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
        for dialect in &*self.dialects {
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
    pub fn iter(&self) -> std::slice::Iter<'_, Dialect> {
        self.dialects.iter()
    }

    /// Returns the primary [`Dialect`](`crate::dialect::Dialect`).
    #[must_use]
    pub fn primary(&self) -> &Dialect {
        &self.dialects[self.default]
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
        self.dialects.iter().position(|d| d == dialect)
    }

    #[must_use]
    pub fn get_by_index(&self, idx: usize) -> Option<&Dialect> {
        self.dialects.get(idx)
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
        dialects: &[Dialect],
        lookup: &AHashMap<AbsoluteUri, usize>,
        default: Option<&AbsoluteUri>,
    ) -> Result<usize, DialectError> {
        let uri = default.unwrap_or(&dialects[0].id);
        lookup
            .get(uri)
            .copied()
            .ok_or(DialectError::DefaultNotFound(uri.clone()))
    }
}

impl<'a> IntoIterator for &'a Dialects {
    type Item = &'a Dialect;

    type IntoIter = std::slice::Iter<'a, Dialect>;

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
mod tests {
    use std::borrow::Cow;

    use serde_json::json;

    use crate::json_schema;

    use super::Dialect;

    #[test]
    fn test_is_pertinent_to() {
        let valid = [
            "http://json-schema.org/draft-04/schema#",
            "http://json-schema.org/draft-04/schema",
            "https://json-schema.org/draft-04/schema#",
            "https://JSON-schema.org/draft-04/schema#",
        ];
        let invalid = [
            "http://json-schema.org/draft-04/schema#fragmented",
            "http://json-schema.org/draft-04",
        ];
        let id = crate::json_schema::draft_04::json_schema_04_uri();
        let dialect = Dialect::builder(id.clone())
            .metaschema(id.clone(), Cow::Owned(json!({})))
            .keyword(json_schema::common::schema::Keyword::new(
                json_schema::SCHEMA,
                true,
            ))
            .keyword(json_schema::common::id::Keyword::new(json_schema::ID, true))
            .build()
            .unwrap();

        for valid in valid {
            let schema = json!({ "$schema": valid });
            assert!(dialect.is_pertinent_to(&schema));
        }
        for invalid in invalid {
            let schema = json!({ "$schema": invalid });
            assert!(!dialect.is_pertinent_to(&schema));
        }
    }
}
