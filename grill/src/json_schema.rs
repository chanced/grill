//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.

pub mod draft_04;
pub mod draft_07;
pub mod draft_2019_09;
pub mod draft_2020_12;

pub use draft_04::dialect as json_schema_04_dialect;
pub use draft_07::dialect as json_schema_07_dialect;
pub use draft_2019_09::dialect as json_schema_2019_09_dialect;
pub use draft_2020_12::dialect as json_schema_2020_12_dialect;

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

// #[must_use]
// pub fn ident_schema_location_by_dynamic_anchor<'v>(
//     path: &Pointer,
//     value: &'v Value,
//     base_uri: &AbsoluteUri,
// ) -> Option<LocatedSchema<'v>> {
//     let Some(Value::String(anchor)) = value.get(Keyword::DYNAMIC_ANCHOR.as_str()) else { return None };
//     if anchor.is_empty() {
//         return None;
//     }
//     let path = path.clone();
//     let mut uri = base_uri.clone();
//     uri.set_fragment(Some(anchor));
//     Some(Subschema {
//         uri,
//         value,
//         path,
//         keyword: Some(Keyword::DYNAMIC_ANCHOR),
//     })
// }

// #[must_use]
// pub fn ident_schema_location_by_anchor<'v>(
//     path: &Pointer,
//     value: &'v Value,
//     base_uri: &AbsoluteUri,
// ) -> Option<Subschema<'v>> {
//     let Some(Value::String(anchor)) = value.get(Keyword::ANCHOR.as_str()) else { return None };
//     if anchor.is_empty() {
//         return None;
//     }
//     let path = path.clone();
//     let mut uri = base_uri.clone();
//     uri.set_fragment(Some(anchor));
//     Some(Subschema {
//         uri,
//         value,
//         path,
//         keyword: Some(Keyword::ANCHOR),
//     })
// }

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

// fn identify_schema_location_by_path<'v>(
//     path: &Pointer,
//     value: &'v Value,
//     uri: &AbsoluteUri,
// ) -> Subschema<'v> {
//     let mut uri = uri.clone();
//     if !path.is_empty() {
//         uri.set_fragment(Some(path));
//     }

//     Subschema::new(uri, value, path.clone(), None)
// }

// fn identify_schema_location_by_id<'v>(
//     path: &Pointer,
//     value: &'v Value,
//     uri: &mut AbsoluteUri,
//     dialects: &mut Dialects,
// ) -> Result<Option<Subschema<'v>>, IdentifyError> {
//     // Only identified schemas are indexed initially. If a handler needs an
//     // unidentified schema, it will be indexed upon use as part of the `compile`
//     // step.
//     let dialect = dialects.default_dialect();
//     let Ok(Some(id)) = dialect.identify(value) else { return Ok(None) };
//     match id {
//         Uri::Url(url) => {
//             *uri = AbsoluteUri::Url(url);
//         }
//         Uri::Urn(urn) => {
//             *uri = AbsoluteUri::Urn(urn);
//         }
//         Uri::Relative(rel) => {
//             uri.set_path_or_nss(rel.path())?;
//             uri.set_fragment(rel.fragment());
//         }
//     }
//     let path = path.clone();
//     Ok(Some(Subschema {
//         keyword: Some(Keyword::ID),
//         path,
//         uri: uri.clone(),
//         value,
//     }))
// }

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
