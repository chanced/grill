use grill_uri::AbsoluteUri;
use slotmap::Key;

use crate::{state::Transaction, Resolve};

use super::Language;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Compile                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// Request to compile a schema.
#[derive(Debug)]
pub struct Context<'int, 'txn, 'res, L, R, K>
where
    L: Language<K>,
    K: 'static + Key + Send + Sync,
{
    /// Uris to compile
    pub targets: Vec<AbsoluteUri>,

    /// Current state of the [`Interrogator`], including schemas, sources, and
    /// cache. Upon successful compilation, the data will become to new state.
    pub state: Transaction<'int, 'txn, L, K>,

    /// Implementation of [`Resolve`]
    pub resolve: &'res R,

    /// Whether or not to validate the schemas during compilation
    pub must_validate: bool,
}

impl<'int, 'txn, 'res, L, R, K> Context<'int, 'txn, 'res, L, R, K>
where
    L: Language<K>,
    K: 'static + Key + Send + Sync,
{
    pub(crate) fn new(
        uris: Vec<AbsoluteUri>,
        txn: Transaction<'int, 'txn, L, K>,
        resolve: &'res R,
        validate: bool,
    ) -> Self
    where
        L: Language<K>,
        K: 'static + Key + Send + Sync,
        R: 'static + Resolve + Send + Sync,
    {
        Self {
            targets: uris,
            state: txn,
            resolve,
            must_validate: validate,
        }
    }
}
