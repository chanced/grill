use std::{
    collections::{HashMap, HashSet},
};

use super::{CompiledBoolOrNumber, Format, Items, SchemaRef, Types};
use crate::{
    error::EvaluateError, output::Annotation, AbsoluteUri, Compile, Handler, Keyword,
    Scope, Uri,
};
use fancy_regex::Regex;
use num_rational::BigRational;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct CompiledSchema {
    absolute_location: Uri,
    /// $schema, inherrited or defaulted
    meta_schema: AbsoluteUri,
    handlers: Box<[Handler]>,
    subschemas: HashMap<String, SchemaRef>,
    subschema_lists: HashMap<String, Vec<SchemaRef>>,
    subschema_maps: HashMap<String, HashMap<String, SchemaRef>>,
    numbers: HashMap<String, BigRational>,
    regexes: HashMap<String, Regex>,

    /// In the event that the schema is a bool or empty obj, this should be set.
    always: Option<bool>,

    id: Option<String>,
    schema: Option<String>,
    comment: Option<String>,
    vocabulary: HashMap<String, bool>,
    anchor: Option<String>,
    dynamic_anchor: Option<String>,
    dynamic_reference: Option<String>,
    reference: Option<SchemaRef>,
    recursive_anchor: Option<bool>,
    recursive_reference: Option<SchemaRef>,
    types: Option<Types>,
    format: Option<Format>,
    constant: Option<serde_json::Value>,
    definitions: HashMap<String, SchemaRef>,
    definitions_legacy: HashMap<String, SchemaRef>,
    all_of: Vec<SchemaRef>,
    any_of: Vec<SchemaRef>,
    one_of: Vec<SchemaRef>,
    not: Option<SchemaRef>,
    cond_if: Option<SchemaRef>,
    cond_then: Option<SchemaRef>,
    cond_else: Option<SchemaRef>,
    dependent_schemas: HashMap<String, SchemaRef>,
    prefix_items: Vec<SchemaRef>,
    items: Option<Items>,
    additional_items: Option<SchemaRef>,
    contains: Option<SchemaRef>,
    properties: HashMap<String, SchemaRef>,
    pattern_properties: HashMap<String, Regex>,
    additional_properties: Option<SchemaRef>,
    property_names: Option<SchemaRef>,
    unevaluated_items: Option<SchemaRef>,
    unevaluated_properties: Option<SchemaRef>,
    enumeration: Vec<Value>,
    multiple_of: Option<BigRational>,
    maximum: Option<BigRational>,
    exclusive_maximum: Option<CompiledBoolOrNumber>,
    minimum: Option<BigRational>,
    exclusive_minimum: Option<CompiledBoolOrNumber>,
    max_length: Option<usize>,
    min_length: Option<usize>,
    pattern: Option<Regex>,
    max_items: Option<usize>,
    min_items: Option<usize>,
    unique_items: Option<bool>,
    max_contains: Option<usize>,
    min_contains: Option<usize>,
    max_properties: Option<usize>,
    min_properties: Option<usize>,
    required: HashSet<String>,
    dependent_required: HashMap<String, HashSet<String>>,
    content_encoding: Option<String>,
    content_media_type: Option<String>,
    content_schema: Option<SchemaRef>,
    title: Option<String>,
    description: Option<String>,
    deprecated: Option<bool>,
    read_only: Option<bool>,
    write_only: Option<bool>,
    examples: Option<Vec<Value>>,
    additional_keywords: HashMap<String, Value>,
}

impl CompiledSchema {
    #[must_use]
    pub fn new(_c: Compile) -> Self {
        todo!()
        // let obj = schema
        //     .as_object()
        //     .map(|v| Cow::Borrowed(v))
        //     .unwrap_or_default();
        // Self {
        //     absolute_location,
        //     meta_schema,
        //     handlers,
        //     anchor: obj.anchor.clone(),
        //     constant: obj.constant.clone(),
        //     comment: obj.comment.clone(),
        //     content_encoding: obj.content_encoding.clone(),
        //     content_media_type: obj.content_media_type.clone(),
        //     description: obj.description.clone(),
        //     dynamic_anchor: obj.dynamic_anchor.clone(),
        //     dynamic_reference: obj.dynamic_reference.clone(),
        //     enumeration: obj.enumeration.clone(),
        //     deprecated: obj.deprecated,
        //     examples: obj.examples.clone(),
        //     exclusive_maximum: obj
        //         .exclusive_maximum
        //         .as_ref()
        //         .and_then(|v| v.as_bool().map(|v| CompiledBoolOrNumber::Bool(*v))),
        //     exclusive_minimum: obj
        //         .exclusive_minimum
        //         .as_ref()
        //         .and_then(|v| v.as_bool().map(|v| CompiledBoolOrNumber::Bool(*v))),
        //     format: obj.format.clone(),
        //     id: obj.id.clone(),
        //     max_contains: obj.max_contains,
        //     max_items: obj.max_items,
        //     max_length: obj.max_length,
        //     max_properties: obj.max_properties,
        //     min_contains: obj.min_contains,
        //     min_items: obj.min_items,
        //     min_length: obj.min_length,
        //     min_properties: obj.min_properties,
        //     read_only: obj.read_only,
        //     write_only: obj.write_only,
        //     recursive_anchor: obj.recursive_anchor,
        //     schema: obj.schema.clone(),
        //     title: obj.title.clone(),
        //     types: obj.types.clone(),
        //     vocabulary: obj
        //         .vocabulary
        //         .iter()
        //         .map(|(k, v)| (k.clone(), v.clone()))
        //         .collect(),
        //     unique_items: obj.unique_items.clone(),
        //     ..Default::default()
        // }
    }

    // /// # Errors
    // #[allow(clippy::missing_panics_doc)]
    // pub async fn evaluate<'v>(
    //     &self,
    //     value: &'v Value,
    //     structure: Structure,
    // ) -> Result<Output<'v>, Box<dyn std::error::Error>> {
    //     let mut state = State::new();
    //     let location = Location {
    //         absolute_keyword_location: self.absolute_location.clone(),
    //         keyword_location: Pointer::default(),
    //         instance_location: Pointer::default(),
    //     };
    //     let mut scope = Scope::new(location, &mut state);
    //     let annotation = self.annotate("", "", &mut scope, value, structure).await?;
    //     Ok(Output::new(structure, annotation))
    // }

    // #[must_use]
    // pub fn schema(&self) -> &Schema {
    //     self.schema
    // }

    // #[must_use]
    // pub fn number<'a>(&'a self, keyword: &'static str) -> Option<&'a BigRational> {
    //     self.numbers.get(keyword)
    // }

    // #[must_use]
    // pub fn subschema<'a>(&'a self, keyword: &'static str) -> Option<CompiledSubschema<'a>> {
    //     self.schemas
    //         .get(keyword)
    //         .map(|schema| CompiledSubschema { keyword, schema })
    // }

    // /// # Errors
    // /// if a custom [`Handler`](`crate::Handler`) returns a [`Box<dyn Error`](`std::error::Error`)
    // async fn annotate<'v, 'a>(
    //     &self,
    //     instance_location: &'v str,
    //     keyword_location: &'s str,
    //     scope: &'s mut Scope<'a>,
    //     value: &'v Value,
    //     structure: Structure,
    // ) -> Result<Annotation<'v>, Box<dyn std::error::Error>> {
    //     let annotation = Annotate {
    //         absolute_keyword_location: &self.absolute_location,
    //         instance_location,
    //         keyword_location,
    //         scope,
    //         structure,
    //         value,
    //         schema: self,
    //     }
    //     .exec()
    //     .await?;
    //     Ok(annotation)
    // }

    // /// ##`$recursiveRef` The `"$recursiveRef"` and `"$recursiveAnchor"`
    // /// keywords are used to construct extensible recursive schemas.  A
    // /// recursive schema is one that has a reference to its own root, identified
    // /// by the empty fragment URI reference `'#'`.
    // ///
    // /// - [Draft 2019-09 Core # 8.2.4.2.  Recursive References with `"$recursiveRef"`
    // ///   and
    // ///   `"$recursiveAnchor"`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2)
    // #[must_use]
    // #[inline]
    // pub fn recursive_reference(&self) -> Option<&str> {
    //     self.recursive_reference.as_deref()
    // }

    /// Absolute location of this `CompiledSchema`
    #[must_use]
    #[inline]
    pub fn absolute_location(&self) -> &str {
        &self.absolute_location
    }

    /// [`Handler`](`crate::Handler`)s associated with this `CompiledSchema`
    #[must_use]
    #[inline]
    pub fn handlers(&self) -> &[Handler] {
        &self.handlers
    }

    #[must_use]
    #[inline]
    pub fn subschema(&self, keyword: Keyword<'_>) -> Option<SchemaRef> {
        self.subschemas.get(&keyword.to_string()).cloned()
    }

    #[must_use]
    #[inline]
    pub fn additional_keywords(&self) -> &HashMap<String, Value> {
        &self.additional_keywords
    }

    #[must_use]
    #[inline]
    pub fn additional_regexes(&self) -> &HashMap<String, Regex> {
        &self.regexes
    }

    /// ## `$id`
    /// The value of `$id` is a URI-reference without a fragment that resolves
    /// against the Retrieval URI. The resulting URI is the base URI for the
    /// schema.
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.1. The `"$id"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword)
    /// - [Understanding JSON Schema # Structuring a complex schema: `$id`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=id#id)
    #[must_use]
    #[inline]
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// ## `$schema`
    /// The `$schema` keyword is both used as a JSON Schema dialect identifier
    /// and as the identifier of a resource which is itself a JSON Schema, which
    /// describes the set of valid schemas written for this particular dialect.
    ///
    /// - [JSON Schema Core 2020-12 # 8.1.1. The `"$schema"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)
    /// - [Draft 2019-09 Core # 8.1.1. The `"$schema"` Keyword](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.1.1)
    /// - [Draft 7 # 7. The `"$schema"` Keyword](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-7)
    #[must_use]
    #[inline]
    pub fn schema(&self) -> Option<&str> {
        self.schema.as_deref()
    }

    /// ## `$comment`
    /// The `"$comment"` keyword is strictly intended for adding comments to a
    /// schema. Its value must always be a string. Unlike the annotations title,
    /// description, and examples, JSON schema implementations aren’t allowed to
    /// attach any meaning or behavior to it whatsoever, and may even strip them
    /// at any time. Therefore, they are useful for leaving notes to future
    /// editors of a JSON schema, but should not be used to communicate to users
    /// of the schema.
    ///
    /// - [Understanding JSON Schema # Generic keywords:
    ///   Comments](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#comments)
    #[must_use]
    #[inline]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    /// ## `$vocabulary`
    /// The `"$vocabulary"` keyword is used in meta-schemas to identify the
    /// vocabularies available for use in schemas described by that meta-schema.
    /// It is also used to indicate whether each vocabulary is required or
    /// optional, in the sense that an implementation MUST understand the
    /// required vocabularies in order to successfully process the schema.
    /// Together, this information forms a dialect. Any vocabulary that is
    /// understood by the implementation MUST be processed in a manner
    /// consistent with the semantic definitions contained within the
    /// vocabulary.
    ///
    /// The value of this keyword MUST be an object. The property names in the
    /// object MUST be URIs (containing a scheme) and this URI MUST be
    /// normalized. Each URI that appears as a property name identifies a
    /// specific set of keywords and their semantics.
    ///
    /// The URI MAY be a URL, but the nature of the retrievable resource is
    /// currently undefined, and reserved for future use. Vocabulary authors MAY
    /// use the URL of the vocabulary specification, in a human-readable media
    /// type such as text/html or text/plain, as the vocabulary URI. Vocabulary
    /// documents may be added in forthcoming drafts. For now, identifying the
    /// keyword set is deemed sufficient as that, along with meta-schema
    /// validation, is how the current "vocabularies" work today. Any future
    /// vocabulary document format will be specified as a JSON document, so
    /// using text/html or other non-JSON formats in the meantime will not
    /// produce any future ambiguity.
    ///
    /// The values of the object properties MUST be booleans. If the value is
    /// true, then implementations that do not recognize the vocabulary MUST
    /// refuse to process any schemas that declare this meta-schema with
    /// "$schema". If the value is false, implementations that do not recognize
    /// the vocabulary SHOULD proceed with processing such schemas. The value
    /// has no impact if the implementation understands the vocabulary.
    ///
    /// Per 6.5, unrecognized keywords SHOULD be treated as annotations. This
    /// remains the case for keywords defined by unrecognized vocabularies. It
    /// is not currently possible to distinguish between unrecognized keywords
    /// that are defined in vocabularies from those that are not part of any
    /// vocabulary.
    ///
    /// The "$vocabulary" keyword SHOULD be used in the root schema of any
    /// schema document intended for use as a meta-schema. It MUST NOT appear in
    /// subschemas.
    ///
    /// The "$vocabulary" keyword MUST be ignored in schema documents that are
    /// not being processed as a meta-schema. This allows validating a
    /// meta-schema M against its own meta-schema M' without requiring the
    /// validator to understand the vocabularies declared by M.
    ///
    /// - [JSON Schema Core 2020-12 # 8.1.2. The `"$vocabulary"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.2)
    #[must_use]
    #[inline]
    pub fn vocabulary(&self) -> &HashMap<String, bool> {
        &self.vocabulary
    }

    /// ## `$anchor`
    /// Using JSON Pointer fragments requires knowledge of the structure of the
    /// schema. When writing schema documents with the intention to provide
    /// re-usable schemas, it may be preferable to use a plain name fragment
    /// that is not tied to any particular structural location. This allows a
    /// subschema to be relocated without requiring JSON Pointer references to
    /// be updated.
    ///
    /// The `"$anchor"` and `"$dynamicAnchor"` keywords are used to specify such
    /// fragments. They are identifier keywords that can only be used to create
    /// plain name fragments, rather than absolute URIs as seen with `"$id"`.
    ///
    /// The base URI to which the resulting fragment is appended is the
    /// canonical URI of the schema resource containing the `"$anchor"` or
    /// `"$dynamicAnchor"` in question. As discussed in the previous section,
    /// this is either the nearest `"$id"` in the same or parent schema object,
    /// or the base URI for the document as determined according to RFC 3986.
    ///
    /// Separately from the usual usage of URIs, `"$dynamicAnchor"` indicates
    /// that the fragment is an extension point when used with the
    /// `"$dynamicRef"` keyword. This low-level, advanced feature makes it
    /// easier to extend recursive schemas such as the meta-schemas, without
    /// imposing any particular semantics on that extension. See the section on
    /// `"$dynamicRef"` [(Section
    /// 8.2.3.2)](https://json-schema.org/draft/2020-12/json-schema-core.html#dynamic-ref)
    /// for details.
    ///
    /// In most cases, the normal fragment behavior both suffices and is more
    /// intuitive. Therefore it is RECOMMENDED that `"$anchor"` be used to
    /// create plain name fragments unless there is a clear need for
    /// `"$dynamicAnchor"`.
    ///
    /// If present, the value of this keyword MUST be a string and MUST start
    /// with a letter (`[A-Za-z]`) or underscore `'_'`, followed by any number
    /// of letters, digits (`[0-9]`), hyphens `'-'`, underscores `'_'`, and
    /// periods `'.'`. This matches the US-ASCII part of XML's `NCName` production
    /// xml-names. Note that the anchor string does not include the `'#'`
    /// character, as it is not a URI-reference. An `"$anchor": "foo"` becomes
    /// the fragment `"#foo"` when used in a URI. See below for full examples.
    ///
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
    /// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with `"$dynamicRef"`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
    /// - [Draft 2019-09 Core # 8.2.3. Defining location-independent identifiers with `"$anchor"`](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.2.3)
    /// - [Understanding JSON Schema # `$anchor`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=anchor#anchor)
    #[must_use]
    #[inline]
    pub fn anchor(&self) -> Option<&str> {
        self.anchor.as_deref()
    }

    /// ## `$dynamicAnchor`
    /// A `"$dynamicAnchor"` can be thought of like a normal $anchor except that
    /// it can be referenced across schemas rather than just in the schema where
    /// it was defined. You can think of the old `"$recursiveAnchor"` as working
    /// the same way except that it only allowed you to create one anchor per
    /// schema, it had to be at the root of the schema, and the anchor name is
    /// always empty.
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with `"$dynamicRef"`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
    /// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
    /// - [JSON Schema Core 2020-12 Release Notes # `$dynamicRef` and `$dynamicAnchor`](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
    #[must_use]
    #[inline]
    pub fn dynamic_anchor(&self) -> Option<&str> {
        self.dynamic_anchor.as_deref()
    }

    /// ## `$dynamicRef`
    /// The `"$dynamicRef"` keyword is an applicator that allows for deferring the
    /// full resolution until runtime, at which point it is resolved each time
    /// it is encountered while evaluating an instance.
    ///
    /// Together with `"$dynamicAnchor"`, `"$dynamicRef"` implements a cooperative
    /// extension mechanism that is primarily useful with recursive schemas
    /// (schemas that reference themselves). Both the extension point and the
    /// runtime-determined extension target are defined with `"$dynamicAnchor"`,
    /// and only exhibit runtime dynamic behavior when referenced with
    /// "$dynamicRef".
    ///
    /// The value of the `"$dynamicRef"` property MUST be a string which is a
    /// URI-Reference. Resolved against the current URI base, it produces the
    /// URI used as the starting point for runtime resolution. This initial
    /// resolution is safe to perform on schema load.
    ///
    /// - [Draft
    ///   2020-12 # 8.2.3.2. Dynamic References with `"$dynamicRef"`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
    /// - [JSON Schema Core 2020-12 Release Notes # `$dynamicRef` and `$dynamicAnchor`](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
    #[must_use]
    #[inline]
    pub fn dynamic_reference(&self) -> Option<&str> {
        self.dynamic_reference.as_deref()
    }

    /// ## `$ref`
    /// The `"$ref"` keyword is an applicator that is used to reference a
    /// statically identified schema. Its results are the results of the
    /// referenced schema. [CREF5]
    ///
    /// The value of the "$ref" keyword MUST be a string which is a
    /// URI-Reference. Resolved against the current URI base, it produces the URI
    /// of the schema to apply. This resolution is safe to perform on schema
    /// load, as the process of evaluating an instance cannot change how the
    /// reference resolves.
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.3.1. Direct References with `"$ref"`](https://json-schema.org/draft/2020-12/json-schema-core.html#ref)
    /// - [Understanding JSON Schema # Structuring a complex schema `$ref`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=ref#ref)
    #[must_use]
    #[inline]
    pub fn reference<'s>(&self, _scope: &Scope) -> Option<SchemaRef> {
        self.reference
    }

    /// ##`$recursiveAnchor` `"$recursiveAnchor"` is used to dynamically
    /// identify a base URI at runtime for `"$recursiveRef"` by marking where
    /// such a calculation can start, and where it stops.  This keyword MUST NOT
    /// affect the base URI of other keywords, unless they are explicitly
    /// defined to rely on it.
    ///
    /// - [Draft 2019-09 Core # 8.2.4.2.2.  Enabling Recursion with `"$recursiveAnchor"`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2.2)
    #[must_use]
    #[inline]
    pub fn recursive_anchor(&self) -> Option<bool> {
        self.recursive_anchor
    }

    /// ## `type`
    /// The `"type"` keyword is fundamental to JSON Schema. It specifies the
    /// data type for a schema.
    ///
    /// The type keyword may either be a string or an array:
    ///
    /// If it’s a string, it is the name of one of the basic types above.
    ///
    /// If it is an array, it must be an array of strings, where each string is
    /// the name of one of the basic types, and each element is unique. In this
    /// case, the JSON snippet is valid if it matches any of the given types.
    ///
    /// - [Understanding JSON Schema # Type-specific keywords](https://json-schema.org/understanding-json-schema/reference/type.html)
    #[must_use]
    #[inline]
    pub fn types(&self) -> Option<&Types> {
        self.types.as_ref()
    }

    /// The format keyword allows for basic semantic identification of certain
    /// kinds of string values that are commonly used. For example, because JSON
    /// doesn’t have a `DateTime` type, dates need to be encoded as strings.
    /// format allows the schema author to indicate that the string value should
    /// be interpreted as a date. By format is just an annotation and
    /// does not effect validation.
    ///
    /// - [JSON Schema Core 2020-12 # 7. Vocabularies for Semantic Content With `"format"`](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c)
    /// - [Understanding Json Schema # string Built-in Formats](https://json-schema.org/understanding-json-schema/reference/string.html#id7)
    /// - [OpenAPI 3.1 Specification # 4.2 Format](https://spec.openapis.org/oas/v3.1.0#format)
    #[must_use]
    #[inline]
    pub fn format(&self) -> Option<&Format> {
        self.format.as_ref()
    }

    /// ## `const`
    /// The `"const"` keyword is used to restrict a value to a single value.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.1.3. `const`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-const)
    /// - [Understanding JSON Schema - Constant values](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#constant-values)
    #[must_use]
    #[inline]
    pub fn constant(&self) -> Option<&serde_json::Value> {
        self.constant.as_ref()
    }

    /// ## `$defs`
    /// The "$defs" keyword reserves a location for schema authors to inline
    /// re-usable JSON Schemas into a more general schema. The keyword does not
    /// directly affect the validation result.
    ///
    /// This keyword's value MUST be an object. Each member value of this object
    /// MUST be a valid JSON Schema.
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.4. Schema Re-Use With `"$defs"`](https://json-schema.org/draft/2020-12/json-schema-core.html#defs)
    /// - [Understanding JSON Schema # `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
    #[must_use]
    #[inline]
    pub fn definitions(&self) -> &HashMap<String, SchemaRef> {
        &self.definitions
    }

    /// ## `definitions`
    /// Legacy from Draft 07. See [`definitions`](`Object::definitions`).
    ///
    /// ## Note
    /// If using JSON Schema 07, use this field instead of [`definitions`](`Object::definitions`).
    ///
    /// - [Understanding JSON Schema # `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
    #[must_use]
    #[inline]
    pub fn definitions_legacy(&self) -> &HashMap<String, SchemaRef> {
        &self.definitions_legacy
    }

    /// ## `allOf`
    /// The `"allOf"` keyword acts as an `AND` where each subschema must be
    /// valid
    ///
    /// - [Understanding JSON Schema # Schema Composition `allOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=allOf#allOf)
    #[must_use]
    #[inline]
    pub fn all_of(&self) -> &[SchemaRef] {
        &self.all_of
    }

    /// ## `anyOf`
    /// The `"anyOf"` keyword acts as an `OR` where at least one of the
    /// subschemas must be valid
    ///
    /// - [Understanding JSON Schema # Schema Composition `anyOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=anyof#anyOf)
    #[must_use]
    #[inline]
    pub fn any_of(&self) -> &[SchemaRef] {
        &self.any_of
    }

    /// ## `oneOf`
    /// The `"oneOf"` keyword acts as an `XOR` where exactly one of the
    /// subschemas must be valid
    ///
    /// - [Understanding JSON Schema # Schema Composition `oneOf`](https://json-schema.org/understanding-json-schema/reference/combining.html#oneof)
    #[must_use]
    #[inline]
    pub fn one_of(&self) -> &[SchemaRef] {
        &self.one_of
    }

    /// ## `not`
    /// The not keyword declares that an instance validates if it doesn’t
    /// validate against the given subschema.
    ///
    /// - [Understanding JSON Schema # Schema Composition `not`](https://json-schema.org/understanding-json-schema/reference/combining.html?#id8)
    #[must_use]
    #[inline]
    pub fn not(&self) -> Option<SchemaRef> {
        self.not
    }

    /// ## `if`
    /// This validation outcome of this keyword's subschema has no direct effect
    /// on the overall validation result. Rather, it controls which of the
    /// `"then"` or `"else"` keywords are evaluated. Instances that successfully
    /// validate against this keyword's subschema MUST also be valid against the
    /// subschema value of the `"then"` keyword, if present.
    ///
    /// Instances that fail to validate against this keyword's subschema MUST
    /// also be valid against the subschema value of the `"else"` keyword, if
    /// present.
    ///
    /// - [JSON Schema Core 2020-12 # 10.2.2.1. `if`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.2.1)
    #[must_use]
    #[inline]
    pub fn cond_if<'s, 'v>(
        &self,
        _scope: &Scope<'s>,
        _value: &'v Value,
    ) -> Option<Result<Option<Annotation<'v>>, EvaluateError>> {
        // self.cond_if.as_ref()
        todo!()
    }

    /// ## `then`
    /// When `"if"` is present, and the instance successfully validates against
    /// its subschema, then validation succeeds against this keyword if the
    /// instance also successfully validates against this keyword's subschema.
    ///
    /// This keyword has no effect when `"if"` is absent, or when the instance
    /// fails to validate against its subschema. Implementations MUST NOT
    /// evaluate the instance against this keyword, for either validation or
    /// annotation collection purposes, in such cases.
    ///
    /// - [JSON Schema Core 2020-12 # 10.2.2.2. `then`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.2.2)
    #[must_use]
    #[inline]
    pub fn cond_then(&self) -> Option<SchemaRef> {
        self.cond_then
    }

    /// ## `else`
    /// When `"if"` is present, and the instance fails to validate against its
    /// subschema, then validation succeeds against this keyword if the instance
    /// successfully validates against this keyword's subschema.
    ///
    /// This keyword has no effect when `"if"` is absent, or when the instance
    /// successfully validates against its subschema. Implementations MUST NOT
    /// evaluate the instance against this keyword, for either validation or
    /// annotation collection purposes, in such cases.
    ///
    /// - [JSON Schema Core 2020-12 # 10.2.2.3. `else`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-else)
    #[must_use]
    #[inline]
    pub fn cond_else(&self) -> Option<SchemaRef> {
        self.cond_else()
    }

    /// ## `dependentSchemas`
    /// This keyword specifies subschemas that are evaluated if the instance is
    /// an object and contains a certain property.
    ///
    /// This keyword's value MUST be an object. Each value in the object MUST be
    /// a valid JSON Schema.
    ///
    /// If the object key is a property in the instance, the entire instance
    /// must validate against the subschema. Its use is dependent on the
    /// presence of the property.
    /// - [JSON Schema Core 2020-12 # 10.2.2.4. `dependentSchemas`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dependentschemas)
    #[must_use]
    #[inline]
    pub fn dependent_schemas(&self) -> &HashMap<String, SchemaRef> {
        &self.dependent_schemas
    }

    /// ## `prefixItems`
    /// Validation succeeds if each element of the instance validates against
    /// the schema at the same position, if any. This keyword does not constrain
    /// the length of the array. If the array is longer than this keyword's
    /// value, this keyword validates only the prefix of matching length.
    ///
    /// This keyword produces an annotation value which is the largest index to
    /// which this keyword applied a subschema. The value MAY be a boolean true
    /// if a subschema was applied to every index of the instance, such as is
    /// produced by the "items" keyword. This annotation affects the behavior of
    /// `"items"` and `"unevaluatedItems"`.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty array.
    ///
    /// - [JSON Schema Core 2020-12 # 10.3.1.1. `prefixItems`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-prefixitems)
    #[must_use]
    #[inline]
    pub fn prefix_items(&self) -> &[SchemaRef] {
        &self.prefix_items
    }

    /// ## `items`
    /// This keyword applies its subschema to all instance elements at indexes
    /// greater than the length of the "prefixItems" array in the same schema
    /// object, as reported by the annotation result of that "prefixItems"
    /// keyword. If no such annotation result exists, "items" applies its
    /// subschema to all instance array elements. Note that the behavior of
    /// "items" without "prefixItems" is identical to that of the schema form of
    /// "items" in prior drafts. When "prefixItems" is present, the behavior of
    /// "items" is identical to the former "additionalItems" keyword.
    ///
    /// ## For Draft 2019, 07:
    /// If "items" is a schema, validation succeeds if all elements in the
    /// array successfully validate against that schema.
    ///
    /// If "items" is an array of schemas, validation succeeds if each
    /// element of the instance validates against the schema at the same
    /// position, if any.

    /// - [JSON Schema Core 2020-12 # 10.3.1.2. `items`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-items)
    /// - [JSON Schema Validation 07 # 6.4.1 `items`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.4.1)
    #[must_use]
    #[inline]
    pub fn items(&self) -> Option<&Items> {
        self.items.as_ref()
    }

    /// - [JSON Schema Core 2019-09 # 9.3.1.2.  `additionalItems`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-9.3.1.2)
    #[must_use]
    #[inline]
    pub fn additional_items(&self) -> Option<SchemaRef> {
        self.additional_items
    }

    /// ## `contains`
    /// An array instance is valid against `"contains"` if at least one of its
    /// elements is valid against the given schema, except when "minContains" is
    /// present and has a value of `0`, in which case an array instance MUST be
    /// considered valid against the `"contains"` keyword, even if none of its
    /// elements is valid against the given schema.
    ///
    /// This keyword produces an annotation value which is an array of the
    /// indexes to which this keyword validates successfully when applying its
    /// subschema, in ascending order. The value MAY be a boolean "true" if the
    /// subschema validates successfully when applied to every index of the
    /// instance. The annotation MUST be present if the instance array to which
    /// this keyword's schema applies is empty.
    ///
    /// This annotation affects the behavior of `"unevaluatedItems"` in the
    /// Unevaluated vocabulary, and MAY also be used to implement the
    /// `"minContains"` and `"maxContains"` keywords in the Validation
    /// vocabulary.
    ///
    /// The subschema MUST be applied to every array element even after the
    /// first match has been found, in order to collect annotations for use by
    /// other keywords. This is to ensure that all possible annotations are
    /// collected.
    ///
    ///  - [JSON Schema Core 2020-12 # 10.3.1.3. `contains`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-contains)
    #[must_use]
    #[inline]
    pub fn contains(&self) -> Option<SchemaRef> {
        self.contains
    }

    /// ## `properties`
    /// Validation succeeds if, for each name that appears in both the instance
    /// and as a name within this keyword's value, the child instance for that
    /// name successfully validates against the corresponding schema.
    ///
    /// The annotation result of this keyword is the set of instance property
    /// names matched by this keyword. This annotation affects the behavior of
    /// "additionalProperties" (in this vocabulary) and "unevaluatedProperties"
    /// in the Unevaluated vocabulary.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// object.
    /// - [JSON Schema Core 2020-12 # 10.3.2.1. properties](https://json-schema.org/draft/2020-12/json-schema-core.html#name-properties)
    #[must_use]
    #[inline]
    pub fn properties(&self) -> &HashMap<String, SchemaRef> {
        &self.properties
    }

    /// ## `patternProperties`
    /// Each property name of this object SHOULD be a valid regular expression,
    /// according to the ECMA-262 regular expression dialect. Each property
    /// value of this object MUST be a valid JSON Schema.
    ///
    /// Validation succeeds if, for each instance name that matches any regular
    /// expressions that appear as a property name in this keyword's value, the
    /// child instance for that name successfully validates against each schema
    /// that corresponds to a matching regular expression.
    ///
    /// The annotation result of this keyword is the set of instance property
    /// names matched by this keyword. This annotation affects the behavior of
    /// "additionalProperties" (in this vocabulary) and "unevaluatedProperties"
    /// (in the Unevaluated vocabulary).
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// object.
    ///
    /// - [JSON Schema Core 2020-12 # 10.3.2.2. `patternProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-patternproperties)
    #[must_use]
    #[inline]
    pub fn pattern_properties(&self) -> &HashMap<String, Regex> {
        &self.pattern_properties
    }

    /// ## `additionalProperties`
    /// The value of "additionalProperties" MUST be a valid JSON Schema.
    ///
    /// The behavior of this keyword depends on the presence and annotation results
    /// of `"properties"` and `"patternProperties"` within the same schema object.
    /// Validation with `"additionalProperties"` applies only to the child values of
    /// instance names that do not appear in the annotation results of either
    /// `"properties"` or `"patternProperties"`.
    ///
    /// For all such properties, validation succeeds if the child instance validates
    /// against the `"additionalProperties"` schema.
    ///
    /// The annotation result of this keyword is the set of instance property names
    /// validated by this keyword's subschema. This annotation affects the behavior
    /// of `"unevaluatedProperties"` in the Unevaluated vocabulary.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty schema.
    ///
    /// - [JSON Schema Core 2020-12 # 10.3.2.3.`additionalProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-additionalproperties)
    #[must_use]
    #[inline]
    pub fn additional_properties(&self) -> Option<SchemaRef> {
        self.additional_properties
    }

    /// ## `propertyNames`
    /// If the instance is an object, this keyword validates if every property
    /// name in the instance validates against the provided schema. Note the
    /// property name that the schema is testing will always be a string.
    ///
    /// - [JSON Schema Core 2020-12 # 10.3.2.4.`propertyNames`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-propertynames)
    #[must_use]
    #[inline]
    pub fn property_names(&self) -> Option<SchemaRef> {
        self.property_names
    }

    /// ## `unevaluatedItems`
    /// The behavior of this keyword depends on the annotation results of
    /// adjacent keywords that apply to the instance location being validated.
    /// Specifically, the annotations from "prefixItems", "items", and
    /// "contains", which can come from those keywords when they are adjacent to
    /// the "unevaluatedItems" keyword. Those three annotations, as well as
    /// "unevaluatedItems", can also result from any and all adjacent in-place
    /// applicator (Section 10.2) keywords. This includes but is not limited to
    /// the in-place applicators defined in this document.
    ///
    /// If no relevant annotations are present, the "unevaluatedItems" subschema
    /// MUST be applied to all locations in the array. If a boolean true value
    /// is present from any of the relevant annotations, "unevaluatedItems" MUST
    /// be ignored. Otherwise, the subschema MUST be applied to any index
    /// greater than the largest annotation value for "prefixItems", which does
    /// not appear in any annotation value for "contains".
    ///
    /// This means that "prefixItems", "items", "contains", and all in-place
    /// applicators MUST be evaluated before this keyword can be evaluated.
    /// Authors of extension keywords MUST NOT define an in-place applicator
    /// that would need to be evaluated after this keyword.
    ///
    /// If the "unevaluatedItems" subschema is applied to any positions within
    /// the instance array, it produces an annotation result of boolean true,
    /// analogous to the behavior of "items". This annotation affects the
    /// behavior of "unevaluatedItems" in parent schemas.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty
    /// schema.
    ///
    /// - [JSON Schema Core 2020-12 # 11.2. `unevaluatedItems`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluateditems)
    #[must_use]
    #[inline]
    pub fn unevaluated_items(&self) -> Option<SchemaRef> {
        self.unevaluated_items
    }
    /// ## `unevaluatedProperties`
    /// The behavior of this keyword depends on the annotation results of adjacent
    /// keywords that apply to the instance location being validated. Specifically,
    /// the annotations from "properties", "patternProperties", and
    /// "additionalProperties", which can come from those keywords when they are
    /// adjacent to the "unevaluatedProperties" keyword. Those three annotations, as
    /// well as "unevaluatedProperties", can also result from any and all adjacent
    /// in-place applicator (Section 10.2) keywords. This includes but is not
    /// limited to the in-place applicators defined in this document.
    ///
    /// Validation with "unevaluatedProperties" applies only to the child values of
    /// instance names that do not appear in the "properties", "patternProperties",
    /// "additionalProperties", or "unevaluatedProperties" annotation results that
    /// apply to the instance location being validated.
    ///
    /// For all such properties, validation succeeds if the child instance validates
    /// against the "unevaluatedProperties" schema.
    ///
    /// This means that "properties", "patternProperties", "additionalProperties",
    /// and all in-place applicators MUST be evaluated before this keyword can be
    /// evaluated. Authors of extension keywords MUST NOT define an in-place
    /// applicator that would need to be evaluated after this keyword.
    ///
    /// The annotation result of this keyword is the set of instance property names
    /// validated by this keyword's subschema. This annotation affects the behavior
    /// of "unevaluatedProperties" in parent schemas.
    ///
    /// Omitting this keyword has the same assertion behavior as an empty schema.
    ///
    /// - [JSON Schema Core 2020-12 # 11.3. `unevaluatedProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties)
    #[must_use]
    #[inline]
    pub fn unevaluated_properties(&self) -> Option<SchemaRef> {
        self.unevaluated_properties
    }

    /// ## `enum`
    /// An instance validates successfully against this keyword if its value is
    /// equal to one of the elements in this keyword's array value.
    ///
    /// Elements in the array might be of any type, including null.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.1.2. `enum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-enum)
    #[must_use]
    #[inline]
    pub fn enumeration(&self) -> &[Value] {
        &self.enumeration
    }

    /// ## `multipleOf`
    /// The value of `"multipleOf"` MUST be a number, strictly greater than 0.
    ///
    /// A numeric instance is valid only if division by this keyword's value
    /// results in an integer.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.2.1. `multipleOf`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-multipleof)
    #[must_use]
    #[inline]
    pub fn multiple_of(&self) -> Option<&BigRational> {
        self.multiple_of.as_ref()
    }

    /// #`maximum`
    /// The value of `"maximum"` MUST be a number, representing an inclusive upper
    /// limit for a numeric instance.
    ///
    /// If the instance is a number, then this keyword validates only if the
    /// instance is less than or exactly equal to `"maximum"`.
    /// - [JSON Schema Validation 2020-12 # 6.2.2. `maximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maximum)
    #[must_use]
    #[inline]
    pub fn maximum(&self) -> Option<&BigRational> {
        self.maximum.as_ref()
    }

    /// ## `exclusiveMaximum`
    /// For JSON Schema drafts 7 and higher, the value of `"exclusiveMaximum"` MUST be a number, representing an
    /// exclusive upper limit for a numeric instance. For JSON Schema Draft 4, the value of `"exclusiveMaximum"` MUST
    /// be a boolean.
    ///
    ///
    /// If the instance is a number, then the instance is valid only if it has a
    /// value strictly less than (not equal to) `"exclusiveMaximum"`.
    /// - [JSON Schema Validation 2020-12 # 6.2.3. `exclusiveMaximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusivemaximum)
    #[must_use]
    #[inline]
    pub fn exclusive_maximum(&self) -> Option<&CompiledBoolOrNumber> {
        self.exclusive_maximum.as_ref()
    }

    /// ## `minimum`
    /// The value of `"minimum"` MUST be a number, representing an inclusive
    /// lower limit for a numeric instance.
    ///
    /// If the instance is a number, then this keyword validates only if the
    /// instance is greater than or exactly equal to `"minimum"`.
    /// - [JSON Schema Validation 2020-12 # 6.2.4. `minimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minimum)

    #[must_use]
    #[inline]
    pub fn minimum(&self) -> Option<&BigRational> {
        self.minimum.as_ref()
    }

    /// ## `exclusiveMinimum`
    ///
    /// For JSON Schema drafts 7 and higher, the value of `"exclusiveMinimum"` MUST be a number, representing an
    /// exclusive lower limit for a numeric instance.
    ///
    /// If the instance is a number, then the instance is valid only if it has a
    /// value strictly greater than (not equal to) `"exclusiveMinimum"`.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.2.5. `exclusiveMinimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusiveminimum)
    #[must_use]
    #[inline]
    pub fn exclusive_minimum(&self) -> Option<&CompiledBoolOrNumber> {
        self.exclusive_minimum.as_ref()
    }

    /// ## `maxLength`
    /// The value of `"maxLength"` MUST be a non-negative integer.
    ///
    /// A string instance is valid against this keyword if its length is less
    /// than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.1. `maxLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxlength)
    #[must_use]
    #[inline]
    pub fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    /// ## `minLength`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// A string instance is valid against this keyword if its length is greater
    /// than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.2. `minLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minlength)
    #[must_use]
    #[inline]
    pub fn min_length(&self) -> Option<usize> {
        self.min_length
    }

    /// ## `pattern`
    /// The value of this keyword MUST be a string. This string SHOULD be a
    /// valid regular expression, according to the ECMA-262 regular expression
    /// dialect.
    ///
    /// A string instance is considered valid if the regular expression matches
    /// the instance successfully.
    ///
    /// Regular expressions are not implicitly anchored.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.3. `pattern`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-pattern)
    #[must_use]
    #[inline]
    pub fn pattern(&self) -> Option<&Regex> {
        self.pattern.as_ref()
    }

    /// ## `maxItems`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An array instance is valid against "maxItems" if its size is less than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.3. `maxItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxitems)
    #[must_use]
    #[inline]
    pub fn max_items(&self) -> Option<usize> {
        self.max_items
    }

    /// ## `minItems`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An array instance is valid against "minItems" if its size is greater than,
    /// or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of 0.
    /// - [JSON Schema Validation 2020-12 # 6.3.3. `minItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minitems)
    #[must_use]
    #[inline]
    pub fn min_items(&self) -> Option<usize> {
        self.min_items
    }

    /// ## `uniqueItems`
    /// The value of this keyword MUST be a boolean.
    ///
    /// If this keyword has boolean value false, the instance validates
    /// successfully. If it has boolean value true, the instance validates
    /// successfully if all of its elements are unique.
    ///
    /// Omitting this keyword has the same behavior as a value of false.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.4.3. `uniqueItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-uniqueitems)
    #[must_use]
    #[inline]
    pub fn unique_items(&self) -> Option<bool> {
        self.unique_items
    }

    /// ## `maxContains`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// If `"contains"` is not present within the same schema object, then this
    /// keyword has no effect.
    ///
    /// An instance array is valid against "maxContains" in two ways, depending on
    /// the form of the annotation result of an adjacent "contains" [json-schema]
    /// keyword. The first way is if the annotation result is an array and the
    /// length of that array is less than or equal to the "maxContains" value. The
    /// second way is if the annotation result is a boolean "true" and the instance
    /// array length is less than or equal to the "maxContains" value.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.4.4. `maxContains`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxcontains)
    #[must_use]
    #[inline]
    pub fn max_contains(&self) -> Option<usize> {
        self.max_contains
    }

    /// ## `minContains`
    /// The value of this keyword MUST be a non-negative integer.

    /// If `"contains"` is not present within the same schema object, then this
    /// keyword has no effect.
    ///
    /// An instance array is valid against "minContains" in two ways, depending on
    /// the form of the annotation result of an adjacent "contains" [json-schema]
    /// keyword. The first way is if the annotation result is an array and the
    /// length of that array is greater than or equal to the "minContains" value.
    /// The second way is if the annotation result is a boolean "true" and the
    /// instance array length is greater than or equal to the "minContains" value.
    ///
    /// A value of `0` is allowed, but is only useful for setting a range of
    /// occurrences from 0 to the value of "maxContains". A value of 0 causes
    /// "minContains" and "contains" to always pass validation (but validation can
    /// still fail against a "maxContains" keyword).
    ///
    /// Omitting this keyword has the same behavior as a value of `1`.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.4.4. `minContains`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-mincontains)
    #[must_use]
    #[inline]
    pub fn min_contains(&self) -> Option<usize> {
        self.min_contains
    }

    /// ## `maxProperties`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An object instance is valid against "maxProperties" if its number of properties is less than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.5.1 `maxProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxproperties)
    #[must_use]
    #[inline]
    pub fn max_properties(&self) -> Option<usize> {
        self.max_properties
    }

    /// ## `minProperties`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An object instance is valid against "minProperties" if its number of
    /// properties is greater than, or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of `0`.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.5.2 `minProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minproperties)
    #[must_use]
    #[inline]
    pub fn min_properties(&self) -> Option<usize> {
        self.min_properties
    }

    /// ## `required`
    /// The value of this keyword MUST be an array. Elements of this array, if
    /// any, MUST be strings, and MUST be unique.
    ///
    /// An object instance is valid against this keyword if every item in the
    /// array is the name of a property in the instance.
    ///
    /// Omitting this keyword has the same behavior as an empty array.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.5.3 `required`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-required)
    #[must_use]
    #[inline]
    pub fn required(&self) -> &HashSet<String> {
        &self.required
    }

    /// ## `dependentRequired`
    /// The value of this keyword MUST be an object. Properties in this object,
    /// if any, MUST be arrays. Elements in each array, if any, MUST be strings,
    /// and MUST be unique.
    ///
    /// This keyword specifies properties that are required if a specific other
    /// property is present. Their requirement is dependent on the presence of the
    /// other property.
    ///
    /// Validation succeeds if, for each name that appears in both the instance and
    /// as a name within this keyword's value, every item in the corresponding array
    /// is also the name of a property in the instance.
    ///
    /// Omitting this keyword has the same behavior as an empty object.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.5.4 `dependentRequired`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-dependentrequired)
    #[must_use]
    #[inline]
    pub fn dependent_required(&self) -> &HashMap<String, HashSet<String>> {
        &self.dependent_required
    }

    /// ## `contentEncoding`
    /// If the instance value is a string, this property defines that the string
    /// SHOULD be interpreted as encoded binary data and decoded using the
    /// encoding named by this property.
    ///
    /// Possible values indicating base 16, 32, and 64 encodings with several
    /// variations are listed in [RFC
    /// 4648](https://www.rfc-editor.org/info/rfc4648). Additionally, sections
    /// 6.7 and 6.8 of [RFC 2045](https://www.rfc-editor.org/info/rfc2045)
    /// provide encodings used in MIME. This keyword is derived from MIME's
    /// Content-Transfer-Encoding header, which was designed to map binary data
    /// into ASCII characters. It is not related to HTTP's Content-Encoding
    /// header, which is used to encode (e.g. compress or encrypt) the content
    /// of HTTP request and responses.
    ///
    /// As "base64" is defined in both RFCs, the definition from RFC 4648 SHOULD
    /// be assumed unless the string is specifically intended for use in a MIME
    /// context. Note that all of these encodings result in strings consisting
    /// only of 7-bit ASCII characters. Therefore, this keyword has no meaning
    /// for strings containing characters outside of that range.
    ///
    /// If this keyword is absent, but "contentMediaType" is present, this
    /// indicates that the encoding is the identity encoding, meaning that no
    /// transformation was needed in order to represent the content in a UTF-8
    /// string.
    ///
    /// The value of this property MUST be a string.
    /// - [JSON Schema Validation 2020-12 # 8.3. `contentEncoding`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentencoding)
    /// - [Understanding JSON Schema # Media: string-encoding non-JSON data - `contentEncoding`](https://json-schema.org/understanding-json-schema/reference/non_json_data.html#id2)
    #[must_use]
    #[inline]
    pub fn content_encoding(&self) -> Option<&str> {
        self.content_encoding.as_deref()
    }

    /// ## `contentMediaType`
    /// If the instance is a string, this property indicates the media type of
    /// the contents of the string. If "contentEncoding" is present, this
    /// property describes the decoded string.
    ///
    /// The value of this property MUST be a string, which MUST be a media type, as defined by [RFC 2046]().
    ///
    /// - [JSON Schema Validation 2020-12 # 8.4. `contentMediaType`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentmediatype)
    #[must_use]
    #[inline]
    pub fn content_media_type(&self) -> Option<&str> {
        self.content_media_type.as_deref()
    }

    /// ## `contentSchema`
    /// If the instance is a string, and if "contentMediaType" is present, this property contains a schema which describes the structure of the string.
    ///
    /// This keyword MAY be used with any media type that can be mapped into JSON Schema's data model.
    ///
    /// The value of this property MUST be a valid JSON schema. It SHOULD be ignored if "contentMediaType" is not present   
    ///
    /// ### Example
    /// ```json
    /// {
    ///     "type": "string",
    ///     "contentMediaType": "application/jwt",
    ///     "contentSchema": {
    ///         "type": "array",
    ///         "minItems": 2,
    ///         "prefixItems": [
    ///             {
    ///                 "const": {
    ///                     "typ": "JWT",
    ///                     "alg": "HS256"
    ///                 }
    ///             },
    ///             {
    ///                 "type": "object",
    ///                 "required": ["iss", "exp"],
    ///                 "properties": {
    ///                     "iss": {"type": "string"},
    ///                     "exp": {"type": "integer"}
    ///                 }
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// - [JSON Schema Validation 2020-12 # 8.5. `contentSchema`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentschema)
    #[must_use]
    #[inline]
    pub fn content_schema(&self) -> Option<SchemaRef> {
        self.content_schema
    }

    /// ## `title`
    /// A title can be used to decorate a user interface with information about
    /// the data produced by this user interface.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.1 `"title"` and "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
    #[must_use]
    #[inline]
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// ## `description`
    /// A `description` can provide explanation about the purpose of the
    /// instance described by this schema.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.1 `"title"` and
    ///   "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
    #[must_use]
    #[inline]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// ## `deprecated`
    /// The value of this keyword MUST be a boolean. When multiple occurrences
    /// of this keyword are applicable to a single sub-instance, applications
    /// SHOULD consider the instance location to be deprecated if any occurrence
    /// specifies a true value.
    ///
    /// If "deprecated" has a value of boolean true, it indicates that
    /// applications SHOULD refrain from usage of the declared property. It MAY
    /// mean the property is going to be removed in the future.
    ///
    /// A root schema containing "deprecated" with a value of true indicates
    /// that the entire resource being described MAY be removed in the future.
    ///
    /// The "deprecated" keyword applies to each instance location to which the
    /// schema object containing the keyword successfully applies. This can
    /// result in scenarios where every array item or object property is
    /// deprecated even though the containing array or object is not.
    ///
    /// Omitting this keyword has the same behavior as a value of false.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.3 `"deprecated"`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-deprecated)    
    #[must_use]
    #[inline]
    pub fn deprecated(&self) -> Option<bool> {
        self.deprecated
    }

    /// ## `readOnly`
    ///  When multiple occurrences of these keywords are applicable to a single
    ///  sub-instance, the resulting behavior SHOULD be as for a `true` value if
    ///  any occurrence specifies a `true` value, and SHOULD be as for a `false`
    ///  value otherwise.
    ///
    /// If `"readOnly"` has a value of boolean `true`, it indicates that the value
    /// of the instance is managed exclusively by the owning authority, and
    /// attempts by an application to modify the value of this property are
    /// expected to be ignored or rejected by that owning authority.
    ///
    /// An instance document that is marked as "readOnly" for the entire
    /// document MAY be ignored if sent to the owning authority, or MAY result
    /// in an error, at the authority's discretion.
    ///
    /// This keyword can be used to assist in user interface instance
    /// generation.  The "readOnly" keyword does not imply how a server handles
    /// writes to a value in the case of a conflict - e.g., whether it rejects
    /// them or whether it attempts to resolve the conflict in some manner.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.4 `"readOnly"` and `"writeOnly"`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-readonly-and-writeonly)    

    pub fn read_only(&self) -> Option<bool> {
        self.read_only
    }
    /// ## `writeOnly`
    /// When multiple occurrences of these keywords are applicable to a single
    /// sub-instance, the resulting behavior SHOULD be as for a `true` value if
    /// any occurrence specifies a `true` value, and SHOULD be as for a `false`
    /// value otherwise.
    ///
    /// If `"writeOnly"` has a value of boolean true, it indicates that the value
    /// is never present when the instance is retrieved from the owning
    /// authority. It can be present when sent to the owning authority to update
    /// or create the document (or the resource it represents), but it will not
    /// be included in any updated or newly created version of the instance.
    ///
    /// An instance document that is marked as `"writeOnly"` for the entire
    /// document MAY be returned as a blank document of some sort, or MAY
    /// produce an error upon retrieval, or have the retrieval request ignored,
    /// at the authority's discretion.
    /// - [JSON Schema Validation 2020-12 # 9.4 `"readOnly"` and `"writeOnly"`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-readonly-and-writeonly)    
    #[must_use]
    #[inline]
    pub fn write_only(&self) -> Option<bool> {
        self.write_only
    }

    /// ## `examples`
    /// The value of this keyword MUST be an array. There are no restrictions placed on the values within the array. When multiple occurrences of this keyword are applicable to a single sub-instance, implementations MUST provide a flat array of all values rather than an array of arrays.
    ///
    /// This keyword can be used to provide sample JSON values associated with a particular schema, for the purpose of illustrating usage. It is RECOMMENDED that these values be valid against the associated schema.
    ///
    /// Implementations MAY use the value(s) of "default", if present, as an
    /// additional example. If "examples" is absent, "default" MAY still be used
    /// in this manner.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.5 `"examples"`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-examples)
    #[must_use]
    #[inline]
    pub fn examples(&self) -> Option<&Vec<Value>> {
        self.examples.as_ref()
    }
}
impl CompiledSchema {
    fn set_subschemas(
        &mut self,
        schemas: HashMap<Keyword<'_>, SchemaRef>,
        schema_lists: HashMap<Keyword<'_>, Vec<SchemaRef>>,
        schema_maps: HashMap<Keyword<'_>, HashMap<Keyword<'_>, SchemaRef>>,
    ) {
        self.set_single_schemas(schemas);
        self.set_subschema_lists(schema_lists);
        self.set_subschema_maps(schema_maps);
    }
    fn set_single_schemas(&mut self, _schemas: HashMap<Keyword<'_>, SchemaRef>) {}
    fn set_subschema_maps(
        &mut self,
        schema_maps: HashMap<Keyword<'_>, HashMap<Keyword<'_>, SchemaRef>>,
    ) {
        self.subschema_maps = HashMap::new();
        for (k, v) in schema_maps {
            let m = self
                .subschema_maps
                .entry(k.to_string())
                .or_insert(HashMap::new());
            for (i, s) in &v {
                m.insert(i.to_string(), s.clone());
                let key = format!("{k}/{i}");
                self.subschemas.insert(key, s.clone());
            }
            match k {
                Keyword::DEFS => self.definitions = collect_schemas_from_map(v),
                Keyword::DEFINITIONS_LEGACY => {
                    self.definitions_legacy = collect_schemas_from_map(v)
                }
                Keyword::PROPERTIES => self.properties = collect_schemas_from_map(v),
                Keyword::DEPENDENT_SCHEMAS => self.dependent_schemas = collect_schemas_from_map(v),
                _ => {}
            }
        }
    }
    fn set_subschema_lists(&mut self, schema_lists: HashMap<Keyword<'_>, Vec<SchemaRef>>) {
        self.subschema_lists = HashMap::new();

        for (k, v) in schema_lists {
            self.subschema_lists.insert(k.to_string(), v.clone());
            for (i, s) in v.iter().enumerate() {
                let key = format!("{k}/{i}");
                self.subschemas.insert(key, s.clone());
            }
            match k {
                Keyword::ALL_OF => self.all_of = v,
                Keyword::ANY_OF => self.any_of = v,
                Keyword::ONE_OF => self.one_of = v,
                Keyword::PREFIX_ITEMS => self.prefix_items = v,
                _ => {}
            }
        }

        // self.all_of = schema_lists.get("allOf").cloned().unwrap_or_default();
        // self.any_of = schema_lists.get("anyOf").cloned().unwrap_or_default();
        // self.one_of = schema_lists.get("oneOf").cloned().unwrap_or_default();
        // self.prefix_items = schema_lists.get("prefixItems").cloned().unwrap_or_default();
    }
}

fn collect_schemas_from_map(v: HashMap<Keyword<'_>, SchemaRef>) -> HashMap<String, SchemaRef> {
    v.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}
