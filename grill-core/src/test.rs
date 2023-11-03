use std::borrow::Cow;

use serde_json::json;

use crate::{
    schema::{dialect, Dialect},
    AbsoluteUri,
};

#[must_use]
pub fn build_dialect() -> dialect::Build {
    let uri = AbsoluteUri::parse("https://json-schema.org/draft/2020-12/schema").unwrap();
    Dialect::build(uri.clone())
        .add_metaschema(
            uri.clone(),
            Cow::Owned(json!({
                "$id": uri,
                "$schema": uri.clone()
            })),
        )
        .add_keyword(keyword::id::Id::new("$id", false))
        .add_keyword(keyword::schema::Schema::new("$schema", false))
}
pub mod keyword {

    pub mod id {

        use serde_json::Value;

        use crate::{
            error::{self, IdentifyError},
            keyword::{self, Kind, Unimplemented},
            schema::Identifier,
            Uri,
        };
        use std::fmt::Display;
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

        impl crate::keyword::Keyword for Id {
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
                _compile: &mut keyword::Compile<'i>,
                _schema: crate::Schema<'i>,
            ) -> Result<bool, error::CompileError> {
                Ok(false)
            }

            fn evaluate<'i, 'v>(
                &'i self,
                _ctx: &'i mut keyword::Context,
                _value: &'v Value,
            ) -> Result<Option<crate::output::Output<'v>>, crate::error::EvaluateError>
            {
                Ok(None)
            }
        }
    }

    pub mod schema {
        use crate::{
            error::{CompileError, EvaluateError, IdentifyError},
            keyword::{self, Context, Kind, Unimplemented},
            uri::AbsoluteUri,
            Output, Schema,
        };
        use serde_json::Value;

        #[derive(Debug, Clone)]
        pub struct Schema {
            pub keyword: &'static str,
            pub allow_fragment: bool,
        }

        impl Schema {
            #[must_use]
            pub fn new(keyword: &'static str, allow_fragment: bool) -> Self {
                Self {
                    keyword,
                    allow_fragment,
                }
            }
        }
        impl keyword::Keyword for Schema {
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

            fn compile<'i>(
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
    }
}
