use std::error::Error as StdError;

use grill_core::Resolve;
use slotmap::Key;

use crate::{compile, JsonSchema};

use super::Specification;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                    Error                                     ║
║                                   ¯¯¯¯¯¯¯                                    ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
pub trait Error<R, S, K>:
    'static + Send + StdError + From<compile::Error<S::Report<'static>, R>>
where
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
    R: 'static + Resolve,
{
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Context                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
#[trait_variant::make(Send)]
/// Context for [`Keyword::compile`].
pub trait Context<'int, 'txn, 'res, R, S, K>: Send + Sync
where
    R: 'static + Resolve + Send,
    S: Specification<K>,
    K: 'static + Key + Send + Sync,
{
    fn core_ctx(
        &mut self,
    ) -> &mut grill_core::lang::compile::Context<'int, 'txn, 'res, JsonSchema<K, S>, R, K>;
}
