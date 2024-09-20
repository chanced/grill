use crate::spec::{self, Specification};
use grill_core::{Key, Resolve};
use serde_json::Value;

pub mod consts;
pub mod context;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Keyword                                    ║
║                                  ¯¯¯¯¯¯¯¯¯                                   ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A JSON Schema keyword.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keyword {}

impl<S, K> spec::keyword::Keyword<S, K> for Keyword
where
    S: Specification<K>,
    K: 'static + Key + Sync + Send,
{
    async fn compile<R>(
        &self,
        compile: S::Compile<'_, '_, '_, R>,
    ) -> Option<Result<(), S::CompileError<R>>>
    where
        R: 'static + Resolve + Send + Sync,
    {
        todo!()
    }

    fn evaluate<'int, 'val, 'req>(
        &self,
        eval: S::Evaluate<'int, 'val, 'req>,
    ) -> Result<(), S::EvaluateError> {
        _ = eval;
        todo!()
    }

    fn reference(&self, _schema: &Value) -> Option<spec::keyword::Found> {
        None
    }

    fn anchor(&self, _schema: &Value) -> Option<spec::keyword::Found> {
        None
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                                   Keywords                                   ║
║                                  ¯¯¯¯¯¯¯¯¯¯                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A slice of [`Keyword`]s belonging to a schema.
pub struct Keywords<'int>(pub &'int [Keyword]);

impl<'int> From<&'int [Keyword]> for Keywords<'int> {
    fn from(keywords: &'int [Keyword]) -> Self {
        Self(keywords)
    }
}
impl<'int> IntoIterator for Keywords<'int> {
    type Item = &'int Keyword;
    type IntoIter = std::slice::Iter<'int, Keyword>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Keywords<'_> {
    /// Returns the slice of keywords.
    pub fn as_slice(&self) -> &[Keyword] {
        self.0
    }
}
impl AsRef<[Keyword]> for Keywords<'_> {
    fn as_ref(&self) -> &[Keyword] {
        self.0
    }
}
