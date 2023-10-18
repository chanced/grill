use serde_json::Value;

use grill_core::{
    error::{CompileError, EvaluateError, IdentifyError},
    keyword::{self, Context, Kind, Unimplemented},
    uri::AbsoluteUri,
    Output, Schema,
};

#[derive(Debug, Clone)]
pub struct Keyword {
    pub keyword: &'static str,
    pub allow_fragment: bool,
}

impl Keyword {
    #[must_use]
    pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
        Self {
            keyword,
            allow_fragment,
        }
    }
}
impl keyword::Keyword for Keyword {
    fn kind(&self) -> Kind {
        self.keyword.into()
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

    fn setup<'i>(
        &mut self,
        _compile: &mut keyword::Compile<'i>,
        _schema: Schema<'i>,
    ) -> Result<bool, CompileError> {
        Ok(false)
    }

    fn evaluate<'i, 'v>(
        &'i self,
        _ctx: &'i mut Context,
        _value: &'v Value,
    ) -> Result<Option<Output<'v>>, EvaluateError> {
        Ok(None)
    }
}
