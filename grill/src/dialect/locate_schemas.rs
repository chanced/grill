use std::collections::HashMap;

use crate::{error::LocateSchemasError, keyword::Keyword, AbsoluteUri};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde_json::Value;

use super::Dialects;

/// Locates identified or anchored schemas within a [`Value`].
///
///
pub trait LocateSchemas: Send + Sync + DynClone {
    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError>;
}
clone_trait_object!(LocateSchemas);

impl<F> LocateSchemas for F
where
    F: Send
        + 'static
        + Sync
        + Clone
        + for<'v, 'd> Fn(
            Pointer,
            &'v Value,
            Dialects,
            &AbsoluteUri,
        ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError>,
{
    fn locate_schemas<'v>(
        &self,
        path: Pointer,
        value: &'v Value,
        dialects: Dialects,
        base_uri: &AbsoluteUri,
    ) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
        (self)(path, value, dialects, base_uri)
    }
}

#[derive(Debug, Clone)]
pub struct LocatedSchema<'v> {
    /// The Uri of the Schema
    pub uri: AbsoluteUri,
    pub value: &'v Value,
    pub path: Pointer,
    pub keyword: Option<Keyword<'static>>,
}

impl<'v> LocatedSchema<'v> {
    #[must_use]
    pub fn new(
        uri: AbsoluteUri,
        value: &'v Value,
        path: Pointer,
        keyword: Option<Keyword<'static>>,
    ) -> Self {
        Self {
            uri,
            value,
            path,
            keyword,
        }
    }
}

pub(crate) struct LocatedSchemas<'v> {
    pub(crate) schemas: HashMap<Pointer, Vec<LocatedSchema<'v>>>,
}
impl<'v> From<Vec<LocatedSchema<'v>>> for LocatedSchemas<'v> {
    fn from(schemas: Vec<LocatedSchema<'v>>) -> Self {
        let mut this = Self {
            schemas: HashMap::default(),
        };
        for schema in schemas {
            this.schemas
                .entry(schema.path.clone())
                .or_default()
                .push(schema);
        }
        this
    }
}
