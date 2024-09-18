use serde_json::Value;
use slotmap::Key;

use crate::{cache::Cache, state::State};

use super::Language;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Evaluate                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Request to evaluate a schema.
pub struct Context<'int, 'req, 'val, L, K>
where
    L: Language<K>,
    K: 'static + Key + Send + Sync,
{
    /// Evaluation context `S::Context`
    pub context: L::Context,

    /// The current, immutable state of the [`Interrogator`]
    pub state: &'int State<L, K>,

    pub eval: &'req mut Cache,

    /// The key of the schema to evaluate
    pub key: K,

    /// The value to evaluate
    pub value: &'val Value,
}
