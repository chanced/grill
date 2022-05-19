#[derive(Debug)]
pub struct Schema {
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
    pub schema: String,
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
    pub defs: Option<BTreeMap<String, Schema>>,

    //
    // -------------------------------------------
    //                applicator
    // -------------------------------------------
    //
    /// ### `prefixItems`
    /// - 2020-12
    pub prefix_items: Option<Vec<Schema>>,
    /// ### `additionalItems`
    /// - 2019-09
    /// - 07
    pub additional_items: Option<Box<Schema>>,
    /// ### `items`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub items: Option<Box<Schema>>,
    /// ### `contains`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub contains: Option<Box<Schema>>,
    /// ### `additionalProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub additional_properties: Option<Box<Schema>>,
    /// ### `properties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `{}`
    pub properties: Option<BTreeMap<String, Schema>>,
    /// ### `patternProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `{}`
    pub pattern_properties: Option<BTreeMap<String, Schema>>,
    /// ### `dependentSchemas`
    /// - 2020-12
    /// - 2019-09
    pub dependent_schemas: Option<BTreeMap<String, Schema>>,
    /// ### `propertyNames`
    /// - 2020-12
    /// - 2019-09
    pub property_names: Option<Box<Schema>>,
    /// ### `if`
    /// - 2020-12
    /// - 2019-09
    pub if_: Option<Box<Schema>>,
    /// ### `then`
    /// - 2020-12
    /// - 2019-09
    pub then: Option<Box<Schema>>,
    /// ### `else`
    /// - 2020-12
    /// - 2019-09
    pub else_: Option<Box<Schema>>,
    /// ### `allOf`
    /// - 2020-12
    /// - 2019-09
    pub all_of: Option<Vec<Schema>>,
    /// ### `anyOf`
    /// - 2020-12
    /// - 2019-09
    pub any_of: Option<Vec<Schema>>,
    /// ### `oneOf`
    /// - 2020-12
    /// - 2019-09
    pub one_of: Option<Vec<Schema>>,
    /// ### `not`
    /// - 2020-12
    /// - 2019-09
    pub not: Option<Box<Schema>>,
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
    pub unevaluated_items: Option<Box<Schema>>,
    /// ### `unevaluatedProperties`
    /// - 2020-12
    /// - 2019-09
    pub unevaluated_properties: Option<BTreeMap<String, Schema>>,
    //
    // -------------------------------------------
    //                validation
    // -------------------------------------------
    //
    /// ### `type`
    /// - 2020-12
    /// - 2019-09
    pub type_: Option<StringOrStrings>,
    /// ### `const`
    /// - 2020-12
    /// - 2019-09
    pub const_: Option<Value>,
    /// ### `enum`
    /// - 2020-12
    /// - 2019-09
    pub enum_: Option<Vec<Value>>,
    /// ### `multipleOf`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub multiple_of: Option<Number>,
    /// ### `maximum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub maximum: Option<Number>,
    /// ### `exclusiveMaximum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub exclusive_maximum: Option<Number>,
    /// ### `minimum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub minimum: Option<Number>,
    /// ### `exclusiveMinimum`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub exclusive_minimum: Option<Number>,
    /// ### `maxLength`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub max_length: Option<Number>,
    /// ### `minLength`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub min_length: Option<Number>,
    /// ### `pattern`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub pattern: Option<String>,
    /// ### `maxItems`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub max_items: Option<Number>,
    /// ### `minItems`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub min_items: Option<Number>,
    /// ### `uniqueItems`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `true`
    pub unique_items: Option<bool>,
    /// ### `maxContains`
    /// - 2020-12
    /// - 2019-09
    pub max_contains: Option<Number>,
    /// ### `minContains`
    /// - 2020-12
    /// - 2019-09
    pub min_contains: Option<Number>,
    /// ### `maxProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + minimum: `0`
    pub max_properties: Option<Number>,
    /// ### `minProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub min_properties: Option<Number>,
    /// ### `required`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub required: Option<Vec<String>>,
    /// ### `dependentRequired`
    ///
    /// The `dependentRequired` keyword conditionally requires that certain
    /// ### `properties` must be present if a given property is present in an
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
    /// ### `title`
    /// - 2020-12
    /// - 2019-09
    pub title: Option<String>,
    /// ### `description`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub description: Option<String>,
    /// ### `default`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub default: Option<Value>,
    /// ### `deprecated`
    /// - 2020-12
    /// - 2019-09
    pub deprecated: Option<bool>,
    /// ### `readOnly`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub read_only: Option<bool>,
    /// ### `writeOnly`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub write_only: Option<bool>,
    /// ### `examples`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub examples: Option<Vec<Value>>,
    //
    // -------------------------------------------
    //                 content
    // -------------------------------------------
    //
    /// ### `contentEncoding`
    /// - 2020-12
    /// - 2019-09
    pub content_encoding: Option<String>,
    /// ### `contentMediaType`
    /// - 2020-12
    /// - 2019-09
    pub content_media_type: Option<String>,
    /// ### `contentSchema`
    /// - 2020-12
    /// - 2019-09
    pub content_schema: Option<Box<Schema>>,
    //
    // -------------------------------------------
    //            format-annotation
    // -------------------------------------------
    //
    /// ### `format`
    /// - 2020-12
    /// - 2019-09
    pub format: Option<String>,
    // -------------------------------------------
    //                 deprecated
    // -------------------------------------------
    /// ### `dependencies`
    /// - 07
    /// - 04
    ///
    /// Draft 2019-09 split `dependencies` into `dependentSchemas` and
    /// `dependentRequired`
    pub dependencies: Option<BTreeMap<String, Dependency>>,
    /// ### `$recursiveRef`
    /// - 2019-09
    /// /// **deprecated in Draft 2020-12**
    pub recursive_ref: Option<String>,
    /// ### `$recursiveAnchor`
    /// - 2019-09
    /// **deprecated in Draft 2020-12**
    pub recursive_anchor: Option<bool>,
    /// ### `definitions`
    /// - 07
    /// - 04
    ///
    /// **Deprecated in Draft 2019-09**
    pub definitions: Option<BTreeMap<String, Schema>>,

    // -------------------------------------------
    //                computed
    // -------------------------------------------
    /// always is set if the schema is either `true` or `false`
    pub always: Option<bool>,
    /// ### `id`
    /// Draft `id`. If `id` is found and the schema
    /// version is Draft-07 and above, the `id` will be placed in the `unknown`
    /// map.
    pub old_id: Option<String>,
    /// unknown fields
    pub unknown: Map<String, Value>,

    src: Value,
    imports: Vec<String>,
}

impl Schema {
    pub fn new(src: impl Into<Value>) -> Result<Self, Error> {
        // TODO: Validate Schema

        Ok(Self {
            id: None,
            schema: String::new(),
            ref_: None,
            anchor: None,
            dynamic_ref: None,
            dynamic_anchor: None,
            vocabulary: None,
            comment: None,
            defs: None,
            prefix_items: None,
            additional_items: None,
            items: None,
            contains: None,
            additional_properties: None,
            properties: None,
            pattern_properties: None,
            dependent_schemas: None,
            property_names: None,
            if_: None,
            then: None,
            else_: None,
            all_of: None,
            any_of: None,
            one_of: None,
            not: None,
            unevaluated_items: None,
            unevaluated_properties: None,
            type_: None,
            const_: None,
            enum_: None,
            multiple_of: None,
            maximum: None,
            exclusive_maximum: None,
            minimum: None,
            exclusive_minimum: None,
            max_length: None,
            min_length: None,
            pattern: None,
            max_items: None,
            min_items: None,
            unique_items: None,
            max_contains: None,
            min_contains: None,
            max_properties: None,
            min_properties: None,
            required: None,
            dependent_required: None,
            title: None,
            description: None,
            default: None,
            deprecated: None,
            read_only: None,
            write_only: None,
            examples: None,
            content_encoding: None,
            content_media_type: None,
            content_schema: None,
            format: None,
            dependencies: None,
            recursive_ref: None,
            recursive_anchor: None,
            definitions: None,
            always: None,
            old_id: None,
            unknown: Map::new(),
            src: src.into(),
            imports: Vec::new(),
        })
    }
}

struct SchemaVisitor {}

impl<'de> Visitor<'de> for SchemaVisitor {
    type Value = Schema;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("json schema")
    }

    fn visit_map<A>(self, access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        todo!()

        // let m = Map::new();
        // let mut obj = Schema::new(access.size_hint().unwrap_or(0));

        // let mut old_id: Option<String> = None;
        // let insert = |k: &str, v| obj.src.insert(k.to_string(), v);
        // while let Some(key) = access.next_key()? {
        //     match key {
        //         "$id" => {
        //             let val = access.next_value()?;
        //             obj.id = Some(val);
        //             insert(key, Value::String(val));
        //         }
        //         "id" => {
        //             let val = access.next_value()?;
        //             old_id = Some(access.next_value()?);
        //             obj.src.insert(key.to_string(), Value::from(val));
        //         }
        //         "$schema" => {
        //             let val: String = access.next_value()?;
        //             obj.schema = Some(val.clone());
        //             insert(key, Value::from(val));
        //         }
        //         "$ref" => {
        //             let val = access.next_value()?;
        //             obj.ref_ = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$anchor" => {
        //             let val = access.next_value()?;
        //             obj.anchor = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$dynamicRef" => {
        //             let val = access.next_value()?;
        //             obj.dynamic_ref = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$dynamicAnchor" => {
        //             let val = access.next_value()?;
        //             obj.dynamic_anchor = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$vocabulary" => {
        //             let val = access.next_value()?;
        //             obj.vocabulary = Some(val);
        //             insert(key, Value::from(val.clone()));
        //         }
        //         "$comment" => {
        //             let val = access.next_value()?;
        //             obj.comment = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$defs" => {
        //             let val = access.next_value()?;
        //             obj.defs = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "prefixItems" => {
        //             let val = access.next_value()?;
        //             obj.prefix_items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "additionalItems" => {
        //             let val = access.next_value()?;
        //             obj.additional_items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "items" => {
        //             let val = access.next_value()?;
        //             obj.items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "contains" => {
        //             let val = access.next_value()?;
        //             obj.contains = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "additionalProperties" => {
        //             let val = access.next_value()?;
        //             obj.additional_properties = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "properties" => {
        //             let val = access.next_value()?;
        //             obj.properties = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "patternProperties" => {
        //             let val = access.next_value()?;
        //             obj.pattern_properties = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "dependentSchemas" => {
        //             let val = access.next_value()?;
        //             obj.dependent_schemas = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "propertyNames" => {
        //             let val = access.next_value()?;
        //             obj.property_names = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "if" => {
        //             let val = access.next_value()?;
        //             obj.if_ = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "then" => {
        //             let val = access.next_value()?;
        //             obj.then = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "else" => {
        //             let val = access.next_value()?;
        //             obj.else_ = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "allOf" => {
        //             let val = access.next_value()?;
        //             obj.all_of = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "anyOf" => {
        //             let val = access.next_value()?;
        //             obj.any_of = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "oneOf" => {
        //             let val = access.next_value()?;
        //             obj.one_of = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "not" => {
        //             let val = access.next_value()?;
        //             obj.not = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "unevaluatedItems" => {
        //             let val = access.next_value()?;
        //             obj.unevaluated_items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "unevaluatedProperties" => {
        //             let val = access.next_value()?;
        //             obj.unevaluated_properties = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "type" => {
        //             let val = access.next_value()?;
        //             obj.type_ = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "const" => {
        //             let val = access.next_value()?;
        //             obj.const_ = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "enum" => {
        //             let val = access.next_value()?;
        //             obj.enum_ = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "multipleOf" => {
        //             let val = access.next_value()?;
        //             obj.multiple_of = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "maximum" => {
        //             let val = access.next_value()?;
        //             obj.maximum = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "exclusiveMaximum" => {
        //             let val = access.next_value()?;
        //             obj.exclusive_maximum = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "minimum" => {
        //             let val = access.next_value()?;
        //             obj.minimum = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "exclusiveMinimum" => {
        //             let val = access.next_value()?;
        //             obj.exclusive_minimum = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "maxLength" => {
        //             let val = access.next_value()?;
        //             obj.max_length = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "minLength" => {
        //             let val = access.next_value()?;
        //             obj.min_length = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "pattern" => {
        //             let val = access.next_value()?;
        //             obj.pattern = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "maxItems" => {
        //             let val = access.next_value()?;
        //             obj.max_items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "minItems" => {
        //             let val = access.next_value()?;
        //             obj.min_items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "uniqueItems" => {
        //             let val = access.next_value()?;
        //             obj.unique_items = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "maxContains" => {
        //             let val = access.next_value()?;
        //             obj.max_contains = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "minContains" => {
        //             let val = access.next_value()?;
        //             obj.min_contains = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "maxProperties" => {
        //             let val = access.next_value()?;
        //             obj.max_properties = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "minProperties" => {
        //             let val = access.next_value()?;
        //             obj.min_properties = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "required" => {
        //             let val = access.next_value()?;
        //             obj.required = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "dependentRequired" => {
        //             let val = access.next_value()?;
        //             obj.dependent_required = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "title" => {
        //             let val = access.next_value()?;
        //             obj.title = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "description" => {
        //             let val = access.next_value()?;
        //             obj.description = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "default" => {
        //             let val = access.next_value()?;
        //             obj.default = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "deprecated" => {
        //             let val = access.next_value()?;
        //             obj.deprecated = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "readOnly" => {
        //             let val = access.next_value()?;
        //             obj.read_only = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "writeOnly" => {
        //             let val = access.next_value()?;
        //             obj.write_only = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "examples" => {
        //             let val = access.next_value()?;
        //             obj.examples = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "contentEncoding" => {
        //             let val = access.next_value()?;
        //             obj.content_encoding = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "contentMediaType" => {
        //             let val = access.next_value()?;
        //             obj.content_media_type = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "contentSchema" => {
        //             let val = access.next_value()?;
        //             obj.content_schema = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "format" => {
        //             let val = access.next_value()?;
        //             obj.format = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "dependencies" => {
        //             let val = access.next_value()?;
        //             obj.dependencies = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$recursiveRef" => {
        //             let val = access.next_value()?;
        //             obj.recursive_ref = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "$recursiveAnchor" => {
        //             let val = access.next_value()?;
        //             obj.recursive_anchor = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //         "definitions" => {
        //             let val = access.next_value()?;
        //             obj.definitions = Some(val);
        //             insert(key, Value::from(val));
        //         }
        //     }
    }
    // obj.imports
    // }
}
impl<'de> Deserialize<'de> for Schema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(SchemaVisitor {})
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

pub struct HyperSchema {
    // -------------------------------------------
    //                 hyper
    // -------------------------------------------
    base: Option<String>,
    links: Option<Vec<Link>>,
    // -------------------------------------------
    //                schema
    // -------------------------------------------

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
    pub defs: Option<BTreeMap<String, Schema>>,

    //
    // -------------------------------------------
    //                applicator
    // -------------------------------------------
    //
    /// `prefixItems`
    /// - 2020-12
    pub prefix_items: Option<Vec<Schema>>,
    /// `additionalItems`
    /// - 2019-09
    /// - 07
    pub additional_items: Option<Box<Schema>>,
    /// `items`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub items: Option<Box<Schema>>,
    /// `contains`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub contains: Option<Box<Schema>>,
    /// `additionalProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    pub additional_properties: Option<Box<Schema>>,
    /// `properties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `{}`
    pub properties: Option<BTreeMap<String, Schema>>,
    /// `patternProperties`
    /// - 2020-12
    /// - 2019-09
    /// - 07
    /// + default: `{}`
    pub pattern_properties: Option<BTreeMap<String, Schema>>,
    /// `dependentSchemas`
    /// - 2020-12
    /// - 2019-09
    pub dependent_schemas: Option<BTreeMap<String, Schema>>,
    /// `propertyNames`
    /// - 2020-12
    /// - 2019-09
    pub property_names: Option<Box<Schema>>,
    /// `if`
    /// - 2020-12
    /// - 2019-09
    pub if_: Option<Box<Schema>>,
    /// `then`
    /// - 2020-12
    /// - 2019-09
    pub then: Option<Box<Schema>>,
    /// `else`
    /// - 2020-12
    /// - 2019-09
    pub else_: Option<Box<Schema>>,
    /// `allOf`
    /// - 2020-12
    /// - 2019-09
    pub all_of: Option<Vec<Schema>>,
    /// `anyOf`
    /// - 2020-12
    /// - 2019-09
    pub any_of: Option<Vec<Schema>>,
    /// `oneOf`
    /// - 2020-12
    /// - 2019-09
    pub one_of: Option<Vec<Schema>>,
    /// `not`
    /// - 2020-12
    /// - 2019-09
    pub not: Option<Box<Schema>>,
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
    pub unevaluated_items: Option<Box<Schema>>,
    /// `unevaluatedProperties`
    /// - 2020-12
    /// - 2019-09
    pub unevaluated_properties: Option<BTreeMap<String, Schema>>,
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
    pub content_schema: Option<Box<Schema>>,
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
    /// for 2019+, split to `dependentSchemas` and `dependentRequired`
    dependencies: BTreeMap<String, Dependency>,
    /// `$recursiveRef`
    /// - 2019-09
    recursive_ref: Option<String>,
    /// `$recursiveAnchor`
    /// - 2019-09
    recursive_anchor: Option<String>,
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
    definitions: Option<BTreeMap<String, Schema>>,
}

use crate::{Error, InvalidType};
use serde::de::Visitor;
pub use serde::{Deserialize, Serialize};
pub const SIMPLE_TYPES: &[&str] = &["string", "boolean", "object", "array", "number", "null"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum TypeOrTypes {
    Type(Type),
    Types(Vec<Type>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Array => "array".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Integer => "integer".to_string(),
            Type::Number => "number".to_string(),
            Type::Null => "null".to_string(),
            Type::Object => "object".to_string(),
            Type::String => "string".to_string(),
        }
    }
}

impl FromStr for Type {
    type Err = InvalidType;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "array" => Ok(Type::Array),
            "boolean" => Ok(Type::Boolean),
            "integer" => Ok(Type::Integer),
            "number" => Ok(Type::Number),
            "null" => Ok(Type::Null),
            "object" => Ok(Type::Object),
            "string" => Ok(Type::String),
            _ => Err(InvalidType(s.to_string())),
        }
    }
}

impl Type {
    pub fn type_names() -> &'static [&'static str] {
        SIMPLE_TYPES
    }
}

pub(crate) struct TypeVisitor {}

impl<'de> Visitor<'de> for TypeVisitor {
    type Value = TypeOrTypes;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a json schema type (string) or array of types (string[])")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Type::from_str(v)
            .map(TypeOrTypes::Type)
            .map_err(|_| serde::de::Error::unknown_variant(v, Type::type_names()))
    }
}
