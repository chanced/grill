use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use std::collections::{BTreeMap, HashSet};

use crate::Schema;

use super::{BoolOrNumber, Format, Items, Types};

/// A raw JSON Schema object.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Object {
    /// ## `$id`
    /// The value of `$id` is a URI-reference without a fragment that resolves
    /// against the Retrieval URI. The resulting URI is the base URI for the
    /// schema.
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.1. The `"$id"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword)
    /// - [Understanding JSON Schema # Structuring a complex schema: `$id`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=id#id)
    #[serde(rename = "$id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// ## `$schema`
    /// The `$schema` keyword is both used as a JSON Schema dialect identifier
    /// and as the identifier of a resource which is itself a JSON Schema, which
    /// describes the set of valid schemas written for this particular dialect.
    ///
    /// - [JSON Schema Core 2020-12 # 8.1.1. The `"$schema"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)
    /// - [Draft 2019-09 Core # 8.1.1. The `"$schema"` Keyword](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.1.1)
    /// - [Draft 7 # 7. The `"$schema"` Keyword](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-7)
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

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
    #[serde(rename = "$comment", default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
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
    #[serde(
        rename = "$vocabulary",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub vocabulary: BTreeMap<String, bool>,

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
    /// periods `'.'`. This matches the US-ASCII part of XML's NCName production
    /// [xml-names]. Note that the anchor string does not include the `'#'`
    /// character, as it is not a URI-reference. An `"$anchor": "foo"` becomes
    /// the fragment `"#foo"` when used in a URI. See below for full examples.
    ///
    ///
    /// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
    /// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with `"$dynamicRef"`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
    /// - [Draft 2019-09 Core # 8.2.3. Defining location-independent identifiers with `"$anchor"`](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.2.3)
    /// - [Understanding JSON Schema # `$anchor`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=anchor#anchor)
    #[serde(rename = "$anchor", default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,

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
    #[serde(
        rename = "$dynamicAnchor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_anchor: Option<String>,

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
    #[serde(
        rename = "$dynamicRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_reference: Option<String>,

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
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// ##`$recursiveAnchor` `"$recursiveAnchor"` is used to dynamically
    /// identify a base URI at runtime for `"$recursiveRef"` by marking where
    /// such a calculation can start, and where it stops.  This keyword MUST NOT
    /// affect the base URI of other keywords, unless they are explicitly
    /// defined to rely on it.
    ///
    /// - [Draft 2019-09 Core # 8.2.4.2.2.  Enabling Recursion with `"$recursiveAnchor"`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2.2)
    #[serde(
        rename = "$recursiveAnchor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursive_anchor: Option<bool>,

    /// ##`$recursiveRef` The `"$recursiveRef"` and `"$recursiveAnchor"`
    /// keywords are used to construct extensible recursive schemas.  A
    /// recursive schema is one that has a reference to its own root, identified
    /// by the empty fragment URI reference `'#'`.
    ///
    /// - [Draft 2019-09 Core # 8.2.4.2.  Recursive References with `"$recursiveRef"`
    ///   and
    ///   `"$recursiveAnchor"`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2)
    #[serde(
        rename = "$recursiveRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursive_reference: Option<String>,
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
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Types>,

    /// The format keyword allows for basic semantic identification of certain
    /// kinds of string values that are commonly used. For example, because JSON
    /// doesn’t have a `DateTime` type, dates need to be encoded as strings.
    /// format allows the schema author to indicate that the string value should
    /// be interpreted as a date. By default, format is just an annotation and
    /// does not effect validation.
    ///
    /// - [JSON Schema Core 2020-12 # 7. Vocabularies for Semantic Content With `"format"`](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c)
    /// - [Understanding Json Schema # string Built-in Formats](https://json-schema.org/understanding-json-schema/reference/string.html#id7)
    /// - [OpenAPI 3.1 Specification # 4.2 Format](https://spec.openapis.org/oas/v3.1.0#format)
    #[serde(rename = "format", default, skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,

    /// ## `const`
    /// The `"const"` keyword is used to restrict a value to a single value.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.1.3. `const`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-const)
    /// - [Understanding JSON Schema - Constant values](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#constant-values)
    #[serde(rename = "const", default, skip_serializing_if = "Option::is_none")]
    pub constant: Option<serde_json::Value>,

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
    #[serde(rename = "$defs", default, skip_serializing_if = "Option::is_none")]
    pub definitions: Option<BTreeMap<String, Schema>>,

    /// ## `definitions`
    /// Legacy from Draft 07. See [`definitions`](`Object::definitions`).
    ///
    /// ## Note
    /// If using JSON Schema 07, use this field instead of [`definitions`](`Object::definitions`).
    ///
    /// - [Understanding JSON Schema # `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
    #[serde(
        rename = "definitions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[deprecated]
    pub definitions_legacy: Option<BTreeMap<String, Schema>>,

    /// ## `allOf`
    /// The `"allOf"` keyword acts as an `AND` where each subschema must be
    /// valid
    ///
    /// - [Understanding JSON Schema # Schema Composition `allOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=allOf#allOf)
    #[serde(rename = "allOf", default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<Schema>,

    /// ## `anyOf`
    /// The `"anyOf"` keyword acts as an `OR` where at least one of the
    /// subschemas must be valid
    ///
    /// - [Understanding JSON Schema # Schema Composition `anyOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=anyof#anyOf)
    #[serde(rename = "anyOf", default, skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<Schema>,

    /// ## `oneOf`
    /// The `"oneOf"` keyword acts as an `XOR` where exactly one of the
    /// subschemas must be valid
    ///
    /// - [Understanding JSON Schema # Schema Composition `oneOf`](https://json-schema.org/understanding-json-schema/reference/combining.html#oneof)
    #[serde(rename = "oneOf", default, skip_serializing_if = "Vec::is_empty")]
    pub one_of: Vec<Schema>,

    /// ## `not`
    /// The not keyword declares that an instance validates if it doesn’t
    /// validate against the given subschema.
    ///
    /// - [Understanding JSON Schema # Schema Composition `not`](https://json-schema.org/understanding-json-schema/reference/combining.html?#id8)
    #[serde(rename = "not", default, skip_serializing_if = "Option::is_none")]
    pub not: Option<Schema>,

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
    #[serde(rename = "if", default, skip_serializing_if = "Option::is_none")]
    pub cond_if: Option<Schema>,

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
    #[serde(rename = "then", default, skip_serializing_if = "Option::is_none")]
    pub cond_then: Option<Schema>,

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
    #[serde(rename = "else", default, skip_serializing_if = "Option::is_none")]
    pub cond_else: Option<Schema>,

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
    #[serde(
        rename = "dependentSchemas",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub dependent_schemas: BTreeMap<String, Schema>,

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
    #[serde(rename = "prefixItems", default, skip_serializing_if = "Vec::is_empty")]
    pub prefix_items: Vec<Schema>,

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
    #[serde(rename = "items", default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Items>,

    /// - [JSON Schema Core 2019-09 # 9.3.1.2.  `additionalItems`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-9.3.1.2)
    pub additional_items: Option<Schema>,

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
    #[serde(rename = "contains", default, skip_serializing_if = "Option::is_none")]
    pub contains: Option<Schema>,

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
    #[serde(
        rename = "properties",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub properties: BTreeMap<String, Schema>,

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
    #[serde(
        rename = "patternProperties",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub pattern_properties: BTreeMap<String, String>,

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
    #[serde(
        rename = "additionalProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_properties: Option<Schema>,

    /// ## `propertyNames`
    /// If the instance is an object, this keyword validates if every property
    /// name in the instance validates against the provided schema. Note the
    /// property name that the schema is testing will always be a string.
    ///
    /// - [JSON Schema Core 2020-12 # 10.3.2.4.`propertyNames`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-propertynames)
    #[serde(
        rename = "propertyNames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub property_names: Option<Schema>,

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
    #[serde(
        rename = "unevaluatedItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unevaluated_items: Option<Schema>,
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
    #[serde(
        rename = "unevaluatedProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unevaluated_properties: Option<Schema>,

    /// ## `enum`
    /// An instance validates successfully against this keyword if its value is
    /// equal to one of the elements in this keyword's array value.
    ///
    /// Elements in the array might be of any type, including null.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.1.2. `enum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-enum)
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enumeration: Vec<Value>,

    /// ## `multipleOf`
    /// The value of `"multipleOf"` MUST be a number, strictly greater than 0.
    ///
    /// A numeric instance is valid only if division by this keyword's value
    /// results in an integer.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.2.1. `multipleOf`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-multipleof)
    #[serde(rename = "multipleOf", skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<Number>,

    /// #`maximum`
    /// The value of `"maximum"` MUST be a number, representing an inclusive upper
    /// limit for a numeric instance.
    ///
    /// If the instance is a number, then this keyword validates only if the
    /// instance is less than or exactly equal to `"maximum"`.
    /// - [JSON Schema Validation 2020-12 # 6.2.2. `maximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maximum)
    #[serde(rename = "maximum", skip_serializing_if = "Option::is_none")]
    pub maximum: Option<Number>,

    /// ## `exclusiveMaximum`
    /// For JSON Schema drafts 7 and higher, the value of `"exclusiveMaximum"` MUST be a number, representing an
    /// exclusive upper limit for a numeric instance. For JSON Schema Draft 4, the value of `"exclusiveMaximum"` MUST
    /// be a boolean.
    ///
    ///
    /// If the instance is a number, then the instance is valid only if it has a
    /// value strictly less than (not equal to) `"exclusiveMaximum"`.
    /// - [JSON Schema Validation 2020-12 # 6.2.3. `exclusiveMaximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusivemaximum)
    #[serde(rename = "exclusiveMaximum", skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<BoolOrNumber>,

    /// ## `minimum`
    /// The value of `"minimum"` MUST be a number, representing an inclusive
    /// lower limit for a numeric instance.
    ///
    /// If the instance is a number, then this keyword validates only if the
    /// instance is greater than or exactly equal to `"minimum"`.
    /// - [JSON Schema Validation 2020-12 # 6.2.4. `minimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minimum)
    #[serde(rename = "minimum", skip_serializing_if = "Option::is_none")]
    pub minimum: Option<Number>,

    /// ## `exclusiveMinimum`
    ///
    /// For JSON Schema drafts 7 and higher, the value of `"exclusiveMinimum"` MUST be a number, representing an
    /// exclusive lower limit for a numeric instance.
    ///
    /// If the instance is a number, then the instance is valid only if it has a
    /// value strictly greater than (not equal to) `"exclusiveMinimum"`.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.2.5. `exclusiveMinimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusiveminimum)
    #[serde(rename = "exclusiveMinimum", skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<BoolOrNumber>,

    /// ## `maxLength`
    /// The value of `"maxLength"` MUST be a non-negative integer.
    ///
    /// A string instance is valid against this keyword if its length is less
    /// than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.1. `maxLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxlength)
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,

    /// ## `minLength`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// A string instance is valid against this keyword if its length is greater
    /// than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.2. `minLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minlength)
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,

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
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// ## `maxItems`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An array instance is valid against "maxItems" if its size is less than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.3.3. `maxItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxitems)
    #[serde(rename = "maxItems", skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,

    /// ## `minItems`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An array instance is valid against "minItems" if its size is greater than,
    /// or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of 0.
    /// - [JSON Schema Validation 2020-12 # 6.3.3. `minItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minitems)
    #[serde(rename = "minItems", skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,

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
    #[serde(rename = "uniqueItems", skip_serializing_if = "Option::is_none")]
    pub unique_items: Option<bool>,

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
    #[serde(rename = "maxContains", skip_serializing_if = "Option::is_none")]
    pub max_contains: Option<usize>,

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
    #[serde(rename = "minContains", skip_serializing_if = "Option::is_none")]
    pub min_contains: Option<usize>,

    /// ## `maxProperties`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An object instance is valid against "maxProperties" if its number of properties is less than, or equal to, the value of this keyword.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.5.1 `maxProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxproperties)
    #[serde(rename = "maxProperties", skip_serializing_if = "Option::is_none")]
    pub max_properties: Option<usize>,

    /// ## `minProperties`
    /// The value of this keyword MUST be a non-negative integer.
    ///
    /// An object instance is valid against "minProperties" if its number of
    /// properties is greater than, or equal to, the value of this keyword.
    ///
    /// Omitting this keyword has the same behavior as a value of `0`.
    ///
    /// - [JSON Schema Validation 2020-12 # 6.5.2 `minProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minproperties)
    #[serde(rename = "minProperties", skip_serializing_if = "Option::is_none")]
    pub min_properties: Option<usize>,

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
    #[serde(
        rename = "required",
        default,
        skip_serializing_if = "HashSet::is_empty"
    )]
    pub required: HashSet<String>,

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
    #[serde(
        rename = "dependentRequired",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub dependent_required: BTreeMap<String, HashSet<String>>,

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
    #[serde(rename = "contentEncoding", skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,

    /// ## `contentMediaType`
    /// If the instance is a string, this property indicates the media type of
    /// the contents of the string. If "contentEncoding" is present, this
    /// property describes the decoded string.
    ///
    /// The value of this property MUST be a string, which MUST be a media type, as defined by [RFC 2046]().
    ///
    /// - [JSON Schema Validation 2020-12 # 8.4. `contentMediaType`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentmediatype)
    #[serde(rename = "contentMediaType", skip_serializing_if = "Option::is_none")]
    pub content_media_type: Option<String>,

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
    #[serde(rename = "contentSchema", skip_serializing_if = "Option::is_none")]
    pub content_schema: Option<Schema>,

    /// ## `title`
    /// A title can be used to decorate a user interface with information about
    /// the data produced by this user interface.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.1 `"title"` and "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// ## `description`
    /// A `description` can provide explanation about the purpose of the
    /// instance described by this schema.
    ///
    /// - [JSON Schema Validation 2020-12 # 9.1 `"title"` and
    ///   "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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
    #[serde(rename = "deprecated", skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,

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
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
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
    #[serde(rename = "writeOnly", skip_serializing_if = "Option::is_none")]
    pub write_only: Option<bool>,

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
    #[serde(rename = "examples", skip_serializing_if = "Option::is_none")]
    pub examples: Option<Vec<Value>>,

    #[serde(flatten, default)]
    pub additional_keywords: BTreeMap<String, Value>,
}

// fn non_fragmented_uri<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let mut s: Option<String> = Option::deserialize(deserializer)?;
//     if let Some(v) = s.as_ref() {
//         if matches!(v.find('#'), Some(idx) if idx < v.len() - 1) {
//             let v = v.trim();
//             if matches!(v.find('#'), Some(idx) if idx < v.len() - 1) {
//                 return Err(serde::de::Error::custom("URI must not contain a fragment"));
//             }
//             s = Some(v.to_string());
//         }
//     }
//     Ok(s)
// }
