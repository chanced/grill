//! # `$anchor` keyword.
//!
//! - [Learn JSON Schema - anchor](https://www.learnjsonschema.com/2020-12/core/anchor/)
//! - [Draft 2020-12 Specification](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.3.2.1)

use std::{borrow::Cow, sync::Arc};

use grill_core::{
    error::{AnchorError, EvaluateError, Expected, InvalidTypeError},
    keyword::{Context, Keyword, Kind, Unimplemented},
    output::Output,
    schema, static_pointer_fn, Key,
};

use serde_json::Value;

use super::ANCHOR;

/// [`Keyword`] implementation for `$anchor`.
#[derive(Debug, Clone, Default)]
pub struct Anchor {
    /// The value of `$anchor`.
    pub anchor: String,
    /// The value of `$anchor` as a [`Value`] wrapped in an `Arc`.
    pub anchor_value: Arc<Value>,
    /// the default [`Key`] for the schema.
    pub key: Key,
}

impl Anchor {}
impl Keyword for Anchor {
    fn kind(&self) -> Kind {
        Kind::Keyword(ANCHOR)
    }
    fn compile<'i>(
        &mut self,
        _compile: &mut grill_core::keyword::Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        let Some(value) = schema.get(ANCHOR) else {
            return Ok(false);
        };
        let Value::String(anchor) = value else {
            return Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(value.clone()),
            }
            .into());
        };

        self.key = schema.key;
        self.anchor_value = Arc::new(value.clone());
        self.anchor = anchor.clone();
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
        // TODO: should $anchor annotate?
        // Ok(Some(ctx.annotate(
        //     Some(ANCHOR),
        //     Some(Annotation::Arc(self.anchor_value.clone())),
        // )))
    }

    fn anchors(
        &self,
        schema: &Value,
    ) -> Result<Result<Vec<schema::Anchor>, AnchorError>, Unimplemented> {
        let Some(anchor) = schema.get(ANCHOR) else {
            return Ok(Ok(Vec::new()));
        };
        let Value::String(anchor) = anchor else {
            return Ok(Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(anchor.clone()),
            }
            .into()));
        };
        if let Err(err) = super::validate_anchor(ANCHOR, anchor) {
            return Ok(Err(err));
        }
        Ok(Ok(vec![schema::Anchor {
            name: anchor.clone(),
            path: Cow::Borrowed(anchor_pointer()),
            keyword: ANCHOR,
        }]))
    }
}

static_pointer_fn!(pub anchor "/$anchor");

#[cfg(test)]
mod tests {
    use grill_core::Interrogator;
    use serde_json::json;

    use crate::JsonSchema;

    #[tokio::test]
    async fn test_setup() {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .init();

        let interrogator = Interrogator::build()
            .json_schema_2020_12()
            .source_owned_value(
                "https://localhost:1234/anchors/2",
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "http://localhost:1234/draft2020-12/root",
                    "$ref": "http://localhost:1234/draft2020-12/nested.json#foo",
                    "$defs": {
                        "A": {
                            "$id": "nested.json",
                            "$defs": {
                                "B": {
                                    "$anchor": "foo",
                                    "type": "integer"
                                }
                            }
                        }
                    }
                }),
            )
            .source_owned_value(
                "https://localhost:1234/anchors/3",
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$defs": {
                        "anchor_in_enum": {
                            "enum": [
                                {
                                    "$anchor": "my_anchor",
                                    "type": "null"
                                }
                            ]
                        },
                        "real_identifier_in_schema": {
                            "$anchor": "my_anchor",
                            "type": "string"
                        },
                        "zzz_anchor_in_const": {
                            "const": {
                                "$anchor": "my_anchor",
                                "type": "null"
                            }
                        }
                    },
                    "anyOf": [
                        { "$ref": "#/$defs/anchor_in_enum" },
                        { "$ref": "#my_anchor" }
                    ]
                }),
            )
            .source_owned_value(
                "https://localhost:1234/anchors/4",
                json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "$id": "http://localhost:1234/draft2020-12/foobar",
                    "$defs": {
                        "A": {
                            "$id": "child1",
                            "allOf": [
                                {
                                    "$id": "child2",
                                    "$anchor": "my_anchor",
                                    "type": "number"
                                },
                                {
                                    "$anchor": "my_anchor",
                                    "type": "string"
                                }
                            ]
                        }
                    },
                    "$ref": "child1#my_anchor"
                }),
            )
            .precompile(["https://localhost:1234/anchors/4"])
            .finish()
            .await
            .unwrap();
        let _ = interrogator;
    }
}
