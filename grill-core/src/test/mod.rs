use std::borrow::Cow;

use serde_json::json;

use crate::{
    keyword::Keyword,
    schema::{dialect, Dialect},
    AbsoluteUri,
};

pub fn build_dialect() -> dialect::Build {
    let uri = AbsoluteUri::parse("https://json-schema.org/draft/2020-12/schema").unwrap();
    Dialect::build(uri.clone()).with_metaschema(
        uri,
        Cow::Owned(json!({
            "$id": uri.clone(),
            "$schema": uri.clone()
        })),
    )
}
