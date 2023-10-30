use std::{borrow::Cow, collections::HashMap, sync::Arc};

use grill_core::{
    error::{AnchorError, EvaluateError, Expected, InvalidTypeError},
    keyword::{Context, Keyword, Kind, Unimplemented},
    output::{Annotation, Output},
    schema::Anchor,
    static_pointer_fn, Key,
};
use serde_json::Value;

use super::DYNAMIC_ANCHOR;

/// [`Keyword`] implementation for `$dynamicAnchor`.
#[derive(Debug, Clone, Default)]
pub struct DynamicAnchor {
    /// The value of `$dynamicAnchor`.
    pub dynamic_anchor: String,
    /// The value of `$dynamicAnchor` as a [`Value`] wrapped in an `Arc`.
    pub dynamic_anchor_value: Arc<Value>,
    /// the default [`Key`] for the schema.
    pub key: Key,
}
impl DynamicAnchor {
    /// Constructs a new [`DynamicAnchor`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
impl Keyword for DynamicAnchor {
    fn kind(&self) -> Kind {
        Kind::Keyword(DYNAMIC_ANCHOR)
    }
    fn compile<'i>(
        &mut self,
        _compile: &mut grill_core::keyword::Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        let Some(value) = schema.get(DYNAMIC_ANCHOR) else {
            return Ok(false);
        };
        let Value::String(dynamic_anchor) = value else {
            return Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(value.clone()),
            }
            .into());
        };
        self.key = schema.key;
        self.dynamic_anchor_value = Arc::new(value.clone());
        self.dynamic_anchor = dynamic_anchor.clone();
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        let dynamic_anchors = ctx.eval_state().entry::<DynamicAnchors>().or_default();
        dynamic_anchors.set(&self.dynamic_anchor, self.key);
        Ok(Some(ctx.annotate(
            Some(DYNAMIC_ANCHOR),
            Some(Annotation::Arc(self.dynamic_anchor_value.clone())),
        )))
    }

    fn anchors(&self, schema: &Value) -> Result<Result<Vec<Anchor>, AnchorError>, Unimplemented> {
        let Some(dynamic_anchor) = schema.get(DYNAMIC_ANCHOR) else {
            return Ok(Ok(Vec::new()));
        };
        let Value::String(dynamic_anchor) = dynamic_anchor else {
            return Ok(Err(InvalidTypeError {
                expected: Expected::String,
                actual: Box::new(dynamic_anchor.clone()),
            }
            .into()));
        };
        if let Err(err) = super::validate_anchor(DYNAMIC_ANCHOR, dynamic_anchor) {
            return Ok(Err(err));
        }
        Ok(Ok(vec![Anchor {
            name: dynamic_anchor.clone(),
            path: Cow::Borrowed(dynamic_anchor_pointer()),
            keyword: DYNAMIC_ANCHOR,
        }]))
    }
}

static_pointer_fn!(pub dynamic_anchor "/$dynamicAnchor");

/// A map of `$dynamicAnchor` values to [`Key`]s.
#[derive(Default, Clone, Debug)]
pub struct DynamicAnchors {
    map: HashMap<String, Key>,
}

impl DynamicAnchors {
    /// Constructs a new [`DynamicAnchors`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the [`Key`] for the given anchor, if it exists.
    #[must_use]
    pub fn get(&self, anchor: &str) -> Option<Key> {
        self.map.get(anchor).copied()
    }

    /// Sets the [`Key`] for the given anchor if it does not exist.
    pub fn set(&mut self, anchor: &str, key: Key) -> bool {
        if self.map.contains_key(anchor) {
            return false;
        }
        self.map.insert(anchor.to_string(), key);
        true
    }
}
