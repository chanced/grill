use crate::{Map, Number, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::string::String;
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrStrings {
    String(String),
    Strings(Vec<String>),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// `anchor`
    /// - 202012,
    anchor: Option<String>,
    /// `anchorPoint
    anchor_point: Option<String>,
    /// `rel`
    rel: Option<StringOrStrings>,
    /// `href`
    href: Option<String>,
    /// `hrefSchema`
    ///
    /// default: `false`
    href_schema: Option<Box<HyperSchema>>,
    /// `templatePointers`
    template_pointers: Option<BTreeMap<String, String>>,
    /// `templateRequired`
    ///
    /// unique items -- use a BTreeSet for builder
    template_required: Option<Vec<String>>,
    /// `title`
    title: Option<String>,
    /// `description`
    description: Option<String>,
    /// `targetSchema`
    /// - 202012
    /// + default: `true`
    target_schema: Option<HyperSchema>,
    /// `targetMediaType`
    target_media_type: Option<String>,
    /// `targetHints'
    target_hints: Option<Value>,
    /// `headerSchema`
    ///
    /// - default: `true`
    header_schema: Option<Box<HyperSchema>>,

    /// `submissionMediaType`
    ///
    /// default: `"application/json"`
    submission_media_type: Option<String>,
    /// `submissionSchema
    ///
    /// default: `true`
    submission_schema: Option<HyperSchema>,

    /// `$comment`
    comment: Option<String>,

    #[serde(flatten)]
    uknown_fields: Map<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct HyperSchema {
    base: Option<String>,
    links: Option<Vec<Link>>,
    #[serde(flatten)]
    schema: Box<Schema>,
}

pub struct Schema {
    //
    // -------------------------------------------
    //                    core
    // -------------------------------------------
    //
    /// `$id`
    /// - 2020-12
    /// - 2019-09
    id: Option<String>,
    /// `$schema`
    /// - 2020-12
    /// - 2019-09
    schema: Option<String>,
    /// `$ref`
    /// - 2020-12
    /// - 2019-09
    ref_: Option<String>,
    /// `$anchor`
    /// - 2020-12
    /// - 2019-09
    anchor: Option<String>,
    /// `$dynamicRef`
    /// - 2020-12
    dynamic_ref: Option<String>,
    /// `$dynamicAnchor`
    /// - 2020-12
    dynamic_anchor: Option<String>,
    /// `$vocabulary`
    /// - 2020-12
    /// - 2019-09
    vocabulary: Option<BTreeMap<String, bool>>,
    /// `$comment`
    /// - 2020-12
    /// - 2019-09
    comment: Option<String>,
    /// `$defs`
    /// - 2020-12
    /// - 2019-09
    defs: Option<BTreeMap<String, Schema>>,
    /// `$recursiveRef`
    /// - 2019-09
    recursive_ref: Option<String>,
    /// `$recursiveAnchor`
    /// - 2019-09
    recursive_anchor: Option<String>,

    //
    // -------------------------------------------
    //                applicator
    // -------------------------------------------
    //
    /// `prefixItems`
    /// - 2020-12
    prefix_items: Option<Vec<Schema>>,
    /// `additionalItems`
    /// - 2019-09
    additional_items: Option<Box<Schema>>,
    /// `items`
    /// - 2020-12
    /// - 2019-09
    items: Option<Box<Schema>>,
    /// `contains`
    /// - 2020-12
    /// - 2019-09
    contains: Option<Box<Schema>>,
    /// `additionalProperties`
    /// - 2020-12
    /// - 2019-09
    additional_properties: Option<Box<Schema>>,
    /// `properties`
    /// - 2020-12
    /// - 2019-09
    properties: Option<BTreeMap<String, Schema>>,
    /// `patternProperties`
    /// - 2020-12
    /// - 2019-09
    pattern_properties: Option<BTreeMap<String, Schema>>,
    /// `dependentSchemas`
    /// - 2020-12
    /// - 2019-09
    dependent_schemas: Option<BTreeMap<String, Schema>>,
    /// `propertyNames`
    /// - 2020-12
    /// - 2019-09
    property_names: Option<Box<Schema>>,
    /// `if`
    /// - 2020-12
    /// - 2019-09
    if_: Option<Box<Schema>>,
    /// `then`
    /// - 2020-12
    /// - 2019-09
    then: Option<Box<Schema>>,
    /// `else`
    /// - 2020-12
    /// - 2019-09
    else_: Option<Box<Schema>>,
    /// `allOf`
    /// - 2020-12
    /// - 2019-09
    all_of: Option<Vec<Schema>>,
    /// `anyOf`
    /// - 2020-12
    /// - 2019-09
    any_of: Option<Vec<Schema>>,
    /// `oneOf`
    /// - 2020-12
    /// - 2019-09
    one_of: Option<Vec<Schema>>,
    /// `not`
    /// - 2020-12
    /// - 2019-09
    not: Option<Box<Schema>>,
    //
    // -------------------------------------------
    //                unevaluated
    // -------------------------------------------
    //
    /// `unevaluatedItems`
    /// - 2020-12
    /// - 2019-09
    unevaluated_items: Option<Box<Schema>>,
    /// `unevaluatedProperties`
    /// - 2019-09
    unevaluated_properties: Option<BTreeMap<String, Schema>>,
    //
    // -------------------------------------------
    //                validation
    // -------------------------------------------
    //
    /// `type`
    /// - 2020-12
    /// - 2019-09
    type_: Option<StringOrStrings>,
    /// `const`
    /// - 2020-12
    /// - 2019-09
    const_: Option<Value>,
    /// `enum`
    /// - 2020-12
    /// - 2019-09
    enum_: Option<Vec<Value>>,
    /// `multipleOf`
    /// - 2020-12
    /// - 2019-09
    multiple_of: Option<Number>,
    /// `maximum`
    maximum: Option<Number>,
    /// `exclusiveMaximum`
    exclusive_maximum: Option<Number>,
    /// `minimum`
    minimum: Option<Number>,
    /// `exclusiveMinimum`
    exclusive_minimum: Option<Number>,
    /// `maxLength`
    max_length: Option<Number>,
    /// `minLength`
    min_length: Option<Number>,
    /// `pattern`
    pattern: Option<String>,
    /// `maxItems`
    max_items: Option<Number>,
    /// `minItems`
    min_items: Option<Number>,
    /// `uniqueItems`
    unique_items: Option<bool>,
    /// `maxContains`
    max_contains: Option<Number>,
    /// `minContains`
    min_contains: Option<Number>,
    /// `maxProperties`
    max_properties: Option<Number>,
    /// `minProperties`
    min_properties: Option<Number>,
    /// `required`
    required: Option<Vec<String>>,
    /// `dependentRequired`
    dependent_required: Option<BTreeMap<String, Vec<String>>>,
    //
    // -------------------------------------------
    //                meta-data
    // -------------------------------------------
    //
    /// `title`
    title: Option<String>,
    /// `description`
    description: Option<String>,
    /// `default`
    default: Option<Value>,
    /// `deprecated`
    deprecated: Option<bool>,
    /// `readOnly`
    read_only: Option<bool>,
    /// `writeOnly`
    write_only: Option<bool>,
    /// `examples`
    examples: Option<Vec<Value>>,

    //
    // -------------------------------------------
    //                 content
    // -------------------------------------------
    //
    /// `contentEncoding`
    content_encoding: Option<String>,
    /// `contentMediaType`
    content_media_type: Option<String>,
    /// `contentSchema`
    content_schema: Option<Box<Schema>>,
    //
    // -------------------------------------------
    //            format-annotation
    // -------------------------------------------
    //
    /// `format`
    format: Option<String>,
}

struct FormAssertion {
    //
    // -------------------------------------------
    //            format-assertion
    // -------------------------------------------
    //
    /// `base`
    base: Option<String>,
}

impl<'de> Deserialize<'de> for Schema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}

impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}
