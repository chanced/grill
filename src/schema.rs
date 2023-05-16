mod format;
mod types;

pub use format::Format;

use crate::Uri;
use serde::{Deserialize, Serialize};
pub use types::{Type, Types};

/// A raw JSON Schema object.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Object {
    /// The value of `$id` is a URI-reference without a fragment that resolves
    /// against the Retrieval URI. The resulting URI is the base URI for the
    /// schema.
    ///
    /// Note: In JSON Schema Draft 4, field was `id` rather than `$id`.
    ///
    /// - [Draft 2020-12 # 8.2.1. The `"$id"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword)
    /// - [Understanding JSON Schema # `$id`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=id#id)
    #[serde(
        rename = "$id",
        alias = "id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Uri>,

    /// The `$schema` keyword is both used as a JSON Schema dialect identifier
    /// and as the identifier of a resource which is itself a JSON Schema, which
    /// describes the set of valid schemas written for this particular dialect.
    ///
    /// - [Draft 2020-12 # 8.1.1. The `"$schema"` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)
    /// - [Draft 2019-09 # 8.1.1. The `"$schema"` Keyword](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.1.1)
    /// - [Draft 7 # 7. The `"$schema"` Keyword](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-7)
    /// - [Draft 4 # 6. The `"$schema"` Keyword](https://datatracker.ietf.org/doc/html/draft-zyp-json-schema-04#section-6)
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Uri>,

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
    /// with a letter (`[A-Za-z]`) or underscore `'_'`, followed by any number of
    /// letters, digits (`[0-9]`), hyphens `'-'`, underscores `'_'`, and periods
    /// `'.'`. This matches the US-ASCII part of XML's NCName production
    /// [xml-names]. Note that the anchor string does not include the `'#'`
    /// character, as it is not a URI-reference. An `"$anchor": "foo"` becomes
    /// the fragment `"#foo"` when used in a URI. See below for full examples.
    ///
    /// - [Draft
    ///   2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
    /// - [Draft
    ///   2019-09](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.2.3)
    /// - [Understanding JSON
    ///   Schema # `$anchor`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=anchor#anchor)
    #[serde(rename = "$anchor", default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,

    /// A `"$dynamicAnchor"` can be thought of like a normal $anchor except that
    /// it can be referenced across schemas rather than just in the schema where
    /// it was defined. You can think of the old `"$recursiveAnchor"` as working
    /// the same way except that it only allowed you to create one anchor per
    /// schema, it had to be at the root of the schema, and the anchor name is
    /// always empty.
    ///
    /// - [Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
    /// - [Draft 2020-12 Release Notes](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
    #[serde(
        rename = "$dynamicAnchor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_anchor: Option<String>,

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
    ///   2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
    /// - [Draft 2020-12 Release
    ///   Notes](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
    #[serde(
        rename = "$dynamicRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_reference: Option<Uri>,

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
    /// - [Draft 2020-12](https://json-schema.org/draft/2020-12/json-schema-core.html#ref)
    /// - [Understanding JSON Schema # `$ref`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=ref#ref)
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<Uri>,

    /// `"$recursiveAnchor"` is used to dynamically identify a base URI at
    /// runtime for `"$recursiveRef"` by marking where such a calculation can
    /// start, and where it stops.  This keyword MUST NOT affect the base URI of
    /// other keywords, unless they are explicitly defined to rely on it.
    ///
    /// - [Draft 2019-09](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2.2)
    #[serde(
        rename = "$recursiveAnchor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursive_anchor: Option<bool>,

    /// The `"$recursiveRef"` and `"$recursiveAnchor"` keywords are used to
    /// construct extensible recursive schemas.  A recursive schema is one
    /// that has a reference to its own root, identified by the empty
    /// fragment URI reference `'#'`.
    ///
    /// - [Draft 2019-09](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2)
    #[serde(
        rename = "$recursiveRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recursive_reference: Option<Uri>,

    /// The `"$recursiveRef"` keyword is used for extending recursive schemas such
    /// as meta-schemas    

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
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Types>,
}

/// A raw JSON Schema document.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Schema {
    Bool(bool),
    Object(Object),
}
