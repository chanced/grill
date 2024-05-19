//! A container of [`Keyword`]s and semantics which determine how to evaluate a
//! [`Value`] against a [`Schema`](crate::Schema).

use super::Anchor;
use crate::{
    error::{
        new_dialect_error::{
            DefaultNotFoundSnafu, DialectNotImplementedSnafu, FragmentedIdSnafu,
            IdentifyNotImplementedSnafu,
        },
        new_dialects_error::DuplicateDialectSnafu,
        AnchorError, IdentifyError, NewDialectError, NewDialectsError, RefsError,
    },
    language::{Keyword, Language, Ref},
    source::Input,
    ControlFlowExt, EMPTY_OBJ,
};
use grill_uri::{AbsoluteUri, TryIntoAbsoluteUri};
use jsonptr::Pointer;
use serde_json::Value;
use slotmap::Key;
use snafu::{ensure, OptionExt};
use std::{
    borrow::Cow,
    convert::Into,
    fmt::Debug,
    hash::Hash,
    iter::IntoIterator,
    ops::{ControlFlow, Deref},
};
use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

/// Builds a [`Dialect`].
pub struct Build<L: Language<K>, K: 'static + Key> {
    id: AbsoluteUri,
    metaschemas: Vec<(Result<AbsoluteUri, grill_uri::Error>, Cow<'static, Value>)>,
    keywords: Vec<L::Keyword>,
    marker: PhantomData<fn(K) -> !>,
}

impl<L, K> Build<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Adds a metaschema to the [`Dialect`]. These are used to validate the
    /// schemas of the [`Dialect`], as determined by [`Dialect::is_pertinent_to`].
    #[must_use]
    pub fn add_metaschema(
        mut self,
        id: impl TryIntoAbsoluteUri,
        schema: Cow<'static, Value>,
    ) -> Self {
        self.metaschemas.push((id.try_into_absolute_uri(), schema));
        self
    }

    /// Adds a [`Keyword`] to the [`Dialect`].
    #[must_use]
    pub fn add_keyword(mut self, keyword: L::Keyword) -> Self {
        self.keywords.push(keyword);
        self
    }

    /// Finalizes the [`Dialect`].
    pub fn finish(self) -> Result<Dialect<L, K>, NewDialectError> {
        let metaschemas: Vec<(AbsoluteUri, Cow<'static, Value>)> = self
            .metaschemas
            .into_iter()
            .map(|(id, schema)| {
                let id = id?;
                Ok((id.clone(), schema))
            })
            .collect::<Result<_, grill_uri::Error>>()?;
        Dialect::new(self.id, metaschemas, self.keywords)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Dialect                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A set of keywords and semantics which are used to evaluate a [`Value`] against a
/// schema.
#[derive(Clone)]
pub struct Dialect<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Identifier of the `Dialect`. A meta schema must be defined in
    /// `metaschemas` with this `id`.
    id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    metaschemas: HashMap<AbsoluteUri, Cow<'static, Value>>,
    /// Set of [`Keyword`]s defined by the dialect.
    keywords: Box<[L::Keyword]>,
    /// indexes of keywords which implement the `identify` method.
    identify_indexes: Box<[u16]>,
    /// indexes of keywords which implement the `dialect` method.
    dialect_indexes: Box<[u16]>,
    /// indexes of keywords which implement the `subschemas` method.
    subschemas_indexes: Box<[u16]>,
    /// indexes of keywords which implement the `anchors` method.
    anchors_indexes: Box<[u16]>,
    /// indexes of keywords which implement the `refs` method.
    references_indexes: Box<[u16]>,

    pub(crate) schema_key: K,
}
impl<L, K> std::fmt::Display for Dialect<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id, f)
    }
}

impl<L, K> Dialect<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Returns a new `Dialect` [`Build`].
    #[must_use]
    pub fn build(id: AbsoluteUri) -> Build<L, K> {
        Build {
            id,
            metaschemas: Vec::new(),
            keywords: Vec::new(),
            marker: PhantomData,
        }
    }

    /// Creates a new [`Dialect`].
    fn new(
        id: AbsoluteUri,
        metaschemas: Vec<(AbsoluteUri, Cow<'static, Value>)>,
        keywords: Vec<L::Keyword>,
    ) -> Result<Self, NewDialectError> {
        let metaschemas: HashMap<AbsoluteUri, Cow<'static, Value>> =
            metaschemas.into_iter().collect();
        let keywords = keywords.into_boxed_slice();
        let identify_indexes = Self::find_identify_indexes(&id, &keywords)?;
        let dialect_indexes = Self::find_dialect_indexes(&id, &keywords)?;
        let subschemas_indexes = Self::find_impl_indexes(&keywords, Keyword::dialect);
        let anchors_indexes = Self::find_impl_indexes(&keywords, Keyword::anchors);
        let references_indexes = Self::find_impl_indexes(&keywords, Keyword::refs);

        ensure!(
            metaschemas.contains_key(&id),
            DefaultNotFoundSnafu { uri: id }
        );
        Ok(Self {
            id,
            metaschemas,
            keywords,
            identify_indexes,
            dialect_indexes,
            subschemas_indexes,
            anchors_indexes,
            references_indexes,
            schema_key: K::default(),
        })
    }

    /// Attempts to identify a `schema` based on the [`Keyword`]s associated
    /// with this `Dialect`. It returns the primary id (if any) and all
    /// additional, secondary [`AbsoluteUri`]s the
    /// [`Schema`](`crate::schema::Schema`) can b referenced by.
    pub fn identify(
        &self,
        uri: AbsoluteUri,
        schema: &Value,
    ) -> Result<(Option<AbsoluteUri>, Vec<AbsoluteUri>), IdentifyError> {
        let mut uris = vec![uri.clone()];
        let mut primary = None;

        for idx in self.identify_indexes.iter().copied() {
            let Some(id) = self.keywords[idx as usize]
                .identify(schema)
                .unwrap_continue()?
            else {
                continue;
            };
            let uri = uri.resolve(&id)?;
            primary.get_or_insert(uri.clone());
            uris.push(uri);
        }
        Ok((primary, uris))
    }

    /// Returns the [`AbsoluteUri`] of the primary metaschema.
    #[must_use]
    pub fn primary_metaschema_id(&self) -> &AbsoluteUri {
        &self.id
    }

    /// Attempts to locate nested schemas within `source` by calling
    /// [`Keyword::subschemas`] for each attached
    /// `Keyword` of this `Dialect`.
    ///
    #[must_use]
    pub fn subschemas(&self, src: &Value) -> HashSet<Pointer> {
        self.subschemas_indexes
            .iter()
            .flat_map(|&idx| {
                self.keywords[idx as usize]
                    .subschemas(src)
                    .unwrap_continue()
            })
            .collect()
    }

    /// Attempts to locate [`Ref`]s of a schema, composed of results from associated [`Keyword`]'s
    /// [`Keyword::refs`] method.
    ///
    /// # Errors
    /// Returns [`RefError`] if any [`Keyword`] fails to parse the [`Ref`]s. This could include
    /// invalid JSON types or malformed URIs.
    pub fn refs(&self, source: &Value) -> Result<Vec<Ref>, RefsError> {
        let mut refs = Vec::new();
        for res in self
            .references_indexes
            .iter()
            .copied()
            .map(|idx| self.keywords[idx as usize].refs(source).unwrap_continue())
        {
            refs.append(&mut res?);
        }
        Ok(refs)
    }

    /// Attempts to locate [`Anchor`]s of a schema, composed of results from associated [`Keyword`]'s
    /// [`Keyword::anchors`] method.
    ///
    /// # Errors
    /// Returns [`AnchorError`] if any [`Keyword`] fails to parse the [`Anchor`]s. This could include
    /// invalid JSON types or malformed anchors.
    pub fn anchors(&self, source: &Value) -> Result<Vec<Anchor>, AnchorError> {
        let mut anchors: Vec<Anchor> = Vec::new();
        let mut names = HashSet::new();
        for res in self.anchors_indexes.iter().copied().map(|idx| {
            self.keywords[idx as usize]
                .anchors(source)
                .unwrap_continue()
        }) {
            for anchor in res? {
                if names.contains(&anchor.name) {
                    let existing = anchors.iter().find(|a| a.name == anchor.name).unwrap();
                    return Err(crate::error::DuplicateAnchorError {
                        existing: existing.clone(),
                        duplicate: anchor,
                        backtrace: snafu::Backtrace::capture(),
                    }
                    .into());
                }
                names.insert(anchor.name.clone());
                anchors.push(anchor);
            }
        }
        Ok(anchors)
    }

    /// Determines if the schema is pertinent to this `Dialect`.
    #[must_use]
    pub fn is_pertinent_to(&self, schema: &Value) -> bool {
        for idx in &*self.dialect_indexes {
            let idx = *idx as usize;
            let dialect = self.keywords[idx].dialect(schema).unwrap_continue();
            let Ok(Some(dialect)) = dialect else { continue };
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
    pub fn metaschemas(&self) -> &HashMap<AbsoluteUri, Cow<'static, Value>> {
        &self.metaschemas
    }

    #[must_use]
    /// Returns the [`Keyword`]s of this `Dialect`.
    pub fn keywords(&self) -> &[L::Keyword] {
        self.keywords.as_ref()
    }

    fn find_impl_indexes<'a, F, B, R>(keywords: &'a [L::Keyword], func: F) -> Box<[u16]>
    where
        F: for<'b> Fn(&'a L::Keyword, &'b Value) -> ControlFlow<B, R>,
    {
        keywords
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut acc, (idx, keyword)| {
                if func(keyword, &EMPTY_OBJ).is_continue() {
                    #[allow(clippy::cast_possible_truncation)]
                    acc.push(idx as u16);
                };
                acc
            })
            .into_boxed_slice()
    }
    fn find_identify_indexes(
        uri: &AbsoluteUri,
        keywords: &[L::Keyword],
    ) -> Result<Box<[u16]>, NewDialectError> {
        let indexes = Self::find_impl_indexes(keywords, Keyword::identify);
        ensure!(
            !indexes.is_empty(),
            IdentifyNotImplementedSnafu { uri: uri.clone() }
        );
        Ok(indexes)
    }

    fn find_dialect_indexes(
        uri: &AbsoluteUri,
        keywords: &[L::Keyword],
    ) -> Result<Box<[u16]>, NewDialectError> {
        let indexes = Self::find_impl_indexes(keywords, Keyword::dialect);
        ensure!(
            !indexes.is_empty(),
            DialectNotImplementedSnafu { uri: uri.clone() }
        );
        Ok(indexes)
    }
}

impl<L, K> PartialEq for Dialect<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<L, K> Hash for Dialect<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<L, K> Debug for Dialect<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Dialects                               ║
║                               ¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A collection of [`Dialect`]s.
#[derive(Debug, Clone)]
pub struct Dialects<L: Language<K>, K: 'static + Key> {
    dialects: Vec<Dialect<L, K>>,
    default: usize,
}

impl<L, K> Deref for Dialects<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    type Target = [Dialect<L, K>];

    fn deref(&self) -> &Self::Target {
        &self.dialects
    }
}

impl<L, K> Dialects<L, K>
where
    L: Language<K>,
    K: 'static + Key,
{
    /// Creates a new [`Dialects`] from a [`Vec`] of [`Dialect`]s.
    ///
    /// If `default` is `None`, the first [`Dialect`] in the list is used as the
    /// default.
    pub fn new(
        dialects: Vec<Dialect<L, K>>,
        default: Option<AbsoluteUri>,
    ) -> Result<Self, NewDialectsError> {
        ensure!(
            !dialects.is_empty(),
            crate::error::new_dialects_error::EmptySnafu
        );

        let mut collected = Vec::with_capacity(dialects.len());
        let mut lookup: HashMap<AbsoluteUri, usize> = HashMap::with_capacity(dialects.len());
        for (i, dialect) in dialects.into_iter().enumerate() {
            ensure!(
                dialect.id.fragment().unwrap_or_default().trim().is_empty(),
                FragmentedIdSnafu {
                    uri: dialect.id.clone()
                }
            );
            ensure!(
                !lookup.contains_key(&dialect.id),
                DuplicateDialectSnafu {
                    uri: dialect.id.clone()
                }
            );
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

    /// Returns the [`Dialect`].
    #[must_use]
    pub fn get(&self, id: &AbsoluteUri) -> Option<&Dialect<L, K>> {
        self.index_of(id).map(|idx| &self.dialects[idx])
    }

    fn index_of(&self, id: &AbsoluteUri) -> Option<usize> {
        self.dialects
            .iter()
            .enumerate()
            .find(|(_, d)| d.id == *id)
            .map(|(idx, _)| idx)
    }

    /// Attempts to determine if the `schema` [`Value`] is pertinent to a
    /// [`Dialect`] based upon the [`Keyword`]s of this [`Dialect`].
    ///
    /// ## Special handling for Dialect IDs that are URLs
    /// Logic is in place to handle cases where the URI `$id` of the
    /// schema is in the form of a URL which matches on both the `"http"` and
    /// `"https"` schemes as well as with an empty or non-existent fragment. For
    /// example, a `Dialect` with the `$id` `"https://example.com"` would consider
    /// a schema with a `$schema` of `"http://example.com#"` to be pertinent.
    #[must_use]
    pub fn pertinent_to(&self, schema: &Value) -> Option<&Dialect<L, K>> {
        self.dialects
            .iter()
            .find(|&dialect| dialect.is_pertinent_to(schema))
    }

    /// Returns the index of the [`Dialect`] that is pertinent to the schema.
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
    /// Returns the [`DialectExistsError`] if a `Dialect` already exists with the same `id`.
    pub fn push(&mut self, dialect: Dialect<L, K>) -> Result<(), AbsoluteUri> {
        if self.contains(&dialect.id) {
            return Err(dialect.id.clone());
        }
        self.dialects.push(dialect);
        Ok(())
    }

    /// Returns `true` if the [`Dialects`] contains a [`Dialect`] with the given
    /// [`AbsoluteUri`](`crate::uri::AbsoluteUri`) as an ID.
    #[must_use]
    pub fn contains(&self, id: &AbsoluteUri) -> bool {
        self.dialects.iter().any(|d| &d.id == id)
    }

    /// Returns the [`Dialect`] that is pertinent to the schema or the default
    /// [`Dialect`] if the [`Dialect`] can not be determined from schema.
    #[must_use]
    pub fn pertinent_to_or_default(&self, schema: &Value) -> &Dialect<L, K> {
        self.pertinent_to(schema).unwrap_or(self.primary())
    }

    /// Returns the index of the [`Dialect`] that is pertinent to the schema or
    /// the index of the default [`Dialect`] if the [`Dialect`] can not be
    /// determined from schema.
    #[must_use]
    pub fn pertinent_to_or_default_idx(&self, schema: &Value) -> usize {
        self.pertinent_to(schema)
            .map_or(self.default, |d| self.position(d).unwrap())
    }

    /// Returns an [`Iterator`] of [`&AbsoluteUri`](`crate::uri::AbsoluteUri`) for each metaschema in each [`Dialect`].
    pub fn source_ids(&self) -> impl Iterator<Item = &AbsoluteUri> {
        self.dialects.iter().flat_map(|d| d.metaschemas.keys())
    }

    #[must_use]
    pub(crate) fn sources(&self) -> Vec<Input> {
        let mut result = Vec::with_capacity(self.dialects.len());
        for dialect in &*self.dialects {
            for metaschema in &dialect.metaschemas {
                result.push(Input::Value(metaschema.0.clone(), metaschema.1.clone()));
            }
        }
        result
    }

    /// Returns a slice of [`Dialect`].
    #[must_use]
    pub fn as_slice(&self) -> &[Dialect<L, K>] {
        &self.dialects
    }

    /// Returns the index (`usize`) of the default.
    /// [`Dialect`]
    #[must_use]
    pub fn default_index(&self) -> usize {
        self.default
    }
    /// Returns an [`Iterator`] over the [`Dialect`]s.
    pub fn iter(&self) -> std::slice::Iter<'_, Dialect<L, K>> {
        self.dialects.iter()
    }

    /// Returns the primary [`Dialect`].
    #[must_use]
    pub fn primary(&self) -> &Dialect<L, K> {
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
    pub fn position(&self, dialect: &Dialect<L, K>) -> Option<usize> {
        self.dialects.iter().position(|d| d == dialect)
    }

    /// Returns the [`Dialect`] at the given index.
    #[must_use]
    pub fn get_by_index(&self, idx: usize) -> Option<&Dialect<L, K>> {
        self.dialects.get(idx)
    }

    fn find_primary(
        dialects: &[Dialect<L, K>],
        lookup: &HashMap<AbsoluteUri, usize>,
        default: Option<&AbsoluteUri>,
    ) -> Result<usize, NewDialectError> {
        let uri = default.unwrap_or(&dialects[0].id);
        lookup
            .get(uri)
            .copied()
            .with_context(|| DefaultNotFoundSnafu { uri: uri.clone() })
    }

    pub(crate) fn set_keys(&mut self, keys: Vec<(AbsoluteUri, K)>) {
        for (uri, key) in keys {
            self.dialects
                .iter_mut()
                .find(|d| d.id == uri)
                .unwrap()
                .schema_key = key;
        }
    }
}

impl<'a, L: Language<K>, K: 'static + Key> IntoIterator for &'a Dialects<L, K> {
    type Item = &'a Dialect<L, K>;

    type IntoIter = std::slice::Iter<'a, Dialect<L, K>>;

    fn into_iter(self) -> Self::IntoIter {
        self.dialects.iter()
    }
}

#[cfg(test)]
mod tests {

    use grill_uri::AbsoluteUri;

    #[test]
    fn test_is_pertinent_to() {
        let _valid = [
            "http://json-schema.org/draft-04/schema#",
            "http://json-schema.org/draft-04/schema",
            "https://json-schema.org/draft-04/schema#",
            "https://JSON-schema.org/draft-04/schema#",
        ];
        let _invalid = [
            "http://json-schema.org/draft-04/schema#fragmented",
            "http://json-schema.org/draft-04",
        ];
        let _id: AbsoluteUri = "http://json-schema.org/draft-04/schema#".parse().unwrap();
        // let dialect = Dialect::build(id.clone())
        //     .add_metaschema(id.clone(), Cow::Owned(json!({})))
        //     .add_keyword(test::keyword::schema::SchemaKeyword::new("$schema", true))
        //     .add_keyword(test::keyword::id::Id::new("$id", true))
        //     .finish()
        //     .unwrap();

        // for valid in valid {
        //     let schema = json!({ "$schema": valid });
        //     assert!(dialect.is_pertinent_to(&schema));
        // }
        // for invalid in invalid {
        //     let schema = json!({ "$schema": invalid });
        //     assert!(!dialect.is_pertinent_to(&schema));
        // }
    }
}
