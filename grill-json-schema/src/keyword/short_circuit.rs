//! Determines whether or not an evaluation can be short-circuited

use grill_core::{
    keyword::{Keyword, Kind},
    output::Structures,
    Structure,
};
use serde_json::Value;

/// Determines whether or not an evaluation can be short-circuited
/// based on the target `Structure` and whether or not any of the
/// supplied keywords are present in the schema.
#[derive(Clone, Debug)]
pub struct ShortCircuit {
    /// `Structure`s which would enable short-circuiting
    pub enabling_structures: Structures,
    /// The set of keywords to check that disable short-circuiting
    pub disabling_keywords: Vec<&'static str>,

    /// whether or not the schema can be short-circuited
    pub can_short_circuit: bool,
}

impl Keyword for ShortCircuit {
    fn kind(&self) -> Kind {
        Kind::Logic
    }

    fn compile<'i>(
        &mut self,
        _compile: &mut grill_core::keyword::Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        if let Value::Object(obj) = schema.value() {
            for keyword in &self.disabling_keywords {
                if obj.contains_key(*keyword) {
                    self.can_short_circuit = false;
                    return Ok(true);
                }
            }
        }
        Ok(true)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut grill_core::keyword::Context,
        _value: &'v serde_json::Value,
    ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
        if !self.can_short_circuit {
            ctx.disable_short_circuiting();
            return Ok(None);
        }
        if self.enabling_structures.contains(ctx.structure().into()) {
            ctx.enable_short_circuiting();
        }
        Ok(None)
    }
}

impl ShortCircuit {
    /// Construct a new `ShortCircuit` keyword.
    #[must_use]
    pub fn new(
        enabling_structures: impl IntoIterator<Item = Structure>,
        disabling_keywords: impl IntoIterator<Item = &'static str>,
    ) -> ShortCircuit {
        Self {
            enabling_structures: enabling_structures.into(),
            disabling_keywords: disabling_keywords.into_iter().collect(),
            can_short_circuit: true,
        }
    }
}
