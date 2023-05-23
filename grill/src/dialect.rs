use std::collections::HashMap;

use crate::{error::DialectError, schema::Object, Handler};

pub mod json_schema_07;

/// Defines a set of keywords and semantics that can be used to evaluate a
/// JSON Schema document.
#[derive(Clone, Debug)]
pub struct Vocabulary {
    /// The URI of the vocabulary.
    pub id: String,
    /// Set of handlers for keywords defined by the vocabulary.
    pub handlers: Vec<Box<Handler>>,
}

/// A composition of [`Vocabulary`].
#[derive(Debug, Clone)]
pub struct Dialect {
    /// The URI of the dialect, used as [`"$schema"`](`crate::schema::Object::schema`).
    pub id: String,

    /// Set of vocabularies defined by the dialect.
    pub vocabularies: HashMap<String, Vocabulary>,

    /// Set of meta schemas which make up the dialect.
    pub meta_schemas: HashMap<String, Object>,
}

fn collect_meta_schemas(meta_schemas: &[Object]) -> Result<HashMap<String, Object>, DialectError> {
    meta_schemas
        .iter()
        .map(|s| {
            if s.id.is_none() {
                Err(DialectError::MissingSchemaId { schema: s.clone() })
            } else {
                let id = s.id.clone().unwrap();
                Ok((id, s.clone()))
            }
        })
        .collect()
}

impl Dialect {
    /// Creates a new [`Dialect`].
    ///
    /// # Errors
    /// Returns a [`NewDialectError`] if:
    /// - A schema [`Object`] in `meta_schemas` does not have an [`id`](crate::schema::Object::id).
    /// - A schema [`Object`] in `meta_schemas` has a non-absolute [`id`](crate::schema::Object::id).
    /// - A schema [`Object`] in `meta_schemas` has a required vocabulary that is not defined in `vocabularies`.
    pub fn new(
        id: impl ToString,
        meta_schemas: &[Object],
        vocabularies: &[Vocabulary],
    ) -> Result<Self, DialectError> {
        let vocabularies = vocabularies
            .iter()
            .map(|v| (v.id.clone(), (v.clone())))
            .collect();
        let meta_schemas = collect_meta_schemas(meta_schemas)?;
        confirm_required_vocabulary(meta_schemas.iter(), &vocabularies)?;
        Ok(Self {
            id: id.to_string(),
            vocabularies,
            meta_schemas,
        })
    }

    /// Inserts a [`Vocabulary`] to the [`Dialect`], returning the previous value if it exists.
    pub fn insert_vocabulary(&mut self, vocabulary: Vocabulary) -> Option<Vocabulary> {
        self.vocabularies.insert(vocabulary.id.clone(), vocabulary)
    }

    /// Attempts to insert a [`Schema`] to the [`Dialect`].
    ///
    /// # Errors
    /// Returns a [`DialectError`] if:
    /// - The [`Schema`] does not have an [`id`](crate::schema::Object::id).
    /// - The [`Schema`] has a non-absolute [`id`](crate::schema::Object::id).
    /// - The [`Schema`] has a required vocabulary that is not defined in the [`Dialect`].
    pub fn insert_meta_schema(&mut self, schema: Object) -> Result<Option<Object>, DialectError> {
        if let Some(id) = schema.id.as_ref() {
            // let id: AbsoluteUri =
            //     id.clone()
            //         .try_into()
            //         .map_err(|err| DialectError::SchemaIdNotAbsolute {
            //             err,
            //             id: id.clone(),
            //         })?;
            confirm_required_vocabulary(std::iter::once((id, &schema)), &self.vocabularies)?;
            Ok(self.meta_schemas.insert(id.to_string(), schema))
        } else {
            Err(DialectError::MissingSchemaId { schema })
        }
    }
}

fn confirm_required_vocabulary<'a>(
    meta_schemas: impl Iterator<Item = (&'a String, &'a Object)>,
    vocabularies: &HashMap<String, Vocabulary>,
) -> Result<(), DialectError> {
    for (id, obj) in meta_schemas {
        for (vocab_id, _) in obj.vocabulary.iter().filter(|(_, required)| **required) {
            if !vocabularies.contains_key(vocab_id) {
                return Err(DialectError::MissingRequiredVocabulary {
                    vocabulary_id: vocab_id.clone(),
                    meta_schema_id: id.clone(),
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::{dialect::Vocabulary, schema::Object};

    use super::Dialect;

    #[test]
    fn test_new_empty_dialect() {
        let dialect = Dialect::new("http://example.com/dialect", &[], &[]);
        assert!(dialect.is_ok());
    }
    #[test]
    fn test_new_dialect() {
        let dialect = Dialect::new(
            "http://example.com/dialect",
            &[Object {
                id: Some("https://example/meta-schema".into()),
                vocabulary: [("https://example.com/vocab".to_string(), true)]
                    .into_iter()
                    .collect(),
                ..Default::default()
            }],
            &[Vocabulary {
                id: "https://example.com/vocab".into(),
                handlers: vec![],
            }],
        );
        assert!(dialect.is_ok());
        let dialect = dialect.unwrap();
        assert_eq!(dialect.id, "http://example.com/dialect");
        assert_eq!(dialect.vocabularies.len(), 1);
        assert_eq!(dialect.meta_schemas.len(), 1);
    }
    #[test]
    fn test_new_dialect_missing_schema_id() {
        let dialect = Dialect::new(
            "https://example.com/dialect",
            &[Object {
                id: Some("https://example/meta-schema".try_into().unwrap()),
                vocabulary: [("https://example.com/vocab1".to_string(), true)]
                    .into_iter()
                    .collect(),
                ..Default::default()
            }],
            &[],
        );
        assert!(dialect.is_err());
        let err = dialect.unwrap_err();

        assert!(matches!(
            err,
            super::DialectError::MissingRequiredVocabulary { .. }
        ));
    }
}
