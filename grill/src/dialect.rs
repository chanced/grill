//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::{error::IdentifyError, uri::AbsoluteUri, Handler, Metaschema, Object, Uri};
use dyn_clone::{clone_trait_object, DynClone};
use itertools::Itertools;
use serde_json::Value;
use std::{borrow::Borrow, collections::HashMap, fmt::Debug, hash::Hash};

/// Defines a set of keywords and semantics that can be used to evaluate a
/// JSON Schema document.
#[derive(Clone, Debug)]
pub struct Vocabulary {
    /// The URI of the vocabulary.
    pub id: AbsoluteUri,
    /// Set of handlers for keywords defined by the vocabulary.
    pub handlers: Vec<Handler>,
}

impl Vocabulary {
    pub fn new<I, H>(id: &AbsoluteUri, handlers: I) -> Self
    where
        I: IntoIterator<Item = H>,
        H: Into<Handler>,
    {
        let id = id.clone();
        let handlers = handlers
            .into_iter()
            .map(|h| Into::<Handler>::into(h))
            .collect_vec();
        Self { id, handlers }
    }
}

pub trait Filter: Send + Sync + DynClone {
    fn matches(&self, value: &Value) -> bool;
}
clone_trait_object!(Filter);

impl<F> Filter for F
where
    F: Fn(&Value) -> bool + Send + Sync + Clone,
{
    fn matches(&self, value: &Value) -> bool {
        (self)(value)
    }
}
pub trait Identify: Send + Sync + DynClone {
    /// Identifies a schema
    /// # Errors
    /// Returns [`IdentifyError`] if `schema`:
    ///   * The identity fails to parse as a [`Uri`]
    ///   * The identity contains a fragment (e.g. `"https://example.com/example#fragment"`)
    fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError>;
}
clone_trait_object!(Identify);

impl<F> Identify for F
where
    F: Clone + Send + Sync + Fn(&Value) -> Result<Option<Uri>, IdentifyError>,
{
    fn identify(&self, value: &Value) -> Result<Option<Uri>, IdentifyError> {
        (self)(value)
    }
}

#[derive(Clone)]
pub struct Dialect {
    /// Identifier of the `Dialect`
    pub id: AbsoluteUri,
    /// Set of vocabularies defined by the dialect.
    pub vocabularies: HashMap<AbsoluteUri, Box<[Handler]>>,
    /// Set of meta schemas which make up the dialect.
    pub meta_schemas: HashMap<AbsoluteUri, Object>,
    /// Determines whether or not the `Dialect` is applicable to the given schema
    pub filter: Box<dyn 'static + Filter>,
    /// Identifies the schema if possible
    pub identify: Box<dyn 'static + Identify>,
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
impl Dialect {
    /// Creates a new [`Dialect`].
    pub fn new(
        id: impl Borrow<AbsoluteUri>,
        meta_schemas: &[Metaschema],
        vocabularies: &[Vocabulary],
        filter: impl 'static + Filter,
        identify: impl 'static + Identify,
    ) -> Self {
        let meta_schemas = meta_schemas
            .iter()
            .map(|m| (m.id.clone(), m.schema.clone()))
            .collect();
        let vocabularies = vocabularies
            .iter()
            .map(|v| (v.id.clone(), Box::from(v.handlers.clone())))
            .collect();

        Self {
            id: id.borrow().clone(),
            meta_schemas,
            vocabularies,
            filter: Box::new(filter),
            identify: Box::new(identify),
        }
    }
    pub fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        self.identify.identify(schema)
    }
    pub fn matches(&self, schema: &Value) -> bool {
        self.filter.matches(schema)
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::{dialect::Vocabulary, schema::Object};

//     use super::Dialect;

//     #[test]
//     fn test_new_empty_dialect() {
//         let dialect = Dialect::new("http://example.com/dialect".try_into().unwrap(), &[], &[]);
//         assert!(dialect.is_ok());
//     }
//     #[test]
//     fn test_new_dialect() {
//         let dialect = Dialect::new(
//             "http://example.com/dialect".try_into().unwrap(),
//             &[Object {
//                 id: Some("https://example/meta-schema".into()),
//                 vocabulary: [("https://example.com/vocab".to_string(), true)]
//                     .into_iter()
//                     .collect(),
//                 ..Default::default()
//             }],
//             &[Vocabulary {
//                 id: "https://example.com/vocab".into(),
//                 handlers: vec![],
//             }],
//         );
//         assert!(dialect.is_ok());
//         let dialect = dialect.unwrap();
//         assert_eq!(dialect.id, "http://example.com/dialect");
//         assert_eq!(dialect.vocabularies.len(), 1);
//         assert_eq!(dialect.meta_schemas.len(), 1);
//     }
//     #[test]
//     fn test_new_dialect_missing_schema_id() {
//         let dialect = Dialect::new(
//             "https://example.com/dialect".try_into().unwrap(),
//             &[Object {
//                 id: Some("https://example/meta-schema".try_into().unwrap()),
//                 vocabulary: [("https://example.com/vocab1".to_string(), true)]
//                     .into_iter()
//                     .collect(),
//                 ..Default::default()
//             }],
//             &[],
//         );
//         assert!(dialect.is_err());
//         let err = dialect.unwrap_err();

//         assert!(matches!(
//             err,
//             super::DialectError::MissingRequiredVocabulary { .. }
//         ));
//     }
// }
#[cfg(test)]
mod tests {}
