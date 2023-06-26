//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;

pub use draft_04::dialect as draft_04_dialect;
pub use draft_07::dialect as draft_07_dialect;
pub use draft_2019_09::dialect as draft_2019_09_dialect;
pub use draft_2020_12::dialect as draft_2020_12_dialect;

pub use draft_04::{
    is_json_hyper_schema_04_absolute_uri, is_json_hyper_schema_04_uri, is_json_schema_04,
    is_json_schema_04_absolute_uri, is_json_schema_04_uri, json_hyper_schema_04_absolute_uri,
    json_hyper_schema_04_uri, json_hyper_schema_04_url, json_schema_04_absolute_uri,
    json_schema_04_uri, json_schema_04_url,
};

pub use draft_07::{
    is_json_hyper_schema_07_absolute_uri, is_json_hyper_schema_07_uri, is_json_schema_07,
    is_json_schema_07_absolute_uri, is_json_schema_07_uri, json_hyper_schema_07_absolute_uri,
    json_hyper_schema_07_uri, json_hyper_schema_07_url, json_schema_07_absolute_uri,
    json_schema_07_uri, json_schema_07_url,
};
pub use draft_2019_09::{
    is_json_hyper_schema_2019_09_absolute_uri, is_json_hyper_schema_2019_09_uri,
    is_json_schema_2019_09, is_json_schema_2019_09_absolute_uri, is_json_schema_2019_09_uri,
    json_hyper_schema_2019_09_absolute_uri, json_hyper_schema_2019_09_uri,
    json_hyper_schema_2019_09_url, json_schema_2019_09_absolute_uri, json_schema_2019_09_uri,
    json_schema_2019_09_url,
};

pub use draft_2020_12::{
    is_json_hyper_schema_2020_12_absolute_uri, is_json_hyper_schema_2020_12_uri,
    is_json_schema_2020_12, is_json_schema_2020_12_absolute_uri, is_json_schema_2020_12_uri,
    json_hyper_schema_2020_12_absolute_uri, json_hyper_schema_2020_12_uri,
    json_hyper_schema_2020_12_url, json_schema_2020_12_absolute_uri, json_schema_2020_12_uri,
    json_schema_2020_12_url,
};
use jsonptr::{Pointer, Token};

use crate::{
    dialect::{Dialect, Dialects, LocatedSchema},
    error::{HasFragmentError, IdentifyError, LocateSchemasError, UriParseError},
    AbsoluteUri, Array, Uri,
};
use crate::{keyword::Keyword, Anchor};
use serde_json::Value;

fn ident_schema_location_by_anchor<'v>(
    path: Pointer,
    value: &'v Value,
    base_uri: &AbsoluteUri,
) -> Option<LocatedSchema<'v>> {
    let Some(Value::String(anchor)) = value.get(Keyword::ANCHOR.as_str()) else { return None };
    if anchor.is_empty() {
        return None;
    }
    let mut uri = base_uri.clone();
    uri.set_fragment(Some(anchor));
    Some(LocatedSchema {
        uri,
        value,
        path,
        keyword: Keyword::ANCHOR,
    })
}

// removed for now: https://json-schema.slack.com/archives/CT8QRGTK5/p1687528273221999
// fn append_nested_named_locations(located_schemas: &mut Vec<LocatedSchema>, base_uri: &AbsoluteUri) {
//     let mut append = Vec::new();
//     for located in located_schemas.iter() {
//         if located.uri.authority_or_namespace() != base_uri.authority_or_namespace()
//             || located.uri.path_or_nss() != base_uri.path_or_nss()
//         {
//             if let Some(fragment) = located.uri.fragment() {
//                 if !fragment.is_empty() && !fragment.starts_with('/') {
//                     let mut uri = base_uri.clone();
//                     uri.set_fragment(Some(fragment));
//                     append.push(LocatedSchema {
//                         uri,
//                         value: located.value,
//                         path: located.path.clone(),
//                         keyword: located.keyword,
//                     });
//                 }
//             }
//         }
//     }
//     located_schemas.append(&mut append);
// }

fn locate_schemas_in_array<'v>(
    path: Pointer,
    arr: &'v Array,
    dialects: Dialects,
    base_uri: &AbsoluteUri,
) -> Result<Vec<LocatedSchema<'v>>, LocateSchemasError> {
    let located = arr
        .iter()
        .enumerate()
        .map(|(key, value)| {
            let tok = key.into();
            let mut path = path.clone();
            path.push_back(tok);
            dialects
                .default_dialect()
                .locate_schemas(path, value, dialects, base_uri)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(located)
}

fn identify_schema_location_by_id<'v>(
    path: &Pointer,
    value: &'v Value,
    base_uri: &AbsoluteUri,
    dialect: &Dialect,
) -> Option<(LocatedSchema<'v>, LocatedSchema<'v>)> {
    // Only identified schemas are indexed initially. If a handler needs an
    // unidentified schema, it will be indexed upon use as part of the `compile`
    // step for a handler.
    let Ok(Some(id)) = dialect.identify(value) else { return None};
    let path = path.clone();
    let mut uri = base_uri.clone();
    if !path.is_empty() {
        uri.set_fragment(Some(&path));
    }
    let by_ptr = LocatedSchema {
        keyword: Keyword::ID,
        path: path.clone(),
        uri,
        value,
    };
    let uri = match id {
        Uri::Url(url) => AbsoluteUri::Url(url),
        Uri::Urn(urn) => AbsoluteUri::Urn(urn),
        Uri::Relative(rel) => {
            let mut base = base_uri.clone();
            base.set_path_or_nss(rel.path()).unwrap();
            base.set_fragment(None);
            base
        }
    };
    let by_id = LocatedSchema {
        keyword: Keyword::ID,
        path,
        uri,
        value,
    };
    Some((by_ptr, by_id))
}

// #[derive(Default)]
// pub struct JsonSchema<'instance, 'schema, 'state> {
//     _instance_marker: PhantomData<&'instance ()>,
//     _schema_marker: PhantomData<&'schema ()>,
//     _state_marker: PhantomData<&'state ()>,
// }

// impl<'instance, 'schema, 'state> Integration for JsonSchema<'instance, 'schema, 'state> {
//     type Output = Value;
//     type Annotation = Annotation<'instance>;
//     type PartialId = Uri;
//     type Id = AbsoluteUri;
//     type Scope = Scope<'state>;
//     type Compile = Compile<'schema>;
// }
#[cfg(test)]
mod tests {}
