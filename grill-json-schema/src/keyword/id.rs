//! # `$id` or `id` keyword.
//!
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core#section-8.2.1)
use std::fmt::Display;

use serde_json::Value;

use grill_core::{
    error::IdentifyError,
    keyword::{self, Kind, Unimplemented},
    schema::Identifier,
    Uri,
};

/// [`Keyword`] for `"$id"` or `"id"`
#[derive(Debug, Clone)]
pub struct Id {
    /// the keyword to use (e.g. `"$id"` or `"id"`)
    pub keyword: &'static str,
    /// Whether the [`Dialect`](grill_core::Dialect) allows for fragmented IDs
    pub allow_fragment: bool,
}

impl Id {
    /// Construct a new `Id` keyword.
    #[must_use]
    pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
        Self {
            keyword,
            allow_fragment,
        }
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} keyword", self.keyword)
    }
}

impl keyword::Keyword for Id {
    fn kind(&self) -> Kind {
        self.keyword.into()
    }
    fn identify(
        &self,
        schema: &Value,
    ) -> Result<Result<Option<Identifier>, IdentifyError>, Unimplemented> {
        let id = schema.get(self.keyword);
        Ok(match id {
            Some(Value::String(id)) => match Uri::parse(id) {
                Ok(uri) => {
                    if !self.allow_fragment && !uri.is_fragment_empty_or_none() {
                        return Ok(Err(IdentifyError::FragmentedId(uri)));
                    }
                    Ok(Some(Identifier::Primary(uri)))
                }
                Err(e) => Err(e.into()),
            },
            Some(v) => Err(IdentifyError::NotAString {
                value: Box::new(v.clone()),
                keyword: self.keyword,
            }),
            None => Ok(None),
        })
    }

    fn compile<'i>(
        &mut self,
        _compile: &mut keyword::Compile<'i>,
        _schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        Ok(false)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
        Ok(None)
    }
}
