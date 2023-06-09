use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    ops::Deref,
};

use fancy_regex::CompileError;
use serde_json::Value;
use slotmap::SlotMap;

use crate::{
    dialect::Dialect,
    error::{BuildError, EvaluateError},
    graph::DependencyGraph,
    uri::AbsoluteUri,
    Handler, Output, Resolve, SchemaRef, Uri,
};

pub struct Schema {
    /// The URI of the schema.
    pub id: Option<AbsoluteUri>,
    /// The URI of the schema's `Metaschema`.
    pub meta_schema: AbsoluteUri,
    /// The Handlers associated with the schema.
    pub handlers: Box<[Handler]>,
}

/// A JSON Schema compiler and store.
pub struct Interrogator {
    base_uri: Option<AbsoluteUri>,
    dialects: Vec<Dialect>,
    dialect_lookup: HashMap<AbsoluteUri, usize>,
    default_dialect: Option<AbsoluteUri>,
    sources: HashMap<AbsoluteUri, Value>,
    resolvers: Vec<Box<dyn Resolve>>,
    schemas: SlotMap<SchemaRef, Schema>,
}

impl Interrogator {
    /// Attempts to compile the schema at the given URI if not already compiled,
    /// returning the [`SchemaRef`] of either the freshly compiled [`Schema`] or
    /// the existing [`SchemaRef`] of previously compiled, immutable [`Schema`].
    ///
    /// # Errors
    /// Returns [`CompileError`] if the schema fails to compile.
    pub fn compile(&mut self, uri: impl Borrow<AbsoluteUri>) -> Result<SchemaRef, CompileError> {
        todo!()
    }

    pub fn evaluate<'v>(
        &self,
        schema: SchemaRef,
        value: &'v Value,
    ) -> Result<Output<'v>, EvaluateError<'v>> {
        todo!()
    }
}

/// Constructs an [`Interrogator`].
#[derive(Default)]
pub struct Builder {
    dialects: Vec<Dialect>,
    base_uri: Option<AbsoluteUri>,
    sources: Vec<(AbsoluteUri, Value)>,
}

impl Builder {
    pub fn dialect(&mut self, dialect: impl Borrow<Dialect>) -> &mut Self {
        self.dialects.push(dialect.borrow().clone());
        self
    }
    pub fn base_uri(&mut self, uri: impl Borrow<AbsoluteUri>) -> &mut Self {
        self.base_uri = Some(uri.borrow().clone());
        self
    }
    pub fn source(
        &mut self,
        uri: impl Borrow<AbsoluteUri>,
        source: impl Borrow<Value>,
    ) -> &mut Self {
        self.sources
            .push((uri.borrow().clone(), source.borrow().clone()));
        self
    }

    /// Adds [`Dialect`]s for JSON Schema Drafts 2020-12, 2019-09, 7, and 4
    pub fn json_schema(&mut self) -> &mut Self {
        self.json_schema_04()
            .json_schema_07()
            .json_schema_2019_09()
            .json_schema_2020_12()
    }
    /// Adds JSON Schema 04 [`Dialect`]
    pub fn json_schema_04(&mut self) -> &mut Self {
        todo!()
        // self.dialect(crate::dialect::json_schema_04::json_schema_04_dialect())
    }

    /// Adds JSON Schema 07 [`Dialect`]
    pub fn json_schema_07(&mut self) -> &mut Self {
        self.dialect(crate::dialect::json_schema_07::json_schema_07_dialect())
    }

    pub fn json_schema_2019_09(&mut self) -> &mut Self {
        todo!()
        // self.dialect(crate::dialect::json_schema_2019_09::json_schema_2019_09_dialect())
    }

    pub fn json_schema_2020_12(&mut self) -> &mut Self {
        todo!()
        // self.dialect(crate::dialect::json_schema_2020_12::json_schema_2020_12_dialect())
    }

    pub fn sources<I, K, V>(&mut self, sources: I) -> &mut Self
    where
        K: Borrow<AbsoluteUri>,
        V: Borrow<Value>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in sources {
            self.sources.push((k.borrow().clone(), v.borrow().clone()));
        }
        self
    }
    pub fn build(self) -> Result<Interrogator, BuildError> {
        let Self {
            base_uri,
            dialects,
            sources,
        } = self;

        // let mut dialect_lookup = HashMap::new();
        // let graph = DependencyGraph::new();
        // let mut sources = HashMap::new();
        todo!()
    }
}
