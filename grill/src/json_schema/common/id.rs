use std::fmt::Display;

use serde_json::Value;

use crate::{
    error::{IdentifyError, Unimplemented},
    keyword::{Keyword, Kind},
    schema::Identifier,
    Uri,
};

#[derive(Debug, Clone)]
pub struct Id {
    pub keyword: &'static str,
    pub allow_fragment: bool,
}

impl Id {
    #[must_use]
    pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
        Self {
            keyword,
            allow_fragment,
        }
    }

    #[must_use]
    pub fn new_with_translate(keyword: &'static str, allow_fragment: bool) -> Self {
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

impl Keyword for Id {
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
        _compile: &mut crate::keyword::Compile<'i>,
        _schema: crate::Schema<'i>,
    ) -> Result<bool, crate::error::CompileError> {
        Ok(false)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut crate::keyword::Context,
        _value: &'v Value,
    ) -> Result<Option<crate::output::Output<'v>>, crate::error::EvaluateError> {
        todo!()
    }
}
