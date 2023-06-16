//! Keywords and semantics that can be used to evaluate a value against a
//! schema.

use crate::{error::IdentifyError, uri::AbsoluteUri, Anchor, Handler, Metaschema, Object, Uri};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;
use std::{borrow::Borrow, collections::HashMap, convert::Into, fmt::Debug, hash::Hash};

#[derive(Clone)]
pub struct Dialect {
    /// Identifier of the `Dialect`
    pub id: AbsoluteUri,
    /// Set of meta schemas which make up the dialect.
    pub meta_schemas: HashMap<AbsoluteUri, Object>,
    /// Set of [`Handler`]s defined by the dialect.
    pub handlers: Vec<Handler>,
    /// Determines whether or not the `Dialect` is applicable to the given schema
    pub filter: Box<dyn 'static + Match>,
    /// Identifies the schema if possible
    pub identify: Box<dyn 'static + Identify>,
    /// Collects [`Anchor`]s from a [`Value`]
    pub anchors: Box<dyn 'static + Anchors>,
}

impl Dialect {
    /// Creates a new [`Dialect`].
    pub fn new<M, S, I, H>(
        id: impl Borrow<AbsoluteUri>,
        meta_schemas: M,
        handlers: H,
        filter: impl 'static + Match,
        identify: impl 'static + Identify,
        anchors: impl 'static + Anchors,
    ) -> Self
    where
        S: Borrow<Metaschema>,
        M: IntoIterator<Item = S>,
        I: Into<Handler>,
        H: IntoIterator<Item = I>,
    {
        let meta_schemas = meta_schemas
            .into_iter()
            .map(|m| {
                let m = m.borrow();
                (m.id.clone(), m.schema.clone())
            })
            .collect();
        let handlers = handlers.into_iter().map(Into::into).collect();
        let id = id.borrow().clone();
        let filter = Box::new(filter);
        let identify = Box::new(identify);
        let anchors = Box::new(anchors);
        Self {
            id,
            meta_schemas,
            handlers,
            filter,
            identify,
            anchors,
        }
    }
    pub(crate) fn parts(&self) -> Parts {
        Parts {
            id: self.id.clone(),
            handlers: self.handlers.clone(),
            filter: self.filter.clone(),
            identify: self.identify.clone(),
            anchors: self.anchors.clone(),
        }
    }
    pub fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        self.identify.identify(schema)
    }

    #[must_use]
    pub fn matches(&self, schema: &Value) -> bool {
        self.filter.matches(schema)
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
            .field("meta_schemas", &self.meta_schemas)
            .field("handlers", &self.handlers)
            .finish_non_exhaustive()
    }
}

pub trait Anchors: Send + Sync + DynClone {
    fn anchors<'v>(&self, path: Pointer, value: &'v Value) -> Vec<Anchor<'v>>;
}
clone_trait_object!(Anchors);

impl<F> Anchors for F
where
    F: Send + Sync + Clone + Fn(Pointer, &Value) -> Vec<Anchor>,
{
    fn anchors<'v>(&self, path: Pointer, value: &'v Value) -> Vec<Anchor<'v>> {
        (self)(path, value)
    }
}

pub trait Match: Send + Sync + DynClone {
    fn matches(&self, value: &Value) -> bool;
}
clone_trait_object!(Match);

impl<F> Match for F
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

pub(crate) struct Parts {
    pub id: AbsoluteUri,
    pub handlers: Vec<Handler>,
    pub filter: Box<dyn 'static + Match>,
    pub identify: Box<dyn 'static + Identify>,
    pub anchors: Box<dyn 'static + Anchors>,
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
