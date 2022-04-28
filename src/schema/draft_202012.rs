use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Dialect, TypeOrTypes};
use crate::Schema;
use serde_json as json;

#[derive(Debug)]
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
}

impl Draft202012 {
    pub fn new(dialect: Dialect) -> Self {
        Self {
            dialect,
            id: None,
            typ: None,
            properties: None,
        }
    }
}
