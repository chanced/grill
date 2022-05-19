pub struct CompiledSchema {
    //
    // -------------------------------------------
    //                    core
    // -------------------------------------------
    //
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
    ///
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub id: Option<String>,
    /// `$schema`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub schema: Option<String>,
    /// `$ref`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub ref_: Option<String>,
    /// `$anchor`
    /// - 2020-12
    /// - 2019-09
    pub anchor: Option<String>,
    /// `$dynamicRef`
    /// - 2020-12
    pub dynamic_ref: Option<String>,
    /// `$dynamicAnchor`
    /// - 2020-12
    pub dynamic_anchor: Option<String>,
    /// `$vocabulary`
    /// - 2020-12
    /// - 2019-09
    pub vocabulary: Option<BTreeMap<String, bool>>,
    /// `$comment`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub comment: Option<String>,
    /// `$defs`
    /// - 2020-12
    /// - 2019-09
    pub defs: Option<BTreeMap<String, SchemaDef>>,

    //
    // -------------------------------------------
    //                applicator
    // -------------------------------------------
    //
    /// `prefixItems`
    /// - 2020-12
    pub prefix_items: Option<Vec<SchemaDef>>,
    /// `additionalItems`
    /// - 2019-09
    /// - 07
    pub additional_items: Option<Box<SchemaDef>>,
    /// `items`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub items: Option<Box<SchemaDef>>,
    /// `contains`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub contains: Option<Box<SchemaDef>>,
    /// `additionalProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub additional_properties: Option<Box<SchemaDef>>,
    /// `properties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `{}`
    pub properties: Option<BTreeMap<String, SchemaDef>>,
    /// `patternProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `{}`
    pub pattern_properties: Option<BTreeMap<String, SchemaDef>>,
    /// `dependentSchemas`
    /// - 2020-12
    /// - 2019-09
    pub dependent_schemas: Option<BTreeMap<String, SchemaDef>>,
    /// `propertyNames`
    /// - 2020-12
    /// - 2019-09
    pub property_names: Option<Box<SchemaDef>>,
    /// `if`
    /// - 2020-12
    /// - 2019-09
    pub if_: Option<Box<SchemaDef>>,
    /// `then`
    /// - 2020-12
    /// - 2019-09
    pub then: Option<Box<SchemaDef>>,
    /// `else`
    /// - 2020-12
    /// - 2019-09
    pub else_: Option<Box<SchemaDef>>,
    /// `allOf`
    /// - 2020-12
    /// - 2019-09
    pub all_of: Option<Vec<SchemaDef>>,
    /// `anyOf`
    /// - 2020-12
    /// - 2019-09
    pub any_of: Option<Vec<SchemaDef>>,
    /// `oneOf`
    /// - 2020-12
    /// - 2019-09
    pub one_of: Option<Vec<SchemaDef>>,
    /// `not`
    /// - 2020-12
    /// - 2019-09
    pub not: Option<Box<SchemaDef>>,
    //
    // -------------------------------------------
    //                unevaluated
    // -------------------------------------------
    //
    /// The value of `unevaluatedItems` MUST be a valid JSON Schema.
    ///
    /// The behavior of this keyword depends on the annotation results of adjacent
    /// keywords that apply to the instance location being validated. Specifically,
    /// the annotations from "prefixItems", "items", and "contains", which can come
    /// from those keywords when they are adjacent to the `unevaluatedItems` keyword.
    /// Those three annotations, as well as `unevaluatedItems`, can also result from
    /// any and all adjacent in-place applicator keywords. This includes but is not
    /// limited to the in-place applicators defined in this document.
    ///
    /// If no relevant annotations are present, the `unevaluatedItems` subschema MUST
    /// be applied to all locations in the array. If a boolean true value is present
    /// from any of the relevant annotations, `unevaluatedItems` MUST be ignored.
    /// Otherwise, the subschema MUST be applied to any index greater than the
    /// largest annotation value for "prefixItems", which does not appear in any
    /// annotation value for "contains".
    ///
    /// This means that "prefixItems", "items", "contains", and all in-place
    /// applicators MUST be evaluated before this keyword can be evaluated. Authors
    /// of extension keywords MUST NOT define an in-place applicator that would need
    /// to be evaluated after this keyword.
    ///
    /// If the `unevaluatedItems` subschema is applied to any positions within the
    /// instance array, it produces an annotation result of boolean true, analogous
    /// to the behavior of "items". Omitting this keyword has the same assertion
    /// behavior as an empty schema.
    ///
    /// https://json-schema.org/draft/2020-12/json-schema-core.html#rfc.section.11.2
    /// - 2020-12
    /// - 2019-09
    pub unevaluated_items: Option<Box<SchemaDef>>,
    /// `unevaluatedProperties`
    /// - 2020-12
    /// - 2019-09
    pub unevaluated_properties: Option<BTreeMap<String, SchemaDef>>,
    //
    // -------------------------------------------
    //                validation
    // -------------------------------------------
    //
    /// `type`
    /// - 2020-12
    /// - 2019-09
    pub type_: Option<StringOrStrings>,
    /// `const`
    /// - 2020-12
    /// - 2019-09
    pub const_: Option<Value>,
    /// `enum`
    /// - 2020-12
    /// - 2019-09
    pub enum_: Option<Vec<Value>>,
    /// `multipleOf`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub multiple_of: Option<Number>,
    /// `maximum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub maximum: Option<Number>,
    /// `exclusiveMaximum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub exclusive_maximum: Option<Number>,
    /// `minimum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub minimum: Option<Number>,
    /// `exclusiveMinimum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub exclusive_minimum: Option<Number>,
    /// `maxLength`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub max_length: Option<Number>,
    /// `minLength`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub min_length: Option<Number>,
    /// `pattern`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub pattern: Option<String>,
    /// `maxItems`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub max_items: Option<Number>,
    /// `minItems`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub min_items: Option<Number>,
    /// `uniqueItems`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `true`
    pub unique_items: Option<bool>,
    /// `maxContains`
    /// - 2020-12
    /// - 2019-09
    pub max_contains: Option<Number>,
    /// `minContains`
    /// - 2020-12
    /// - 2019-09
    pub min_contains: Option<Number>,
    /// `maxProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + minimum: `0`
    pub max_properties: Option<Number>,
    /// `minProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub min_properties: Option<Number>,
    /// `required`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub required: Option<Vec<String>>,
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
    /// - 2020-12
    /// - 2019-09
    pub dependent_required: Option<BTreeMap<String, Vec<String>>>,
    //
    // -------------------------------------------
    //                meta-data
    // -------------------------------------------
    //
    /// `title`
    /// - 2020-12
    /// - 2019-09
    pub title: Option<String>,
    /// `description`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub description: Option<String>,
    /// `default`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub default: Option<Value>,
    /// `deprecated`
    /// - 2020-12
    /// - 2019-09
    pub deprecated: Option<bool>,
    /// `readOnly`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub read_only: Option<bool>,
    /// `writeOnly`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub write_only: Option<bool>,
    /// `examples`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub examples: Option<Vec<Value>>,
    //
    // -------------------------------------------
    //                 content
    // -------------------------------------------
    //
    /// `contentEncoding`
    /// - 2020-12
    /// - 2019-09
    pub content_encoding: Option<String>,
    /// `contentMediaType`
    /// - 2020-12
    /// - 2019-09
    pub content_media_type: Option<String>,
    /// `contentSchema`
    /// - 2020-12
    /// - 2019-09
    pub content_schema: Option<Box<SchemaDef>>,
    //
    // -------------------------------------------
    //            format-annotation
    // -------------------------------------------
    //
    /// `format`
    /// - 2020-12
    /// - 2019-09
    pub format: Option<String>,
    // -------------------------------------------
    //                 deprecated
    // -------------------------------------------
    /// `dependencies`
    /// - 07
    /// - 04
    ///
    /// Draft 2019-09 split `dependencies` into `dependentSchemas` and
    /// `dependentRequired`
    pub dependencies: Option<BTreeMap<String, Dependency>>,
    /// `$recursiveRef`
    /// - 2019-09
    pub recursive_ref: Option<String>,
    /// `$recursiveAnchor`
    /// - 2019-09
    pub recursive_anchor: Option<bool>,
    /// `id`
    /// - 04
    ///
    /// if `$schema` is draft 04, then `id` will use this value.
    /// Otherwise, the value of this field should be placed in
    /// `unknown` with the key `id`.
    old_id: Option<String>,
    /// `definitions`
    /// - 07
    /// - 04
    /// + warn about deprecation in schemas greater than 07
    /// + combine with `$defs`
    definitions: Option<BTreeMap<String, SchemaDef>>,

    //
    // -------------------------------------------
    //             computed values
    // -------------------------------------------
    //
    /// if the schema was a bool, assign the value here and use the long form
    /// representation.
    /// ```
    /// true => {}
    /// ```
    /// ```
    /// false => { not: {} }
    /// ```
    always: Option<bool>,
    /// computed value for `$schema`
    draft: Option<Draft>,
}
