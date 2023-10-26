//! # `$schema` keyword.
//!
//! - [Learn JSON Schema - const](https://www.learnjsonschema.com/2020-12/core/schema/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)

use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError, IdentifyError},
    keyword::{self, Context, Keyword, Kind, Unimplemented},
    uri::AbsoluteUri,
    Output,
};

/// [`Keyword`] for `$schema`.
#[derive(Debug, Clone)]
pub struct Schema {
    /// the keyword to use (e.g. `$schema`)
    pub keyword: &'static str,
    /// Whether the [`Dialect`](grill_core::Dialect) allows for fragmented
    /// metaschema IDs
    pub allow_fragment: bool,
}

impl Schema {
    /// Construct a new `Schema` keyword.
    #[must_use]
    pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
        Self {
            keyword,
            allow_fragment,
        }
    }
}

impl Keyword for Schema {
    fn kind(&self) -> Kind {
        self.keyword.into()
    }
    fn dialect(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<AbsoluteUri>, IdentifyError>, Unimplemented> {
        let Some(schema) = schema.get(self.keyword) else {
            return Ok(Ok(None));
        };
        let schema = schema.as_str().ok_or(IdentifyError::NotAString {
            keyword: self.keyword,
            value: Box::new(schema.clone()),
        });
        if let Err(err) = schema {
            return Ok(Err(err));
        }
        let schema = schema.unwrap();
        let uri = AbsoluteUri::parse(schema)
            .map(Some)
            .map_err(IdentifyError::InvalidUri);
        if let Err(err) = uri {
            return Ok(Err(err));
        }
        let uri = uri.unwrap();
        if uri.is_none() {
            return Ok(Ok(None));
        }
        let uri = uri.unwrap();
        if !self.allow_fragment && !uri.is_fragment_empty_or_none() {
            return Ok(Err(IdentifyError::FragmentedId(uri.into())));
        }
        Ok(Ok(Some(uri)))
    }

    fn setup<'i>(
        &mut self,
        _compile: &mut keyword::Compile<'i>,
        _schema: grill_core::Schema<'i>,
    ) -> Result<bool, CompileError> {
        Ok(false)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
