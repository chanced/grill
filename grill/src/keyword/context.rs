use jsonptr::Pointer;
use num_rational::BigRational;
use serde_json::{Number, Value};
use slotmap::SlotMap;

use crate::{
    error::{EvaluateError, NumberError},
    output::Node,
    schema::{CompiledSchema, Schemas},
    source::Sources,
    Key,
};

use super::{BigInts, BigRationals, State, Values};
/// Contains global and evaluation level state, schemas, and location
/// information needed to perform an
/// [`evaluation`](`crate::Interrogator::evaluate`).
pub struct Context<'i> {
    /// global state to the interrogator
    pub(crate) global_state: &'i mut State,
    /// per-evaluation state
    pub(crate) eval_state: &'i mut State,
    pub(crate) path: &'i Pointer,
    pub(crate) schemas: &'i Schemas,
    pub(crate) sources: &'i Sources,
    pub(crate) ints: &'i BigInts,
    pub(crate) rationals: &'i BigRationals,
    pub(crate) values: &'i Values,
}

impl<'s> Context<'s> {
    #[must_use]
    pub fn evalute<'v>(&self, key: Key) -> Result<Node<'v>, EvaluateError> {
        let schema = self.schemas.get(key, self.sources)?;
        let abs_loc = schema.id.as_deref().unwrap_or(&schema.uris[0]);
    }
}

fn x(r: impl std::io::Read) -> Result<Option<String>, serde_json::Error> {
    serde_json::from_reader(r).map(Option::Some)
}
