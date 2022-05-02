use crate::{json, Dialect, Schema, TypeOrTypes, TypeVisitor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde::Deserializer;
#[derive(Debug, Deserialize, Serialize)]
pub struct Draft202012 {
    pub dialect: Dialect,

    #[serde(rename = "$id")]
    pub id: Option<String>,
    /// `prefixItems`
    pub properties: Option<HashMap<String, Schema>>,
    pub prefix_items: Option<Vec<Schema>>,
    pub items: Option<Box<Schema>>,
    //
    // ---------------------------------
    //        schema/validation
    // ---------------------------------
    //
    /// `type`
    pub type_: Option<TypeOrTypes>,
    /// `type`
    pub const_: bool,
    /// `enum`
    pub enum_: Option<Vec<json::Value>>,
    /// `multipleOf`
    pub multiple_of: Option<json::Number>,
    /// `maximum`
    pub maximum: Option<json::Number>,
    /// `exclusiveMaximum`
    pub exclusive_maximum: Option<json::Number>,
    /// `minimum`
    pub minimum: Option<json::Number>,
    /// `maxLength`
    pub max_length: Option<json::Number>,
    /// `minLength`
    pub min_length: Option<json::Number>,
    /// `pattern`
    pub pattern: Option<String>,
    /// `maxItems`
    pub max_items: Option<json::Number>,
    /// `minItems`
    pub min_items: Option<json::Number>,
    /// `uniqueItems`
    pub unique_items: Option<bool>,
    /// `maxContains`
    pub max_contains: Option<json::Number>,
    /// `minContains`
    pub min_contains: Option<json::Number>,
    /// `required`
    pub required: Option<Vec<String>>,

    /// ## `dependentRequired`
    /// The `dependentRequired` keyword conditionally requires that certain
    /// `properties` must be present if a given property is present in an
    /// object. For example, suppose we have a schema representing a customer.
    /// If you have their credit card number, you also want to ensure you have a
    /// billing address. If you don’t have their credit card number, a billing
    /// address would not be required. We represent this dependency of one
    /// property on another using the dependentRequired keyword.
    ///
    /// The value of the `dependentRequired` keyword is an object. Each entry in
    /// the object maps from the name of a property, p, to an array of strings
    /// listing properties that are required if p is present.
    ///
    /// https://json-schema.org/understanding-json-schema/reference/conditionals.html#dependentrequired
    pub dependent_required: Option<HashMap<String, Vec<String>>>,
    //
    // ---------------------------------
    //        schema/unevaluated
    // ---------------------------------
    //
    /// ## `unevaluatedItems`
    unevaluated_items: Option<Box<Schema>>,

    unevaluated_properties: Option<HashMap<String, Schema>>,
}

impl Draft202012 {
    pub fn new(dialect: Dialect) -> Self {
        todo!()
        // Self {
        //     dialect,
        //     id: None,
        //     type_: None,
        //     properties: None,
        //     prefix_items: None,
        //     items: None,
        //     const_: false,
        //     enum_: None,
        //     multiple_of: None,
        //     maximum: None,
        //     exclusive_maximum: None,
        //     minimum: None,
        //     max_length: None,
        //     min_length: None,
        //     pattern: None,
        //     max_items: None,
        //     min_items: None,
        //     unique_items: None,
        //     max_contains: None,
        //     min_contains: None,
        //     required: None,
        //     dependent_required: None,
        //     unevaluated_items: None,
        // }
    }
}

fn parse_202012<E: serde::de::Error>(
    map: &json::Map<String, json::Value>,
    dialect: Dialect,
) -> Result<Draft202012, E> {
    let mut schema = Draft202012::new(dialect);

    for (k, v) in map {
        match k.as_str() {
            "$id" => {
                schema.id = v.as_str().map(|s| s.to_string());
            }
            "type" => {
                schema.type_ = Some(
                    v.deserialize_any(TypeVisitor {})
                        .map_err(|e| serde::de::Error::custom(e))?,
                )
            }
            _ => {}
        }
    }
    todo!()
}

// =================================================
// data, need to double check everything
// =================================================

/*
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// `anchor`
    /// 202012,
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
    ///
    /// default: `true`
    target_schema: Option<HyperSchema>,
    /// `targetMediaType`
    target_media_type: Option<String>,
    /// `targetHints'
    target_hints: Option<Value>,
    /// `headerSchema`
    ///
    /// default: true
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
    id: Option<String>,
    /// `$schema`
    schema: Option<String>,
    /// $ref
    ref_: Option<String>,
    /// `$anchor`
    anchor: Option<String>,
    /// `$dynamicRef`
    dynamic_ref: Option<String>,
    /// `$dynamicAnchor`
    dynamic_anchor: Option<String>,
    /// `$vocabulary`
    vocabulary: Option<BTreeMap<String, bool>>,
    /// `$comment`
    comment: Option<String>,
    /// `$defs`
    defs: Option<BTreeMap<String, Schema>>,
    //
    // -------------------------------------------
    //                applicator
    // -------------------------------------------
    //
    /// `prefixItems`
    prefix_items: Option<Vec<Schema>>,
    /// `items`
    items: Option<Box<Schema>>,
    /// `contains`
    contains: Option<Box<Schema>>,
    /// `additionalProperties`
    additional_properties: Option<Box<Schema>>,
    /// `properties`
    properties: Option<BTreeMap<String, Schema>>,
    /// `patternProperties`
    pattern_properties: Option<BTreeMap<String, Schema>>,
    /// `dependentSchemas`
    dependent_schemas: Option<BTreeMap<String, Schema>>,
    /// `propertyNames`
    property_names: Option<Box<Schema>>,
    /// `if`
    if_: Option<Box<Schema>>,
    /// `then`
    then: Option<Box<Schema>>,
    /// `else`
    else_: Option<Box<Schema>>,
    // `allOf`
    all_of: Option<Vec<Schema>>,
    /// `anyOf`
    any_of: Option<Vec<Schema>>,
    /// `oneOf`
    one_of: Option<Vec<Schema>>,
    /// `not`
    not: Option<Box<Schema>>,
    //
    // -------------------------------------------
    //                unevaluated
    // -------------------------------------------
    //
    /// `unevaluatedItems`
    unevaluated_items: Option<Box<Schema>>,
    /// `unevaluatedProperties`
    unevaluated_properties: Option<BTreeMap<String, Schema>>,
    //
    // -------------------------------------------
    //                validation
    // -------------------------------------------
    //
    /// `type`
    type_: Option<StringOrStrings>,
    /// `const`
    const_: Option<Value>,
    /// `enum`
    enum_: Option<Vec<Value>>,
    /// `multipleOf`
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


*/
