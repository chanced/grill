use crate::{json, Dialect, Schema, TypeOrTypes, TypeVisitor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use serde::Deserializer;
#[derive(Debug, Deserialize, Serialize)]
pub struct Draft202012 {
    /// The `$schema` keyword is used to declare which dialect of JSON Schema the
    /// schema was written for. The value of the `$schema` keyword is also the
    /// identifier for a schema that can be used to verify that the schema is valid
    /// according to the dialect $schema identifies. A schema that describes another
    /// schema is called a “meta-schema”.
    ///
    /// `$schema` applies to the entire document and must be at the root level. It
    /// does not apply to externally referenced (`$ref`, `$dynamicRef`) documents. Those
    /// schemas need to declare their own $schema.
    ///
    /// If `$schema` is not used, an implementation might allow you to specify a
    /// value externally or it might make assumptions about which specification
    /// version should be used to evaluate the schema. It’s recommended that all
    /// JSON Schemas have a `$schema` keyword to communicate to readers and tooling
    /// which specification version is intended.
    ///
    /// see also:
    /// - https://json-schema.org/understanding-json-schema/reference/schema.html#schema
    /// - https://json-schema.org/draft/2020-12/json-schema-core.html#rfc.section.8.1.1
    pub dialect: Dialect,
    /// The `$id` keyword identifies a schema resource with its canonical URI.
    ///
    /// Note that this URI is an identifier and not necessarily a network locator.
    /// In the case of a network-addressable URL, a schema need not be downloadable
    /// from its canonical URI.
    ///
    /// If present, the value for this keyword MUST be a string, and MUST represent
    /// a valid URI-reference. This URI-reference SHOULD be normalized, and MUST
    /// resolve to an absolute-URI (without a fragment). Therefore, `$id` MUST NOT
    /// contain a non-empty fragment, and SHOULD NOT contain an empty fragment.
    ///
    /// Since an empty fragment in the context of the application/schema+json media
    /// type refers to the same resource as the base URI without a fragment, an
    /// implementation MAY normalize a URI ending with an empty fragment by removing
    /// the fragment. However, schema authors SHOULD NOT rely on this behavior
    /// across implementations. [CREF3]
    ///
    /// This URI also serves as the base URI for relative URI-references in keywords
    /// within the schema resource, in accordance with RFC 3986 section 5.1.1
    /// regarding base URIs embedded in content.
    ///
    /// The presence of `$id` in a subschema indicates that the subschema
    /// constitutes a distinct schema resource within a single schema document.
    /// Furthermore, in accordance with RFC 3986 section 5.1.2 regarding
    /// encapsulating entities, if an `$id` in a subschema is a relative
    /// URI-reference, the base URI for resolving that reference is the URI of the
    /// parent schema resource.
    ///
    /// If no parent schema object explicitly identifies itself as a resource with
    /// `$id`, the base URI is that of the entire document, as established by the
    /// steps given in the previous section.
    ///
    /// https://json-schema.org/draft/2020-12/json-schema-core.html#rfc.section.8.2.1
    ///
    /// https://json-schema.org/understanding-json-schema/structuring.html#id
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
    /// The value of "unevaluatedItems" MUST be a valid JSON Schema.
    ///
    /// The behavior of this keyword depends on the annotation results of adjacent
    /// keywords that apply to the instance location being validated. Specifically,
    /// the annotations from "prefixItems", "items", and "contains", which can come
    /// from those keywords when they are adjacent to the "unevaluatedItems" keyword.
    /// Those three annotations, as well as "unevaluatedItems", can also result from
    /// any and all adjacent in-place applicator keywords. This includes but is not
    /// limited to the in-place applicators defined in this document.
    ///
    /// If no relevant annotations are present, the "unevaluatedItems" subschema MUST
    /// be applied to all locations in the array. If a boolean true value is present
    /// from any of the relevant annotations, "unevaluatedItems" MUST be ignored.
    /// Otherwise, the subschema MUST be applied to any index greater than the
    /// largest annotation value for "prefixItems", which does not appear in any
    /// annotation value for "contains".
    ///
    /// This means that "prefixItems", "items", "contains", and all in-place
    /// applicators MUST be evaluated before this keyword can be evaluated. Authors
    /// of extension keywords MUST NOT define an in-place applicator that would need
    /// to be evaluated after this keyword.
    ///
    /// If the "unevaluatedItems" subschema is applied to any positions within the
    /// instance array, it produces an annotation result of boolean true, analogous
    /// to the behavior of "items". Omitting this keyword has the same assertion
    /// behavior as an empty schema.
    ///
    /// https://json-schema.org/draft/2020-12/json-schema-core.html#rfc.section.11.2
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


*/
