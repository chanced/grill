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
    error::{IdentifyError, LocateSchemasError},
    keyword::{Keyword, SchemaKeyword},
    uri::AbsoluteUri,
    Handler, Metaschema, Object, Uri,
};
use serde_json::Value;
use std::{
    borrow::Borrow, collections::HashMap, convert::Into, fmt::Debug, hash::Hash, iter::IntoIterator,
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
    /// Determines whether or not the `Dialect` is applicable to the given schema
    pub is_schema: Box<dyn 'static + IsSchema>,
    /// Identifies the schema if possible
    pub identify_schema: Box<dyn 'static + IdentifySchema>,
    /// Collects [`Anchor`]s from a [`Value`]
    pub locate_schemas: Box<dyn 'static + LocateSchemas>,
    /// Keywords in the `Dialect` which may contain one or more schemas, be that
    /// as a direct value, a value in an array, or a property of an object.
    pub schema_keywords: HashMap<Keyword<'static>, SchemaKeyword<'static>>,
}

impl Dialect {
    /// Creates a new [`Dialect`].
    pub fn new<M, S, I, H>(
        id: impl Borrow<AbsoluteUri>,
        meta_schemas: M,
        handlers: H,
        is_schema: impl 'static + IsSchema,
        identify_schema: impl 'static + IdentifySchema,
        locate_schemas: impl 'static + LocateSchemas,
    ) -> Self
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
        let is_schema = Box::new(is_schema);
        let identify_schema = Box::new(identify_schema);
        let locate_schemas = Box::new(locate_schemas);
        let schema_keywords = handlers
            .iter()
            .filter_map(crate::handler::Handler::schema_keywords)
            .flat_map(IntoIterator::into_iter)
            .map(|kw| (kw.keyword(), *kw))
            .collect();

        Self {
            id,
            metaschemas,
            handlers,
            is_schema,
            identify_schema,
            locate_schemas,
            schema_keywords,
        }
    }

    pub(crate) fn parts(&self) -> Parts {
        Parts {
            id: self.id.clone(),
            handlers: self.handlers.clone(),
            is_schema: self.is_schema.clone(),
            identify_schema: self.identify_schema.clone(),
            locate_schemas: self.locate_schemas.clone(),
        }
    }

    pub fn identify(&self, schema: &Value) -> Result<Option<Uri>, IdentifyError> {
        self.identify_schema.identify_schema(schema)
    }

    pub fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        self.locate_schemas
            .locate_schemas(path, value, dialects, base_uri)
    }

    #[must_use]
    pub fn can_keyword_contain_schemas(&self, keyword: Keyword) -> bool {
        self.schema_keywords.contains_key(&keyword)
    }

    /// Determines whether an element in an array or a property on an object, at
    /// `path`, can contain one or more schemas. `value` is the containing array
    /// or object one level above the leaf of `path`.
    #[must_use]
    pub fn is_schema_property(&self, path: &Pointer, value: &Value) -> bool {
        let mut iter = path.into_iter().peekable();
        while let Some(tok) = iter.next() {
            let Some(prop) = self.schema_keywords.get(&Keyword(tok.decoded())) else { return false };
            match prop {
                SchemaKeyword::Array(_) => {
                    let Some(next) = iter.next() else { return false };
                    if !value.is_array() {
                        return false;
                    }
                    if next.parse::<usize>().is_err() {
                        return false;
                    }
                }
                SchemaKeyword::Map(_) => {
                    let next = iter.next();
                    if next.is_none() {
                        return false;
                    };
                }
                SchemaKeyword::SingleOrArray(_) => {
                    if let Some(next) = iter.peek() {
                        if self.schema_keywords.contains_key(&Keyword(next)) {
                            continue;
                        }
                        let next = iter.next().unwrap();
                        if next.parse::<usize>().is_err() {
                            return false;
                        }
                    }
                    return true;
                }
                SchemaKeyword::Single(_) => continue,
            }
        }
        true
    }

    #[must_use]
    pub fn matches(&self, schema: &Value) -> bool {
        self.is_schema.is_schema(schema)
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
#[derive(Debug, Clone, Copy)]
pub struct Dialects<'d> {
    dialects: &'d [Dialect],
    default_dialect: usize,
}

impl<'d> Dialects<'d> {
    #[must_use]
    pub fn new(dialects: &'d [Dialect], default_dialect: &'d Dialect) -> Self {
        let mut this = Self {
            dialects,
            default_dialect: 0,
        };
        this.set_default_dialect_index(this.position(default_dialect).unwrap());
        this
    }
    /// Sets the default [`Dialect`] to use when no other [`Dialect`] matches.
    ///
    /// # Panics
    /// Panics if the default dialect is not in the list of dialects.
    pub fn set_default_dialect_index(&mut self, dialect: usize) {
        assert!(dialect < self.dialects.len());
        self.default_dialect = dialect;
    }
    #[must_use]
    pub fn default_dialect(&self) -> &'d Dialect {
        &self.dialects[self.default_dialect]
    }
    /// Returns the index of the given [`Dialect`] in the list of [`Dialect`]s.
    #[must_use]
    pub fn position(&self, dialect: &Dialect) -> Option<usize> {
        self.dialects.iter().position(|d| d == dialect)
    }
    #[must_use]
    pub fn get(&self, idx: usize) -> Option<&'d Dialect> {
        self.dialects.get(idx)
    }
    #[must_use]
    pub fn dialect_index_for(&self, schema: &Value) -> usize {
        let default = self.default_dialect();
        if default.matches(schema) {
            return self.default_dialect;
        }
        for (idx, dialect) in self.dialects.iter().enumerate() {
            if dialect.id != default.id && dialect.matches(schema) {
                return idx;
            }
        }
        self.default_dialect
    }
}

pub(crate) struct Parts {
    pub id: AbsoluteUri,
    pub handlers: Vec<Handler>,
    pub is_schema: Box<dyn 'static + IsSchema>,
    pub identify_schema: Box<dyn 'static + IdentifySchema>,
    pub locate_schemas: Box<dyn 'static + LocateSchemas>,
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
mod tests {
    use serde_json::json;

    use crate::json_schema::draft_2019_09::JSON_SCHEMA_2019_09_DIALECT;

    #[test]
    fn test_is_is_schema_property() {
        let d = JSON_SCHEMA_2019_09_DIALECT.clone();
        let ptr = |s: &str| s.try_into().unwrap();
        assert!(d.is_schema_property(&ptr("/properties/prop/items/0/$defs/nested"), &json!({})));
        assert!(d.is_schema_property(&ptr("/properties/prop/items/$defs/nested"), &json!({})));
        assert!(d.is_schema_property(&ptr("/anyOf/3/if/$defs/nested/prefixItems/0"), &json!([{}])));
        assert!(!d.is_schema_property(
            &ptr("/anyOf/invalid/if/$defs/nested/prefixItems/34"),
            &json!({})
        ));
        assert!(!d.is_schema_property(&ptr("/invalid/if/$defs/nested/prefixItems/21"), &json!({})));
        assert!(!d.is_schema_property(&ptr("/invalid/if/$defs/nested///"), &json!({})));
    }
}
