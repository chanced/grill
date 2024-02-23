//! # `$schema` keyword.
//!
//! - [Learn JSON Schema - `$schema`](https://www.learnjsonschema.com/2020-12/core/schema/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)

use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError, Expected, IdentifyError, InvalidTypeError},
    keyword::{self, Context, Keyword, Kind, Unimplemented},
    output::Structures,
    uri::AbsoluteUri,
    Output,
};

use super::{UNEVALUATED_ITEMS, UNEVALUATED_PROPERTIES};

/// [`Keyword`] for `$schema`.
#[derive(Debug, Clone)]
pub struct Schema {
    /// the keyword to use (e.g. `$schema`)
    pub keyword: &'static str,
    /// Whether the [`Dialect`](grill_core::Dialect) allows for fragmented
    /// metaschema IDs
    pub allow_fragment: bool,
    /// Whether the schema is a boolean value
    pub boolean: Option<bool>,
    /// whether or not the schema can be short-circuited
    pub can_short_circuit: bool,
}

impl Schema {
    pub const ENABLING_STRUCTURES: Structures = Structures::FLAG;

    /// The set of keywords to check that disable short-circuiting
    pub const DISABLING_KEYWORDS: [&'static str; 2] = [UNEVALUATED_PROPERTIES, UNEVALUATED_ITEMS];

    /// Construct a new `Schema` keyword.
    #[must_use]
    pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
        Self {
            keyword,
            allow_fragment,
            boolean: None,
            can_short_circuit: true,
        }
    }
}

impl Keyword for Schema {
    fn kind(&self) -> Kind {
        self.keyword.into()
    }
    fn compile<'i>(
        &mut self,
        compile: &mut keyword::Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, CompileError> {
        match schema.value() {
            Value::Bool(bool) => {
                self.boolean = Some(*bool);
            }
            Value::Object(obj) => {
                for keyword in Self::DISABLING_KEYWORDS {
                    if obj.contains_key(keyword) {
                        self.can_short_circuit = false;
                        return Ok(true);
                    }
                }
            }
            other => {
                // there should probably be a variant specifically for invalid schema type
                return Err(InvalidTypeError {
                    expected: Expected::AnyOf(&[Expected::Bool, Expected::Object]),
                    actual: Box::new(other.clone()),
                }
                .into());
            }
        }
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        if !self.can_short_circuit {
            ctx.disable_short_circuiting();
            return Ok(None);
        }
        if Self::ENABLING_STRUCTURES.contains(ctx.structure().into()) {
            ctx.enable_short_circuiting();
        }
        let Some(bool) = self.boolean else {
            return Ok(None);
        };
        if bool {
            Ok(Some(ctx.annotate(None, None)))
        } else {
            Ok(Some(ctx.error(None, None)))
        }
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
}
