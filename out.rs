#![feature(prelude_import)]
//! JSON Schema dialect implementations 04, 07, 2019-09, 2020-12.
#![deny(clippy::all, clippy::pedantic)]
#![warn(missing_docs)]
#![allow(clippy::implicit_hasher, clippy::wildcard_imports)]
#![recursion_limit = "256"]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod compile {
    use grill_uri::AbsoluteUri;
    use snafu::Snafu;
    /// Failed to compile a schema.
    #[snafu(display("failed to compile schema \"{uri}\""))]
    pub struct CompileError<E> {
        /// [`AbsoluteUri`] of the schema.
        pub uri: AbsoluteUri,
        /// Cause of the error.
        #[snafu(source, backtrace)]
        pub cause: CompileErrorCause<E>,
    }
    #[automatically_derived]
    impl<E: ::core::fmt::Debug> ::core::fmt::Debug for CompileError<E> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CompileError",
                "uri",
                &self.uri,
                "cause",
                &&self.cause,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl<E> ::snafu::Error for CompileError<E>
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "CompileError",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                Self { ref cause, .. } => ::core::option::Option::Some(cause.as_error_source()),
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                Self { ref cause, .. } => ::core::option::Option::Some(cause.as_error_source()),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl<E> ::snafu::ErrorCompat for CompileError<E> {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref cause, .. } => ::snafu::ErrorCompat::backtrace(cause),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl<E> ::core::fmt::Display for CompileError<E> {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self { ref cause, ref uri } => __snafu_display_formatter
                    .write_fmt(format_args!("failed to compile schema \"{0}\"", uri)),
            }
        }
    }
    ///SNAFU context selector for the `CompileError` error
    struct CompileSnafu<__T0> {
        #[allow(missing_docs)]
        uri: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for CompileSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(f, "CompileSnafu", "uri", &&self.uri)
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for CompileSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for CompileSnafu<__T0> {
        #[inline]
        fn clone(&self) -> CompileSnafu<__T0> {
            CompileSnafu {
                uri: ::core::clone::Clone::clone(&self.uri),
            }
        }
    }
    impl<E, __T0> ::snafu::IntoError<CompileError<E>> for CompileSnafu<__T0>
    where
        CompileError<E>: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<AbsoluteUri>,
    {
        type Source = CompileErrorCause<E>;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> CompileError<E> {
            let error: CompileErrorCause<E> = (|v| v)(error);
            CompileError {
                cause: error,
                uri: ::core::convert::Into::into(self.uri),
            }
        }
    }
    /// The cause of a [`CompileError`].
    pub enum CompileErrorCause<E> {}
    #[automatically_derived]
    impl<E: ::core::fmt::Debug> ::core::fmt::Debug for CompileErrorCause<E> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {}
        }
    }
    #[allow(single_use_lifetimes)]
    impl<E> ::core::fmt::Display for CompileErrorCause<E> {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {}
        }
    }
    #[allow(single_use_lifetimes)]
    impl<E> ::snafu::Error for CompileErrorCause<E>
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {}
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {}
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {}
        }
    }
    #[allow(single_use_lifetimes)]
    impl<E> ::snafu::ErrorCompat for CompileErrorCause<E> {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {}
        }
    }
}
pub mod keyword {
    use crate::{alias, schema::CompiledSchema, Specification};
    use enum_dispatch::enum_dispatch;
    use grill_core::{
        big::BigRational,
        lang::{source::Source, Numbers, Schemas, Sources, Values},
        Key,
    };
    use grill_uri::AbsoluteUri;
    use serde_json::{Number, Value};
    use snafu::Snafu;
    use std::{
        fmt::{self, Debug},
        sync::Arc,
    };
    mod consts {
        /// ## `$id`
        ///
        /// The value of `$id` is a URI-reference without a fragment that resolves
        /// against the Retrieval URI. The resulting URI is the base URI for the schema.
        ///
        /// - [JSON Schema Core 2020-12 # 8.2.1. The `$id`
        ///   Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword)
        /// - [Understanding JSON Schema # Structuring a complex schema:
        ///   `$id`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=id#id)
        pub const ID: &str = "$id";
        /// ## `id`
        ///
        /// The value of `id` is a URI-reference without a fragment that resolves
        /// against the Retrieval URI. The resulting URI is the base URI for the schema.
        ///
        /// # Deprecated
        /// Renamed to `$id` in JSON Schema 07
        pub const ID_LEGACY: &str = "id";
        /// ## `$schema`
        ///
        /// The `$schema` keyword is both used as a JSON Schema dialect identifier and
        /// as the identifier of a resource which is itself a JSON Schema, which
        /// describes the set of valid schemas written for this particular dialect.
        ///
        /// - [JSON Schema Core 2020-12 # 8.1.1. The `$schema`
        ///   Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)
        /// - [Draft 2019-09 Core # 8.1.1. The `$schema`
        ///   Keyword](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.1.1)
        /// - [Draft 7 # 7. The `$schema`
        ///   Keyword](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-7)
        pub const SCHEMA: &str = "$schema";
        /// ## `$comment`
        ///
        /// The `$comment` keyword is strictly intended for adding comments to a schema.
        /// Its value must always be a string. Unlike the annotations title,
        /// description, and examples, JSON schema implementations aren’t allowed to
        /// attach any meaning or behavior to it whatsoever, and may even strip them at
        /// any time. Therefore, they are useful for leaving notes to future editors of
        /// a JSON schema, but should not be used to communicate to users of the schema.
        ///
        /// - [Understanding JSON Schema # Generic keywords:
        ///   Comments](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#comments)
        pub const COMMENT: &str = "$comment";
        /// ## `$vocabulary`
        ///
        /// The `$vocabulary` keyword is used in meta-schemas to identify the
        /// vocabularies available for use in schemas described by that meta-schema. It
        /// is also used to indicate whether each vocabulary is required or optional, in
        /// the sense that an implementation MUST understand the required vocabularies
        /// in order to successfully process the schema. Together, this information
        /// forms a dialect. Any vocabulary that is understood by the implementation
        /// MUST be processed in a manner consistent with the semantic definitions
        /// contained within the vocabulary.
        ///
        /// The value of this keyword MUST be an object. The property names in the
        /// object MUST be URIs (containing a scheme) and this URI MUST be normalized.
        /// Each URI that appears as a property name identifies a specific set of
        /// keywords and their semantics.
        ///
        /// The URI MAY be a URL, but the nature of the retrievable resource is
        /// currently undefined, and reserved for future use. Vocabulary authors MAY use
        /// the URL of the vocabulary specification, in a human-readable media type such
        /// as text/html or text/plain, as the vocabulary URI. Vocabulary documents may
        /// be added in forthcoming drafts. For now, identifying the keyword set is
        /// deemed sufficient as that, along with meta-schema validation, is how the
        /// current "vocabularies" work today. Any future vocabulary document format
        /// will be specified as a JSON document, so using text/html or other non-JSON
        /// formats in the meantime will not produce any future ambiguity.
        ///
        /// The values of the object properties MUST be booleans. If the value is true,
        /// then implementations that do not recognize the vocabulary MUST refuse to
        /// process any schemas that declare this meta-schema with "$schema". If the
        /// value is false, implementations that do not recognize the vocabulary SHOULD
        /// proceed with processing such schemas. The value has no impact if the
        /// implementation understands the vocabulary.
        ///
        /// Per 6.5, unrecognized keywords SHOULD be treated as annotations. This
        /// remains the case for keywords defined by unrecognized vocabularies. It is
        /// not currently possible to distinguish between unrecognized keywords that are
        /// defined in vocabularies from those that are not part of any vocabulary.
        ///
        /// The `$vocabulary` keyword SHOULD be used in the root schema of any schema
        /// document intended for use as a meta-schema. It MUST NOT appear in
        /// subschemas.
        ///
        /// The `$vocabulary` keyword MUST be ignored in schema documents that are not
        /// being processed as a meta-schema. This allows validating a meta-schema M
        /// against its own meta-schema M' without requiring the validator to understand
        /// the vocabularies declared by M.
        ///
        /// - [JSON Schema Core 2020-12 # 8.1.2. The `$vocabulary`
        ///   Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.2)
        pub const VOCABULARY: &str = "$vocabulary";
        /// ## `title`
        ///
        /// A title can be used to decorate a user interface with information about the
        /// data produced by this user interface.
        ///
        /// - [JSON Schema Validation 2020-12 # 9.1 `title` and
        ///   "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
        pub const TITLE: &str = "title";
        /// ## `description`
        ///
        /// A `description` can provide explanation about the purpose of the
        /// instance described by this schema.
        ///
        /// - [JSON Schema Validation 2020-12 # 9.1 `title` and
        ///   "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
        pub const DESCRIPTION: &str = "description";
        /// ## `deprecated`
        ///
        /// The value of this keyword MUST be a boolean. When multiple occurrences of
        /// this keyword are applicable to a single sub-instance, applications SHOULD
        /// consider the instance location to be deprecated if any occurrence specifies
        /// a true value.
        ///
        /// If `deprecated` has a value of boolean true, it indicates that applications
        /// SHOULD refrain from usage of the declared property. It MAY mean the property
        /// is going to be removed in the future.
        ///
        /// A root schema containing `deprecated` with a value of true indicates that
        /// the entire resource being described MAY be removed in the future.
        ///
        /// The `deprecated` keyword applies to each instance location to which the
        /// schema object containing the keyword successfully applies. This can result
        /// in scenarios where every array item or object property is deprecated even
        /// though the containing array or object is not.
        ///
        /// Omitting this keyword has the same behavior as a value of false.
        ///
        /// - [JSON Schema Validation 2020-12 # 9.3
        ///   `deprecated`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-deprecated)
        pub const DEPRECATED: &str = "deprecated";
        /// ## `readOnly`
        ///
        ///  When multiple occurrences of these keywords are applicable to a single
        ///  sub-instance, the resulting behavior SHOULD be as for a `true` value if any
        ///  occurrence specifies a `true` value, and SHOULD be as for a `false` value
        ///  otherwise.
        ///
        /// If `readOnly` has a value of boolean `true`, it indicates that the value of
        /// the instance is managed exclusively by the owning authority, and attempts by
        /// an application to modify the value of this property are expected to be
        /// ignored or rejected by that owning authority.
        ///
        /// An instance document that is marked as `readOnly` for the entire document
        /// MAY be ignored if sent to the owning authority, or MAY result in an error,
        /// at the authority's discretion.
        ///
        /// This keyword can be used to assist in user interface instance generation.
        /// The `readOnly` keyword does not imply how a server handles writes to a value
        /// in the case of a conflict - e.g., whether it rejects them or whether it
        /// attempts to resolve the conflict in some manner.
        ///
        /// - [JSON Schema Validation 2020-12 # 9.4 `readOnly` and
        ///   `writeOnly`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-readOnly-and-writeonly)
        pub const READ_ONLY: &str = "readOnly";
        /// ## `writeOnly`
        ///
        /// When multiple occurrences of these keywords are applicable to a single
        /// sub-instance, the resulting behavior SHOULD be as for a `true` value if any
        /// occurrence specifies a `true` value, and SHOULD be as for a `false` value
        /// otherwise.
        ///
        /// If `writeOnly` has a value of boolean true, it indicates that the value is
        /// never present when the instance is retrieved from the owning authority. It
        /// can be present when sent to the owning authority to update or create the
        /// document (or the resource it represents), but it will not be included in any
        /// updated or newly created version of the instance.
        ///
        /// An instance document that is marked as `writeOnly` for the entire document
        /// MAY be returned as a blank document of some sort, or MAY produce an error
        /// upon retrieval, or have the retrieval request ignored, at the authority's
        /// discretion.
        /// - [JSON Schema Validation 2020-12 # 9.4 `readOnly` and
        ///   `writeOnly`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-readOnly-and-writeonly)
        pub const WRITE_ONLY: &str = "writeOnly";
        /// ## `$anchor`
        ///
        /// Using JSON Pointer fragments requires knowledge of the structure of the
        /// schema. When writing schema documents with the intention to provide
        /// re-usable schemas, it may be preferable to use a plain name fragment that is
        /// not tied to any particular structural location. This allows a subschema to
        /// be relocated without requiring JSON Pointer references to be updated.
        ///
        /// The `$anchor` and `$dynamicAnchor` keywords are used to specify such
        /// fragments. They are identifier keywords that can only be used to create
        /// plain name fragments, rather than absolute URIs as seen with `$id`.
        ///
        /// The base URI to which the resulting fragment is appended is the canonical
        /// URI of the schema resource containing the `$anchor` or `$dynamicAnchor` in
        /// question. As discussed in the previous section, this is either the nearest
        /// `$id` in the same or parent schema object, or the base URI for the document
        /// as determined according to RFC 3986.
        ///
        /// Separately from the usual usage of URIs, `$dynamicAnchor` indicates that the
        /// fragment is an extension point when used with the `$dynamicRef` keyword.
        /// This low-level, advanced feature makes it easier to extend recursive schemas
        /// such as the meta-schemas, without imposing any particular semantics on that
        /// extension. See the section on `$dynamicRef` [(Section
        /// 8.2.3.2)](https://json-schema.org/draft/2020-12/json-schema-core.html#dynamic-ref)
        /// for details.
        ///
        /// In most cases, the normal fragment behavior both suffices and is more
        /// intuitive. Therefore it is RECOMMENDED that `$anchor` be used to create
        /// plain name fragments unless there is a clear need for `$dynamicAnchor`.
        ///
        /// If present, the value of this keyword MUST be a string and MUST start with a
        /// letter (`[A-Za-z]`) or underscore `'_'`, followed by any number of letters,
        /// digits (`[0-9]`), hyphens `'-'`, underscores `'_'`, and periods `'.'`. This
        /// matches the US-ASCII part of XML's `NCName` production [xml-names]. Note
        /// that the anchor string does not include the `'#'` character, as it is not a
        /// URI-reference. An `"$anchor": "foo"` becomes the fragment `"#foo"` when used
        /// in a URI. See below for full examples.
        ///
        ///
        /// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent
        ///   identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
        /// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with
        ///   `$dynamicRef`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
        /// - [Draft 2019-09 Core # 8.2.3. Defining location-independent identifiers
        ///   with
        ///   `$anchor`](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.2.3)
        /// - [Understanding JSON Schema #
        ///   `$anchor`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=anchor#anchor)
        pub const ANCHOR: &str = "$anchor";
        /// ## `$dynamicAnchor`
        ///
        /// A `$dynamicAnchor` can be thought of like a normal $anchor except that it
        /// can be referenced across schemas rather than just in the schema where it was
        /// defined. You can think of the old `$recursiveAnchor` as working the same way
        /// except that it only allowed you to create one anchor per schema, it had to
        /// be at the root of the schema, and the anchor name is always empty.
        ///
        /// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with
        ///   `$dynamicRef`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
        /// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent
        ///   identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
        /// - [JSON Schema Core 2020-12 Release Notes # `$dynamicRef` and
        ///   `$dynamicAnchor`](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
        pub const DYNAMIC_ANCHOR: &str = "$dynamicAnchor";
        /// ## `$dynamicRef`
        ///
        /// The `$dynamicRef` keyword is an applicator that allows for deferring the
        /// full resolution until runtime, at which point it is resolved each time it is
        /// encountered while evaluating an instance.
        ///
        /// Together with `$dynamicAnchor`, `$dynamicRef` implements a cooperative
        /// extension mechanism that is primarily useful with recursive schemas (schemas
        /// that reference themselves). Both the extension point and the
        /// runtime-determined extension target are defined with `$dynamicAnchor`, and
        /// only exhibit runtime dynamic behavior when referenced with `$dynamicRef`.
        ///
        /// The value of the `$dynamicRef` property MUST be a string which is a
        /// URI-Reference. Resolved against the current URI base, it produces the URI
        /// used as the starting point for runtime resolution. This initial resolution
        /// is safe to perform on schema load.
        ///
        /// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with
        ///   `$dynamicRef`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
        /// - [JSON Schema Core 2020-12 Release Notes # `$dynamicRef` and
        ///   `$dynamicAnchor`](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
        pub const DYNAMIC_REF: &str = "$dynamicRef";
        /// ## `$ref`
        ///
        /// The `$ref` keyword is an applicator that is used to reference a statically
        /// identified schema. Its results are the results of the referenced schema.
        /// [CREF5]
        ///
        /// The value of the `$ref` keyword MUST be a string which is a URI-Reference.
        /// Resolved against the current URI base, it produces the URI of the schema to
        /// apply. This resolution is safe to perform on schema load, as the process of
        /// evaluating an instance cannot change how the reference resolves.
        ///
        /// - [JSON Schema Core 2020-12 # 8.2.3.1. Direct References with
        ///   `$ref`](https://json-schema.org/draft/2020-12/json-schema-core.html#ref)
        /// - [Understanding JSON Schema # Structuring a complex schema
        ///   `$ref`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=ref#ref)
        pub const REF: &str = "$ref";
        /// ## `$recursiveAnchor`
        ///
        /// `$recursiveAnchor` is used to dynamically identify a base URI at runtime for
        /// `"$recursiveRef"` by marking where such a calculation can start, and where
        /// it stops.  This keyword MUST NOT affect the base URI of other keywords,
        /// unless they are explicitly defined to rely on it.
        ///
        /// - [Draft 2019-09 Core # 8.2.4.2.2.  Enabling Recursion with
        ///   `$recursiveAnchor`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2.2)
        pub const RECURSIVE_ANCHOR: &str = "$recursiveAnchor";
        /// ## `type`
        ///
        /// The `type` keyword is fundamental to JSON Schema. It specifies the data type
        /// for a schema.
        ///
        /// The type keyword may either be a string or an array:
        ///
        /// If it’s a string, it is the name of one of the basic types above.
        ///
        /// If it is an array, it must be an array of strings, where each string is the
        /// name of one of the basic types, and each element is unique. In this case,
        /// the JSON snippet is valid if it matches any of the given types.
        ///
        /// - [Understanding JSON Schema # Type-specific
        ///   keywords](https://json-schema.org/understanding-json-schema/reference/type.html)
        pub const TYPE: &str = "type";
        /// ## `format`
        /// The format keyword allows for basic semantic identification of certain kinds
        /// of string values that are commonly used. For example, because JSON doesn’t
        /// have a `DateTime` type, dates need to be encoded as strings. format allows
        /// the schema author to indicate that the string value should be interpreted as
        /// a date. By default, format is just an annotation and does not effect
        /// validation.
        ///
        /// - [JSON Schema Core 2020-12 # 7. Vocabularies for Semantic Content With
        ///   `format`](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c)
        /// - [Understanding Json Schema # string Built-in
        ///   Formats](https://json-schema.org/understanding-json-schema/reference/string.html#id7)
        /// - [OpenAPI 3.1 Specification # 4.2
        ///   Format](https://spec.openapis.org/oas/v3.1.0#format)
        pub const FORMAT: &str = "format";
        /// ## `const`
        ///
        /// The `const` keyword is used to restrict a value to a single value.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.1.3.
        ///   `const`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-const)
        /// - [Understanding JSON Schema - Constant
        ///   values](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#constant-values)
        pub const CONST: &str = "const";
        /// ## `$defs`
        ///
        /// The `$defs` keyword reserves a location for schema authors to inline
        /// re-usable JSON Schemas into a more general schema. The keyword does not
        /// directly affect the validation result.
        ///
        /// This keyword's value MUST be an object. Each member value of this object
        /// MUST be a valid JSON Schema.
        ///
        /// - [JSON Schema Core 2020-12 # 8.2.4. Schema Re-Use With
        ///   `$defs`](https://json-schema.org/draft/2020-12/json-schema-core.html#defs)
        /// - [Understanding JSON Schema #
        ///   `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
        pub const DEFS: &str = "$defs";
        /// ## `definitions`
        ///
        /// Legacy definitions from Draft 07 and earlier.
        ///
        /// ## Note
        /// If using JSON Schema 07 or earlier, use this field instead of
        /// [`$defs`](`Keyword::DEFS`).
        ///
        /// - [Understanding JSON Schema #
        ///   `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
        pub const DEFINITIONS_LEGACY: &str = "definitions";
        /// ## `enum`
        ///
        /// An instance validates successfully against this keyword if its value is
        /// equal to one of the elements in this keyword's array value.
        ///
        /// Elements in the array might be of any type, including null.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.1.2.
        ///   `enum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-enum)
        pub const ENUM: &str = "enum";
        /// ## `allOf`
        ///
        /// This keyword's value MUST be a non-empty array. Each item of the array MUST
        /// be a valid JSON Schema.
        ///
        /// The `allOf` keyword acts as an `AND` where each subschema must be valid
        ///
        /// An instance validates successfully against this keyword if it validates
        /// successfully against all schemas defined by this keyword's value.
        ///
        /// - [JSON Schema 2020-12 # 10.2.1.1.
        ///   `allOf`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-allof)
        /// - [Understanding JSON Schema # Schema Composition
        ///   `allOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=allOf#allOf)
        pub const ALL_OF: &str = "allOf";
        /// ## `anyOf`
        ///
        /// This keyword's value MUST be a non-empty array. Each item of the array MUST
        /// be a valid JSON Schema.
        ///
        /// The `anyOf` keyword acts as an `OR` where at least one of the subschemas
        /// must be valid
        ///
        /// An instance validates successfully against this keyword if it validates
        /// successfully against at least one schema defined by this keyword's value.
        /// Note that when annotations are being collected, all subschemas MUST be
        /// examined so that annotations are collected from each subschema that
        /// validates successfully.
        ///
        /// - [JSON Schema Core 2020-12 # 10.2.1.2.
        ///   `anyOf`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.1.2)
        /// - [Understanding JSON Schema # Schema Composition
        ///   `anyOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=anyof#anyOf)
        pub const ANY_OF: &str = "anyOf";
        /// ## `oneOf`
        ///
        /// This keyword's value MUST be a non-empty array. Each item of the array MUST
        /// be a valid JSON Schema.
        ///
        /// The `oneOf` keyword acts as an `XOR` where exactly one of the subschemas
        /// must be valid
        ///
        /// - [JSON Schema Core 2020-12 # 10.2.1.3.
        ///   `oneOf`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-oneof)
        /// - [Understanding JSON Schema # Schema Composition
        ///   `oneOf`](https://json-schema.org/understanding-json-schema/reference/combining.html#oneof)
        pub const ONE_OF: &str = "oneOf";
        /// ## `not`
        ///
        /// This keyword's value MUST be a valid JSON Schema.
        ///
        /// The not keyword declares that an instance validates if it doesn’t validate
        /// against the given subschema.
        ///
        /// An instance is valid against this keyword if it fails to validate
        /// successfully against the schema defined by this keyword.
        ///
        /// - [JSON Schema Core 2020-12
        ///   #](https://json-schema.org/draft/2020-12/json-schema-core.html#name-not)
        /// - [Understanding JSON Schema # Schema Composition
        ///   `not`](https://json-schema.org/understanding-json-schema/reference/combining.html?#id8)
        pub const NOT: &str = "not";
        /// ## `if`
        ///
        /// This keyword's value MUST be a valid JSON Schema.
        ///
        /// This validation outcome of this keyword's subschema has no direct effect on
        /// the overall validation result. Rather, it controls which of the `then` or
        /// `else` keywords are evaluated. Instances that successfully validate against
        /// this keyword's subschema MUST also be valid against the subschema value of
        /// the `then` keyword, if present.
        ///
        /// Instances that fail to validate against this keyword's subschema MUST also
        /// be valid against the subschema value of the `else` keyword, if present.
        ///
        /// - [JSON Schema Core 2020-12 # 10.2.2.1.
        ///   `if`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.2.1)
        pub const IF: &str = "if";
        /// ## `then`
        ///
        /// This keyword's value MUST be a valid JSON Schema.
        ///
        /// When `if` is present, and the instance successfully validates against its
        /// subschema, then validation succeeds against this keyword if the instance
        /// also successfully validates against this keyword's subschema.
        ///
        /// This keyword has no effect when `if` is absent, or when the instance fails
        /// to validate against its subschema. Implementations MUST NOT evaluate the
        /// instance against this keyword, for either validation or annotation
        /// collection purposes, in such cases.
        ///
        /// - [JSON Schema Core 2020-12 # 10.2.2.2.
        ///   `then`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.2.2)
        pub const THEN: &str = "then";
        /// ## `else`
        ///
        /// This keyword's value MUST be a valid JSON Schema.
        ///
        /// When `if` is present, and the instance fails to validate against its
        /// subschema, then validation succeeds against this keyword if the instance
        /// successfully validates against this keyword's subschema.
        ///
        /// This keyword has no effect when `if` is absent, or when the instance
        /// successfully validates against its subschema. Implementations MUST NOT
        /// evaluate the instance against this keyword, for either validation or
        /// annotation collection purposes, in such cases.
        ///
        /// - [JSON Schema Core 2020-12 # 10.2.2.3.
        ///   `else`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-else)
        pub const ELSE: &str = "else";
        /// ## `dependentSchemas`
        ///
        /// This keyword specifies subschemas that are evaluated if the instance is an
        /// object and contains a certain property.
        ///
        /// This keyword's value MUST be an object. Each value in the object MUST be a
        /// valid JSON Schema.
        ///
        /// If the object key is a property in the instance, the entire instance must
        /// validate against the subschema. Its use is dependent on the presence of the
        /// property.
        /// - [JSON Schema Core 2020-12 # 10.2.2.4.
        ///   `dependentSchemas`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dependentschemas)
        pub const DEPENDENT_SCHEMAS: &str = "dependentSchemas";
        /// ## `prefixItems`
        ///
        /// The value of `prefixItems` MUST be a non-empty array of valid JSON Schemas.
        ///
        /// Validation succeeds if each element of the instance validates against the
        /// schema at the same position, if any. This keyword does not constrain the
        /// length of the array. If the array is longer than this keyword's value, this
        /// keyword validates only the prefix of matching length.
        ///
        /// This keyword produces an annotation value which is the largest index to
        /// which this keyword applied a subschema. The value MAY be a boolean true if a
        /// subschema was applied to every index of the instance, such as is produced by
        /// the `items` keyword. This annotation affects the behavior of `items` and
        /// `unevaluatedItems`.
        ///
        /// Omitting this keyword has the same assertion behavior as an empty array.
        ///
        /// - [JSON Schema Core 2020-12 # 10.3.1.1.
        ///   `prefixItems`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-prefixitems)
        pub const PREFIX_ITEMS: &str = "prefixItems";
        /// ## `items`
        ///
        /// The value of `items` MUST be a valid JSON Schema for 2020-12 and the either
        /// a non-empty array of valid JSON Schemas or a valid JSON Schema for 2019-09,
        /// 07.
        ///
        /// This keyword applies its subschema to all instance elements at indexes
        /// greater than the length of the `prefixItems` array in the same schema
        /// object, as reported by the annotation result of that `prefixItems` keyword.
        /// If no such annotation result exists, `items` applies its subschema to all
        /// instance array elements. Note that the behavior of `items` without
        /// `prefixItems` is identical to that of the schema form of `items` in prior
        /// drafts. When `prefixItems` is present, the behavior of `items` is identical
        /// to the former `additionalItems` keyword.
        ///
        /// ## For Draft 2019, 07:
        /// If `items` is a schema, validation succeeds if all elements in the array
        /// successfully validate against that schema.
        ///
        /// If `items` is an array of schemas, validation succeeds if each element of
        /// the instance validates against the schema at the same position, if any.
        /// - [JSON Schema Core 2020-12 # 10.3.1.2.
        ///   `items`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-items)
        /// - [JSON Schema Validation 07 # 6.4.1
        ///   `items`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.4.1)
        pub const ITEMS: &str = "items";
        /// ## `additionalItems`
        ///
        /// The value of `additionalItems` MUST be a valid JSON Schema.
        ///
        /// The behavior of this keyword depends on the presence and annotation result
        /// of `items` within the same schema object.  If `items` is present, and its
        /// annotation result is a number, validation succeeds if every instance element
        /// at an index greater than that number validates against `additionalItems`.
        ///
        /// Otherwise, if `items` is absent or its annotation result is the boolean
        /// true, `additionalItems` MUST be ignored.
        ///
        /// If the `additionalItems` subschema is applied to any positions within the
        /// instance array, it produces an annotation result of boolean true,
        ///
        ///
        /// analogous to the single schema behavior of `items`.  If any
        /// `additionalItems` keyword from any subschema applied to the same instance
        /// location produces an annotation value of true, then the combined result from
        /// these keywords is also true.
        ///
        /// Omitting this keyword has the same assertion behavior as an empty schema.
        ///
        /// Implementations MAY choose to implement or optimize this keyword in another
        /// way that produces the same effect, such as by directly checking for the
        /// presence and size of an `items` array. Implementations that do not support
        /// annotation collection MUST do so.
        ///
        /// - [JSON Schema Core 2019-09 # 9.3.1.2.
        ///   `additionalItems`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-9.3.1.2)
        pub const ADDITIONAL_ITEMS: &str = "additionalItems";
        /// ## `contains`
        ///
        /// The value of this keyword MUST be a valid JSON Schema.
        ///
        /// An array instance is valid against `contains` if at least one of its
        /// elements is valid against the given schema, except when `minContains` is
        /// present and has a value of `0`, in which case an array instance MUST be
        /// considered valid against the `contains` keyword, even if none of its
        /// elements is valid against the given schema.
        ///
        /// This keyword produces an annotation value which is an array of the indexes
        /// to which this keyword validates successfully when applying its subschema, in
        /// ascending order. The value MAY be a boolean "true" if the subschema
        /// validates successfully when applied to every index of the instance. The
        /// annotation MUST be present if the instance array to which this keyword's
        /// schema applies is empty.
        ///
        /// This annotation affects the behavior of `unevaluatedItems` in the
        /// Unevaluated vocabulary, and MAY also be used to implement the `minContains`
        /// and `maxContains` keywords in the Validation vocabulary.
        ///
        /// The subschema MUST be applied to every array element even after the first
        /// match has been found, in order to collect annotations for use by other
        /// keywords. This is to ensure that all possible annotations are collected.
        ///
        ///  - [JSON Schema Core 2020-12 # 10.3.1.3.
        ///    `contains`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-contains)
        pub const CONTAINS: &str = "contains";
        /// ## `properties`
        ///
        /// The value of `properties` MUST be an object. Each value of this object MUST
        /// be a valid JSON Schema.
        ///
        /// Validation succeeds if, for each name that appears in both the instance and
        /// as a name within this keyword's value, the child instance for that name
        /// successfully validates against the corresponding schema.
        ///
        /// The annotation result of this keyword is the set of instance property names
        /// matched by this keyword. This annotation affects the behavior of
        /// `additionalProperties` (in this vocabulary) and `unevaluatedProperties` in
        /// the Unevaluated vocabulary.
        ///
        /// Omitting this keyword has the same assertion behavior as an empty object.
        ///
        /// - [JSON Schema Core 2020-12 # 10.3.2.1.
        ///   properties](https://json-schema.org/draft/2020-12/json-schema-core.html#name-properties)
        pub const PROPERTIES: &str = "properties";
        /// ## `patternProperties`
        ///
        /// Each property name of this object SHOULD be a valid regular expression,
        /// according to the ECMA-262 regular expression dialect. Each property value of
        /// this object MUST be a valid JSON Schema.
        ///
        /// Validation succeeds if, for each instance name that matches any regular
        /// expressions that appear as a property name in this keyword's value, the
        /// child instance for that name successfully validates against each schema that
        /// corresponds to a matching regular expression.
        ///
        /// The annotation result of this keyword is the set of instance property names
        /// matched by this keyword. This annotation affects the behavior of
        /// `additionalProperties` (in this vocabulary) and `unevaluatedProperties` (in
        /// the Unevaluated vocabulary).
        ///
        /// Omitting this keyword has the same assertion behavior as an empty object.
        ///
        /// - [JSON Schema Core 2020-12 # 10.3.2.2.
        ///   `patternProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-patternproperties)
        pub const PATTERN_PROPERTIES: &str = "patternProperties";
        /// ## `additionalProperties`
        ///
        /// The value of `additionalProperties` MUST be a valid JSON Schema.
        ///
        /// The behavior of this keyword depends on the presence and annotation results
        /// of `properties` and `patternProperties` within the same schema object.
        /// Validation with `additionalProperties` applies only to the child values of
        /// instance names that do not appear in the annotation results of either
        /// `properties` or `patternProperties`.
        ///
        /// For all such properties, validation succeeds if the child instance validates
        /// against the `additionalProperties` schema.
        ///
        /// The annotation result of this keyword is the set of instance property names
        /// validated by this keyword's subschema. This annotation affects the behavior
        /// of `unevaluatedProperties` in the Unevaluated vocabulary.
        ///
        /// Omitting this keyword has the same assertion behavior as an empty schema.
        ///
        /// - [JSON Schema Core 2020-12 #
        ///   10.3.2.3.`additionalProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-additionalproperties)
        pub const ADDITIONAL_PROPERTIES: &str = "additionalProperties";
        /// ## `propertyNames`
        ///
        /// The value of `propertyNames` MUST be a valid JSON Schema.
        ///
        /// If the instance is an object, this keyword validates if every property name
        /// in the instance validates against the provided schema. Note the property
        /// name that the schema is testing will always be a string.
        ///
        /// - [JSON Schema Core 2020-12 #
        ///   10.3.2.4.`propertyNames`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-propertynames)
        pub const PROPERTY_NAMES: &str = "propertyNames";
        /// ## `unevaluatedItems`
        ///
        /// The value of `unevaluatedItems` MUST be a valid JSON Schema.
        ///
        /// The behavior of this keyword depends on the annotation results of adjacent
        /// keywords that apply to the instance location being validated. Specifically,
        /// the annotations from `prefixItems`, `items`, and `contains`, which can come
        /// from those keywords when they are adjacent to the `unevaluatedItems`
        /// keyword. Those three annotations, as well as `unevaluatedItems`, can also
        /// result from any and all adjacent in-place applicator (Section 10.2)
        /// keywords. This includes but is not limited to the in-place applicators
        /// defined in this document.
        ///
        /// If no relevant annotations are present, the `unevaluatedItems` subschema
        /// MUST be applied to all locations in the array. If a boolean true value is
        /// present from any of the relevant annotations, `unevaluatedItems` MUST be
        /// ignored. Otherwise, the subschema MUST be applied to any index greater than
        /// the largest annotation value for `prefixItems`, which does not appear in any
        /// annotation value for `contains`.
        ///
        /// This means that `prefixItems`, `items`, `contains`, and all in-place
        /// applicators MUST be evaluated before this keyword can be evaluated. Authors
        /// of extension keywords MUST NOT define an in-place applicator that would need
        /// to be evaluated after this keyword.
        ///
        /// If the `unevaluatedItems` subschema is applied to any positions within the
        /// instance array, it produces an annotation result of boolean true, analogous
        /// to the behavior of `items`. This annotation affects the behavior of
        /// `unevaluatedItems` in parent schemas.
        ///
        /// Omitting this keyword has the same assertion behavior as an empty schema.
        ///
        /// - [JSON Schema Core 2020-12 # 11.2.
        ///   `unevaluatedItems`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluateditems)
        pub const UNEVALUATED_ITEMS: &str = "unevaluatedItems";
        /// ## `unevaluatedProperties`
        ///
        /// The value of `unevaluatedProperties` MUST be a valid JSON Schema.
        ///
        /// The behavior of this keyword depends on the annotation results of adjacent
        /// keywords that apply to the instance location being validated. Specifically,
        /// the annotations from `properties`, `patternProperties`, and
        /// `additionalProperties`, which can come from those keywords when they are
        /// adjacent to the `unevaluatedProperties` keyword. Those three annotations, as
        /// well as `unevaluatedProperties`, can also result from any and all adjacent
        /// in-place applicator (Section 10.2) keywords. This includes but is not
        /// limited to the in-place applicators defined in this document.
        ///
        /// Validation with `unevaluatedProperties` applies only to the child values of
        /// instance names that do not appear in the `properties`, `patternProperties`,
        /// `additionalProperties`, or `unevaluatedProperties` annotation results that
        /// apply to the instance location being validated.
        ///
        /// For all such properties, validation succeeds if the child instance validates
        /// against the `unevaluatedProperties` schema.
        ///
        /// This means that `properties`, `patternProperties`, `additionalProperties`,
        /// and all in-place applicators MUST be evaluated before this keyword can be
        /// evaluated. Authors of extension keywords MUST NOT define an in-place
        /// applicator that would need to be evaluated after this keyword.
        ///
        /// The annotation result of this keyword is the set of instance property names
        /// validated by this keyword's subschema. This annotation affects the behavior
        /// of `unevaluatedProperties` in parent schemas.
        ///
        /// Omitting this keyword has the same assertion behavior as an empty schema.
        ///
        /// - [JSON Schema Core 2020-12 # 11.3.
        ///   `unevaluatedProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties)
        pub const UNEVALUATED_PROPERTIES: &str = "unevaluatedProperties";
        /// ## `multipleOf`
        ///
        /// The value of `multipleOf` MUST be a number, strictly greater than 0.
        ///
        /// A numeric instance is valid only if division by this keyword's value results
        /// in an integer.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.2.1.
        ///   `multipleOf`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-multipleof)
        pub const MULTIPLE_OF: &str = "multipleOf";
        /// #`maximum`
        ///
        /// The value of `maximum` MUST be a number, representing an inclusive upper
        /// limit for a numeric instance.
        ///
        /// If the instance is a number, then this keyword validates only if the
        /// instance is less than or exactly equal to `maximum`.
        /// - [JSON Schema Validation 2020-12 # 6.2.2.
        ///   `maximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maximum)
        pub const MAXIMUM: &str = "maximum";
        /// ## `exclusiveMaximum`
        ///
        /// For JSON Schema drafts 7 and higher, the value of `exclusiveMaximum` MUST be
        /// a number, representing an exclusive upper limit for a numeric instance. For
        /// JSON Schema Draft 4, the value of `exclusiveMaximum` MUST be a boolean.
        ///
        ///
        /// If the instance is a number, then the instance is valid only if it has a
        /// value strictly less than (not equal to) `exclusiveMaximum`.
        /// - [JSON Schema Validation 2020-12 # 6.2.3.
        ///   `exclusiveMaximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusivemaximum)
        pub const EXCLUSIVE_MAXIMUM: &str = "exclusiveMaximum";
        /// ## `minimum`
        ///
        /// The value of `minimum` MUST be a number, representing an inclusive lower
        /// limit for a numeric instance.
        ///
        /// If the instance is a number, then this keyword validates only if the
        /// instance is greater than or exactly equal to `minimum`.
        /// - [JSON Schema Validation 2020-12 # 6.2.4.
        ///   `minimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minimum)
        pub const MINIMUM: &str = "minimum";
        /// ## `exclusiveMinimum`
        ///
        /// For JSON Schema drafts 7 and higher, the value of `exclusiveMinimum` MUST be
        /// a number, representing an exclusive lower limit for a numeric instance.
        ///
        /// If the instance is a number, then the instance is valid only if it has a
        /// value strictly greater than (not equal to) `exclusiveMinimum`.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.2.5.
        ///   `exclusiveMinimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusiveminimum)
        pub const EXCLUSIVE_MINIMUM: &str = "exclusiveMinimum";
        /// ## `maxLength`
        ///
        /// The value of `maxLength` MUST be a non-negative integer.
        ///
        /// A string instance is valid against this keyword if its length is less than,
        /// or equal to, the value of this keyword.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.3.1.
        ///   `maxLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxlength)
        pub const MAX_LENGTH: &str = "maxLength";
        /// ## `minLength`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        ///
        /// A string instance is valid against this keyword if its length is greater
        /// than, or equal to, the value of this keyword.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.3.2.
        ///   `minLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minlength)
        pub const MIN_LENGTH: &str = "minLength";
        /// ## `pattern`
        ///
        /// The value of this keyword MUST be a string. This string SHOULD be a valid
        /// regular expression, according to the ECMA-262 regular expression dialect.
        ///
        /// A string instance is considered valid if the regular expression matches the
        /// instance successfully.
        ///
        /// Regular expressions are not implicitly anchored.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.3.3.
        ///   `pattern`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-pattern)
        pub const PATTERN: &str = "pattern";
        /// ## `maxItems`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        ///
        /// An array instance is valid against `maxItems` if its size is less than, or
        /// equal to, the value of this keyword.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.3.3.
        ///   `maxItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxitems)
        pub const MAX_ITEMS: &str = "maxItems";
        /// ## `minItems`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        ///
        /// An array instance is valid against `minItems` if its size is greater than,
        /// or equal to, the value of this keyword.
        ///
        /// Omitting this keyword has the same behavior as a value of 0.
        /// - [JSON Schema Validation 2020-12 # 6.3.3.
        ///   `minItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minitems)
        pub const MIN_ITEMS: &str = "minItems";
        /// ## `uniqueItems`
        ///
        /// The value of this keyword MUST be a boolean.
        ///
        /// If this keyword has boolean value false, the instance validates
        /// successfully. If it has boolean value true, the instance validates
        /// successfully if all of its elements are unique.
        ///
        /// Omitting this keyword has the same behavior as a value of false.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.4.3.
        ///   `uniqueItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-uniqueitems)
        pub const UNIQUE_ITEMS: &str = "uniqueItems";
        /// ## `maxContains`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        ///
        /// If `contains` is not present within the same schema object, then this
        /// keyword has no effect.
        ///
        /// An instance array is valid against `maxContains` in two ways, depending on
        /// the form of the annotation result of an adjacent `contains` [json-schema]
        /// keyword. The first way is if the annotation result is an array and the
        /// length of that array is less than or equal to the `maxContains` value. The
        /// second way is if the annotation result is a boolean "true" and the instance
        /// array length is less than or equal to the `maxContains` value.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.4.4.
        ///   `maxContains`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxcontains)
        pub const MAX_CONTAINS: &str = "maxContains";
        /// ## `minContains`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        /// If `contains` is not present within the same schema object, then this
        /// keyword has no effect.
        ///
        /// An instance array is valid against `minContains` in two ways, depending on
        /// the form of the annotation result of an adjacent `contains` [json-schema]
        /// keyword. The first way is if the annotation result is an array and the
        /// length of that array is greater than or equal to the `minContains` value.
        /// The second way is if the annotation result is a boolean "true" and the
        /// instance array length is greater than or equal to the `minContains` value.
        ///
        /// A value of `0` is allowed, but is only useful for setting a range of
        /// occurrences from 0 to the value of `maxContains`. A value of 0 causes
        /// `minContains` and `contains` to always pass validation (but validation can
        /// still fail against a `maxContains` keyword).
        ///
        /// Omitting this keyword has the same behavior as a value of `1`.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.4.4.
        ///   `minContains`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-mincontains)
        pub const MIN_CONTAINS: &str = "minContains";
        /// ## `maxProperties`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        ///
        /// An object instance is valid against `maxProperties` if its number of
        /// properties is less than, or equal to, the value of this keyword.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.5.1
        ///   `maxProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxproperties)
        pub const MAX_PROPERTIES: &str = "maxProperties";
        /// ## `minProperties`
        ///
        /// The value of this keyword MUST be a non-negative integer.
        ///
        /// An object instance is valid against `minProperties` if its number of
        /// properties is greater than, or equal to, the value of this keyword.
        ///
        /// Omitting this keyword has the same behavior as a value of `0`.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.5.2
        ///   `minProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minproperties)
        pub const MIN_PROPERTIES: &str = "minProperties";
        /// ## `required`
        ///
        /// The value of this keyword MUST be an array. Elements of this array, if any,
        /// MUST be strings, and MUST be unique.
        ///
        /// An object instance is valid against this keyword if every item in the array
        /// is the name of a property in the instance.
        ///
        /// Omitting this keyword has the same behavior as an empty array.
        ///
        /// - [JSON Schema Validation 2020-12 # 6.5.3
        ///   `required`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-required)
        pub const REQUIRED: &str = "required";
        /// ## `dependentRequired`
        ///
        /// The value of this keyword MUST be an object. Properties in this object, if
        /// any, MUST be arrays. Elements in each array, if any, MUST be strings, and
        /// MUST be unique.
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
        /// - [JSON Schema Validation 2020-12 # 6.5.4
        ///   `dependentRequired`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-dependentrequired)
        pub const DEPENDENT_REQUIRED: &str = "dependentRequired";
        /// ## `contentEncoding`
        ///
        /// If the instance value is a string, this property defines that the string
        /// SHOULD be interpreted as encoded binary data and decoded using the encoding
        /// named by this property.
        ///
        /// Possible values indicating base 16, 32, and 64 encodings with several
        /// variations are listed in [RFC
        /// 4648](https://www.rfc-editor.org/info/rfc4648). Additionally, sections 6.7
        /// and 6.8 of [RFC 2045](https://www.rfc-editor.org/info/rfc2045) provide
        /// encodings used in MIME. This keyword is derived from MIME's
        /// Content-Transfer-Encoding header, which was designed to map binary data into
        /// ASCII characters. It is not related to HTTP's Content-Encoding header, which
        /// is used to encode (e.g. compress or encrypt) the content of HTTP request and
        /// responses.
        ///
        /// As "base64" is defined in both RFCs, the definition from RFC 4648 SHOULD be
        /// assumed unless the string is specifically intended for use in a MIME
        /// context. Note that all of these encodings result in strings consisting only
        /// of 7-bit ASCII characters. Therefore, this keyword has no meaning for
        /// strings containing characters outside of that range.
        ///
        /// If this keyword is absent, but `contentMediaType` is present, this indicates
        /// that the encoding is the identity encoding, meaning that no transformation
        /// was needed in order to represent the content in a UTF-8 string.
        ///
        /// The value of this property MUST be a string.
        ///
        /// - [JSON Schema Validation 2020-12 # 8.3.
        ///   `contentEncoding`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentencoding)
        /// - [Understanding JSON Schema # Media: string-encoding non-JSON data -
        ///   `contentEncoding`](https://json-schema.org/understanding-json-schema/reference/non_json_data.html#id2)
        pub const CONTENT_ENCODING: &str = "contentEncoding";
        /// ## `contentMediaType`
        ///
        /// If the instance is a string, this property indicates the media type of the
        /// contents of the string. If `contentEncoding` is present, this property
        /// describes the decoded string.
        ///
        /// The value of this property MUST be a string, which MUST be a media type, as
        /// defined by [RFC 2046](https://www.rfc-editor.org/info/rfc2046).
        /// - [JSON Schema Validation 2020-12 # 8.4.
        ///   `contentMediaType`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentmediatype)
        pub const CONTENT_MEDIA_TYPE: &str = "contentMediaType";
        /// ## `contentSchema`
        ///
        /// If the instance is a string, and if `contentMediaType` is present, this
        /// property contains a schema which describes the structure of the string.
        ///
        /// This keyword MAY be used with any media type that can be mapped into JSON
        /// Schema's data model.
        ///
        /// The value of this property MUST be a valid JSON schema. It SHOULD be ignored
        /// if `contentMediaType` is not present
        ///
        /// - [JSON Schema Validation 2020-12 # 8.5.
        ///   `contentSchema`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentschema)
        pub const CONTENT_SCHEMA: &str = "contentSchema";
        /// ## `examples`
        ///
        /// The value of this keyword MUST be an array. There are no restrictions placed
        /// on the values within the array. When multiple occurrences of this keyword
        /// are applicable to a single sub-instance, implementations MUST provide a flat
        /// array of all values rather than an array of arrays.
        ///
        /// This keyword can be used to provide sample JSON values associated with a
        /// particular schema, for the purpose of illustrating usage. It is RECOMMENDED
        /// that these values be valid against the associated schema.
        ///
        /// Implementations MAY use the value(s) of "default", if present, as an
        /// additional example. If `examples` is absent, "default" MAY still be used in
        /// this manner.
        ///
        /// - [JSON Schema Validation 2020-12 # 9.5
        ///   `examples`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-examples)
        pub const EXAMPLES: &str = "examples";
    }
    pub use consts::*;
    pub mod spec {
        use enum_dispatch::enum_dispatch;
        use grill_core::Key;
        pub mod schema {
            use grill_core::Key;
            use crate::{alias, CompileError, EvaluateError, Report, Specification};
            use crate::keyword::{Compile, Evaluate, Keyword};
            pub struct Schema<S, K: Key> {
                _marker: std::marker::PhantomData<(S, K)>,
            }
            impl<S, K> Keyword<S, K> for Schema<S, K>
            where
                S: Specification<K>,
                K: Key,
            {
                fn compile<'i>(
                    &self,
                    compile: alias::Compile<S, K>,
                ) -> Option<Result<(), alias::CompileError<S, K>>> {
                    ::core::panicking::panic("not yet implemented")
                }
                fn evaluate<'v>(
                    &self,
                    eval: alias::Evaluate<S, K>,
                ) -> Result<(), alias::EvaluateError<S, K>> {
                    ::core::panicking::panic("not yet implemented")
                }
            }
        }
        pub enum Spec<S, K: Key> {
            Schema(schema::Schema<S, K>),
        }
    }
    pub mod compile {
        use super::*;
        /// Context for [`Keyword::compile`].
        pub trait Compile<K>
        where
            K: Key,
        {
            /// Retrieves a schema from the store by [`AbsoluteUri`].
            fn schema(&self, uri: &AbsoluteUri) -> Option<K>;
            /// Returns a mutable reference to [`Numbers`] cache.
            fn numbers(&mut self) -> &mut Numbers;
            /// Parses a JSON number into an [`Arc<BigRational>`](`BigRational`) and
            /// stores it in the [`Numbers`] cache if it is not already present.
            /// Otherwise, the existing [`BigRational`] is returned.
            fn number(
                &mut self,
                number: &Number,
            ) -> Result<Arc<BigRational>, grill_core::big::ParseError>;
            /// Returns a mutable reference to [`Values`] cache.
            fn values(&mut self) -> &mut Values;
            /// If `value` is already in the [`Values`] cache, the existing
            /// `Arc<Value>` is cloned and returned. Otherwise, `value` is inserted
            /// as an `Arc<Value>`, cloned, and returned.
            fn value(&mut self, value: &Value) -> Arc<Value>;
            /// Returns a reference to [`Sources`].
            fn sources(&self) -> &Sources;
            /// Retrieves a [`Source`] from the store by [`AbsoluteUri`], if a
            /// [`Link`](grill_core::lang::source::Link) exists.
            fn source(&self, uri: &AbsoluteUri) -> Option<Source>;
        }
        /// Context for [`Keyword::compile`].
        pub struct Context<W, K> {
            pub(crate) schemas: Schemas<CompiledSchema<W, K>, K>,
            pub(crate) sources: Sources,
            pub(crate) numbers: Numbers,
            pub(crate) values: Values,
        }
    }
    pub use compile::Compile;
    pub mod eval {
        use super::*;
        /// Context for [`Keyword::evaluate`].
        pub trait Evaluate<K: Key> {
            fn schema(&self, uri: &AbsoluteUri) -> Option<K>;
        }
        /// Context for [`Keyword::evaluate`].
        pub struct Context {}
    }
    pub use eval::Evaluate;
    pub trait EvaluateKeyword {}
    use spec::Spec;
    pub trait Keyword<S, K>: Send + Debug + Clone
    where
        S: Specification<K>,
        K: Key,
    {
        fn compile<'i>(
            &self,
            compile: alias::Compile<S, K>,
        ) -> Option<Result<(), alias::CompileError<S, K>>>;
        fn evaluate<'v>(
            &self,
            eval: alias::Evaluate<S, K>,
        ) -> Result<(), alias::EvaluateError<S, K>>;
    }
    impl<S, K: Key> ::core::convert::From<schema::Schema<S, K>> for Spec<S, K> {
        fn from(v: schema::Schema<S, K>) -> Spec<S, K> {
            Spec::Schema(v)
        }
    }
    impl<S, K: Key> ::core::convert::TryInto<schema::Schema<S, K>> for Spec<S, K> {
        type Error = &'static str;
        fn try_into(
            self,
        ) -> ::core::result::Result<
            schema::Schema<S, K>,
            <Self as ::core::convert::TryInto<schema::Schema<S, K>>>::Error,
        > {
            match self {
                Spec::Schema(v) => Ok(v),
            }
        }
    }
    impl<S, K: Key> Keyword<S, K> for Spec<S, K> {
        #[inline]
        fn compile<'i>(
            &self,
            __enum_dispatch_arg_0: alias::Compile<S, K>,
        ) -> Option<Result<(), alias::CompileError<S, K>>> {
            match self {
                Spec::Schema(inner) => Keyword::<S, K>::compile(inner, __enum_dispatch_arg_0),
            }
        }
        #[inline]
        fn evaluate<'v>(
            &self,
            __enum_dispatch_arg_0: alias::Evaluate<S, K>,
        ) -> Result<(), alias::EvaluateError<S, K>> {
            match self {
                Spec::Schema(inner) => Keyword::<S, K>::evaluate(inner, __enum_dispatch_arg_0),
            }
        }
    }
    pub enum Actual {
        Bool,
        Number,
        String,
        Array,
        Object,
        Null,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Actual {
        #[inline]
        fn clone(&self) -> Actual {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Actual {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Actual::Bool => "Bool",
                    Actual::Number => "Number",
                    Actual::String => "String",
                    Actual::Array => "Array",
                    Actual::Object => "Object",
                    Actual::Null => "Null",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Actual {}
    impl ::core::fmt::Display for Actual {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match *self {
                Actual::Bool => ::core::fmt::Display::fmt("Bool", f),
                Actual::Number => ::core::fmt::Display::fmt("Number", f),
                Actual::String => ::core::fmt::Display::fmt("String", f),
                Actual::Array => ::core::fmt::Display::fmt("Array", f),
                Actual::Object => ::core::fmt::Display::fmt("Object", f),
                Actual::Null => ::core::fmt::Display::fmt("Null", f),
            }
        }
    }
    impl Actual {
        pub fn from_value(value: &Value) -> Self {
            Self::from(value)
        }
    }
    impl From<&Value> for Actual {
        fn from(value: &Value) -> Self {
            match value {
                Value::Null => Self::Null,
                Value::Bool(_) => Self::Bool,
                Value::Number(_) => Self::Number,
                Value::String(_) => Self::String,
                Value::Array(_) => Self::Array,
                Value::Object(_) => Self::Object,
            }
        }
    }
    /// The expected type of a [`Value`].
    pub enum Expectated {
        /// Expected a null value
        Null,
        /// Expected a boolean
        Bool,
        /// Expected a number
        Number,
        /// Expected a string
        String,
        /// Execpted an array
        Array,
        /// Expected an object
        Object,
        /// Expected any of the types in the slice
        AnyOf(&'static [Expectated]),
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Expectated {
        #[inline]
        fn clone(&self) -> Expectated {
            let _: ::core::clone::AssertParamIsClone<&'static [Expectated]>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Expectated {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Expectated::Null => ::core::fmt::Formatter::write_str(f, "Null"),
                Expectated::Bool => ::core::fmt::Formatter::write_str(f, "Bool"),
                Expectated::Number => ::core::fmt::Formatter::write_str(f, "Number"),
                Expectated::String => ::core::fmt::Formatter::write_str(f, "String"),
                Expectated::Array => ::core::fmt::Formatter::write_str(f, "Array"),
                Expectated::Object => ::core::fmt::Formatter::write_str(f, "Object"),
                Expectated::AnyOf(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AnyOf", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Expectated {}
    impl fmt::Display for Expectated {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expectated::Bool => f.write_fmt(format_args!("Bool")),
                Expectated::Number => f.write_fmt(format_args!("Number")),
                Expectated::String => f.write_fmt(format_args!("String")),
                Expectated::Array => f.write_fmt(format_args!("Array")),
                Expectated::Object => f.write_fmt(format_args!("Object")),
                Expectated::AnyOf(anyof) => {
                    f.write_fmt(format_args!("["))?;
                    for (i, expected) in anyof.iter().enumerate() {
                        if i > 0 {
                            f.write_fmt(format_args!(", "))?;
                        }
                        f.write_fmt(format_args!("{0}", expected))?;
                    }
                    f.write_fmt(format_args!("]"))
                }
                Expectated::Null => f.write_fmt(format_args!("Null")),
            }
        }
    }
    /// A [`Value`] was not of the expected type.
    #[snafu(
        display("expected value of type {expected}, found {actual}"),
        module,
        visibility(pub)
    )]
    pub struct InvalidTypeError {
        /// The expected type of value.
        pub expected: Expectated,
        /// The actual value.
        pub value: Box<Value>,
        pub actual: Actual,
        pub backtrace: snafu::Backtrace,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InvalidTypeError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "InvalidTypeError",
                "expected",
                &self.expected,
                "value",
                &self.value,
                "actual",
                &self.actual,
                "backtrace",
                &&self.backtrace,
            )
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for InvalidTypeError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                Self { .. } => "InvalidTypeError",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                Self { .. } => ::core::option::Option::None,
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                Self { .. } => ::core::option::Option::None,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for InvalidTypeError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                Self { ref backtrace, .. } => ::snafu::AsBacktrace::as_backtrace(backtrace),
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for InvalidTypeError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                Self {
                    ref actual,
                    ref backtrace,
                    ref expected,
                    ref value,
                } => __snafu_display_formatter.write_fmt(format_args!(
                    "expected value of type {1}, found {0}",
                    actual, expected
                )),
            }
        }
    }
    pub mod invalid_type_error {
        use super::*;
        ///SNAFU context selector for the `InvalidTypeError` error
        pub struct InvalidTypeSnafu<__T0, __T1, __T2> {
            #[allow(missing_docs)]
            pub expected: __T0,
            #[allow(missing_docs)]
            pub value: __T1,
            #[allow(missing_docs)]
            pub actual: __T2,
        }
        #[automatically_derived]
        impl<__T0: ::core::fmt::Debug, __T1: ::core::fmt::Debug, __T2: ::core::fmt::Debug>
            ::core::fmt::Debug for InvalidTypeSnafu<__T0, __T1, __T2>
        {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "InvalidTypeSnafu",
                    "expected",
                    &self.expected,
                    "value",
                    &self.value,
                    "actual",
                    &&self.actual,
                )
            }
        }
        #[automatically_derived]
        impl<
                __T0: ::core::marker::Copy,
                __T1: ::core::marker::Copy,
                __T2: ::core::marker::Copy,
            > ::core::marker::Copy for InvalidTypeSnafu<__T0, __T1, __T2>
        {
        }
        #[automatically_derived]
        impl<
                __T0: ::core::clone::Clone,
                __T1: ::core::clone::Clone,
                __T2: ::core::clone::Clone,
            > ::core::clone::Clone for InvalidTypeSnafu<__T0, __T1, __T2>
        {
            #[inline]
            fn clone(&self) -> InvalidTypeSnafu<__T0, __T1, __T2> {
                InvalidTypeSnafu {
                    expected: ::core::clone::Clone::clone(&self.expected),
                    value: ::core::clone::Clone::clone(&self.value),
                    actual: ::core::clone::Clone::clone(&self.actual),
                }
            }
        }
        impl<__T0, __T1, __T2> InvalidTypeSnafu<__T0, __T1, __T2> {
            ///Consume the selector and return the associated error
            #[must_use]
            #[track_caller]
            pub fn build(self) -> InvalidTypeError
            where
                __T0: ::core::convert::Into<Expectated>,
                __T1: ::core::convert::Into<Box<Value>>,
                __T2: ::core::convert::Into<Actual>,
            {
                InvalidTypeError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    expected: ::core::convert::Into::into(self.expected),
                    value: ::core::convert::Into::into(self.value),
                    actual: ::core::convert::Into::into(self.actual),
                }
            }
            ///Consume the selector and return a `Result` with the associated error
            #[track_caller]
            pub fn fail<__T>(self) -> ::core::result::Result<__T, InvalidTypeError>
            where
                __T0: ::core::convert::Into<Expectated>,
                __T1: ::core::convert::Into<Box<Value>>,
                __T2: ::core::convert::Into<Actual>,
            {
                ::core::result::Result::Err(self.build())
            }
        }
        impl<__T0, __T1, __T2> ::snafu::IntoError<InvalidTypeError> for InvalidTypeSnafu<__T0, __T1, __T2>
        where
            InvalidTypeError: ::snafu::Error + ::snafu::ErrorCompat,
            __T0: ::core::convert::Into<Expectated>,
            __T1: ::core::convert::Into<Box<Value>>,
            __T2: ::core::convert::Into<Actual>,
        {
            type Source = ::snafu::NoneError;
            #[track_caller]
            fn into_error(self, error: Self::Source) -> InvalidTypeError {
                InvalidTypeError {
                    backtrace: ::snafu::GenerateImplicitData::generate(),
                    expected: ::core::convert::Into::into(self.expected),
                    value: ::core::convert::Into::into(self.value),
                    actual: ::core::convert::Into::into(self.actual),
                }
            }
        }
    }
}
pub mod report {
    pub use self::{basic::Basic, flag::Flag, verbose::Verbose};
    use grill_uri::AbsoluteUri;
    use jsonptr::Pointer;
    use serde::{Deserialize, Deserializer, Serialize};
    use serde_json::Value;
    use std::{borrow::Cow, fmt::Debug};
    /// The output structure of a [`Report`].
    ///
    /// [JSON Schema Core 2020-12 #12.4 Output
    /// Structure](https://json-schema.org/draft/2020-12/json-schema-core#name-output-structure)
    #[serde(rename_all = "lowercase")]
    #[repr(u8)]
    pub enum Output {
        /// A concise structure which only contains a single `"valid"` `bool` field.
        ///
        /// `Flag` may have a positive impact on performance as
        /// [`Keyword`](`crate::keyword::Keyword`)s are expected to short circuit
        /// and return errors as soon as possible.
        ///
        /// # Example
        /// ```json
        /// { "valid": false }
        /// ```
        ///
        /// - [JSON Schema Core 2020-12 # 12.4.1
        ///   Flag](https://json-schema.org/draft/2020-12/json-schema-core.html#name-flag)
        Flag = 1,
        /// A flat list of output units.
        ///
        /// # Example
        /// ```json
        /// {
        ///   "valid": false,
        ///   "errors": [
        ///     {
        ///       "keywordLocation": "",
        ///       "instanceLocation": "",
        ///       "error": "A subschema had errors."
        ///     },
        ///     {
        ///       "keywordLocation": "/items/$ref",
        ///       "absoluteKeywordLocation":
        ///         "https://example.com/polygon#/$defs/point",
        ///       "instanceLocation": "/1",
        ///       "error": "A subschema had errors."
        ///     },
        ///     {
        ///       "keywordLocation": "/items/$ref/required",
        ///       "absoluteKeywordLocation":
        ///         "https://example.com/polygon#/$defs/point/required",
        ///       "instanceLocation": "/1",
        ///       "error": "Required property 'y' not found."
        ///     },
        ///     {
        ///       "keywordLocation": "/items/$ref/additionalProperties",
        ///       "absoluteKeywordLocation":
        ///         "https://example.com/polygon#/$defs/point/additionalProperties",
        ///       "instanceLocation": "/1/z",
        ///       "error": "Additional property 'z' found but was invalid."
        ///     },
        ///     {
        ///       "keywordLocation": "/minItems",
        ///       "instanceLocation": "",
        ///       "error": "Expected at least 3 items but found 2"
        ///     }
        ///   ]
        /// }
        /// ```
        /// - [JSON Schema Core 2020-12 # 12.4.2
        ///   Basic](https://json-schema.org/draft/2020-12/json-schema-core#name-basic)
        Basic = 2,
        /// A tree structure of output units.
        ///
        /// - [JSON Schema Core 2020-12 # 12.4.4
        ///   Verbose](https://json-schema.org/draft/2020-12/json-schema-core#name-verbose)
        Verbose = 4,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Output {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    Output::Flag => "Flag",
                    Output::Basic => "Basic",
                    Output::Verbose => "Verbose",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Output {
        #[inline]
        fn clone(&self) -> Output {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for Output {}
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Output {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Output::Flag => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Output",
                        0u32,
                        "flag",
                    ),
                    Output::Basic => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Output",
                        1u32,
                        "basic",
                    ),
                    Output::Verbose => _serde::Serializer::serialize_unit_variant(
                        __serializer,
                        "Output",
                        2u32,
                        "verbose",
                    ),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Output {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 3",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "flag" => _serde::__private::Ok(__Field::__field0),
                            "basic" => _serde::__private::Ok(__Field::__field1),
                            "verbose" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"flag" => _serde::__private::Ok(__Field::__field0),
                            b"basic" => _serde::__private::Ok(__Field::__field1),
                            b"verbose" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Output>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Output;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum Output")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Output::Flag)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Output::Basic)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(Output::Verbose)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["flag", "basic", "verbose"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Output",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Output>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Output {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Output {
        #[inline]
        fn eq(&self, other: &Output) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for Output {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for Output {
        #[inline]
        fn partial_cmp(&self, other: &Output) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for Output {
        #[inline]
        fn cmp(&self, other: &Output) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Output {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    impl ::core::fmt::Display for Output {
        fn fmt(
            &self,
            f: &mut ::core::fmt::Formatter,
        ) -> ::core::result::Result<(), ::core::fmt::Error> {
            match *self {
                Output::Flag => ::core::fmt::Display::fmt("Flag", f),
                Output::Basic => ::core::fmt::Display::fmt("Basic", f),
                Output::Verbose => ::core::fmt::Display::fmt("Verbose", f),
            }
        }
    }
    ///  
    #[serde(untagged)]
    pub enum Annotation<'v> {
        Schema(AbsoluteUri),
        Unknown(Cow<'v, Value>),
    }
    #[automatically_derived]
    impl<'v> ::core::clone::Clone for Annotation<'v> {
        #[inline]
        fn clone(&self) -> Annotation<'v> {
            match self {
                Annotation::Schema(__self_0) => {
                    Annotation::Schema(::core::clone::Clone::clone(__self_0))
                }
                Annotation::Unknown(__self_0) => {
                    Annotation::Unknown(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    #[automatically_derived]
    impl<'v> ::core::fmt::Debug for Annotation<'v> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Annotation::Schema(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Schema", &__self_0)
                }
                Annotation::Unknown(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Unknown", &__self_0)
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'v> _serde::Serialize for Annotation<'v> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Annotation::Schema(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                    Annotation::Unknown(ref __field0) => {
                        _serde::Serialize::serialize(__field0, __serializer)
                    }
                }
            }
        }
    };
    impl<'v, 'de> Deserialize<'de> for Annotation<'v> {
        fn deserialize<D>(deserializer: D) -> Result<Annotation<'v>, D::Error>
        where
            D: Deserializer<'de>,
        {
            serde_json::Value::deserialize(deserializer)
                .map(Cow::Owned)
                .map(Self::Unknown)
        }
    }
    pub enum Error<'v> {
        X(Cow<'v, str>),
    }
    #[automatically_derived]
    impl<'v> ::core::fmt::Debug for Error<'v> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::X(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "X", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl<'v> ::core::clone::Clone for Error<'v> {
        #[inline]
        fn clone(&self) -> Error<'v> {
            match self {
                Error::X(__self_0) => Error::X(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'v> _serde::Serialize for Error<'v> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Error::X(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "Error",
                        0u32,
                        "X",
                        __field0,
                    ),
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de, 'v> _serde::Deserialize<'de> for Error<'v> {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                _serde::de::Unexpected::Unsigned(__value),
                                &"variant index 0 <= i < 1",
                            )),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "X" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                __value, VARIANTS,
                            )),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"X" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                ))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, 'v> {
                    marker: _serde::__private::PhantomData<Error<'v>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, 'v> _serde::de::Visitor<'de> for __Visitor<'de, 'v> {
                    type Value = Error<'v>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "enum Error")
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => _serde::__private::Result::map(
                                _serde::de::VariantAccess::newtype_variant::<Cow<'v, str>>(
                                    __variant,
                                ),
                                Error::X,
                            ),
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["X"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Error",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Error<'v>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub enum Report<A, E> {
        Flag(Flag),
        Basic(Basic<A, E>),
        Verbose(Verbose<A, E>),
    }
    #[automatically_derived]
    impl<A: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Report<A, E> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Report::Flag(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Flag", &__self_0)
                }
                Report::Basic(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Basic", &__self_0)
                }
                Report::Verbose(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Verbose", &__self_0)
                }
            }
        }
    }
    #[automatically_derived]
    impl<A: ::core::clone::Clone, E: ::core::clone::Clone> ::core::clone::Clone for Report<A, E> {
        #[inline]
        fn clone(&self) -> Report<A, E> {
            match self {
                Report::Flag(__self_0) => Report::Flag(::core::clone::Clone::clone(__self_0)),
                Report::Basic(__self_0) => Report::Basic(::core::clone::Clone::clone(__self_0)),
                Report::Verbose(__self_0) => Report::Verbose(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    impl<A, E> std::fmt::Display for Report<A, E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<A, E> std::error::Error for Report<A, E>
    where
        A: Debug,
        E: Debug,
    {
    }
    impl<A, E> Report<A, E> {
        pub fn instance_location(&self) -> Option<&jsonptr::Pointer> {
            match self {
                Report::Flag(_) => None,
                Report::Basic(b) => b.instance_location(),
                Report::Verbose(v) => Some(v.instance_location()),
            }
        }
        pub fn keyword_location(&self) -> Option<&jsonptr::Pointer> {
            match self {
                Report::Flag(_) => None,
                Report::Basic(b) => b.keyword_location(),
                Report::Verbose(v) => Some(v.keyword_location()),
            }
        }
        pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
            match self {
                Report::Flag(_) => None,
                Report::Basic(b) => b.absolute_keyword_location(),
                Report::Verbose(v) => Some(v.absolute_keyword_location()),
            }
        }
        pub fn is_valid(&self) -> bool {
            match self {
                Report::Flag(f) => f.is_valid(),
                Report::Basic(b) => b.is_valid(),
                Report::Verbose(v) => v.is_valid(),
            }
        }
        pub fn assess(&mut self, location: Location) -> Assess<'_, A, E> {
            match self {
                Report::Flag(f) => f.assess(location),
                Report::Basic(b) => b.assess(location),
                Report::Verbose(v) => v.assess(location),
            }
        }
    }
    impl<'de, A, E> Deserialize<'de> for Report<A, E> {
        fn deserialize<D>(deserializer: D) -> Result<Report<A, E>, D::Error>
        where
            D: Deserializer<'de>,
        {
            ::core::panicking::panic("not yet implemented")
        }
    }
    impl<A, E> Serialize for Report<A, E> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            ::core::panicking::panic("not yet implemented")
        }
    }
    pub mod flag {
        use grill_uri::AbsoluteUri;
        use serde::{Deserialize, Serialize};
        use serde_json::{Map, Value};
        use super::{Assess, Location};
        pub struct Flag {
            /// The validity of the schema.
            pub valid: bool,
            /// Additional properties.
            #[serde(default, flatten)]
            pub additional_properties: Option<Map<String, Value>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Flag {
            #[inline]
            fn clone(&self) -> Flag {
                Flag {
                    valid: ::core::clone::Clone::clone(&self.valid),
                    additional_properties: ::core::clone::Clone::clone(&self.additional_properties),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Flag {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Flag",
                    "valid",
                    &self.valid,
                    "additional_properties",
                    &&self.additional_properties,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Flag {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state =
                        _serde::Serializer::serialize_map(__serializer, _serde::__private::None)?;
                    _serde::ser::SerializeMap::serialize_entry(
                        &mut __serde_state,
                        "valid",
                        &self.valid,
                    )?;
                    _serde::Serialize::serialize(
                        &&self.additional_properties,
                        _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                    )?;
                    _serde::ser::SerializeMap::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Flag {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field<'de> {
                        __field0,
                        __other(_serde::__private::de::Content<'de>),
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field<'de>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_bool<__E>(
                            self,
                            __value: bool,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ))
                        }
                        fn visit_i8<__E>(
                            self,
                            __value: i8,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I8(__value),
                            ))
                        }
                        fn visit_i16<__E>(
                            self,
                            __value: i16,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ))
                        }
                        fn visit_i32<__E>(
                            self,
                            __value: i32,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ))
                        }
                        fn visit_i64<__E>(
                            self,
                            __value: i64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ))
                        }
                        fn visit_u8<__E>(
                            self,
                            __value: u8,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U8(__value),
                            ))
                        }
                        fn visit_u16<__E>(
                            self,
                            __value: u16,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ))
                        }
                        fn visit_u32<__E>(
                            self,
                            __value: u32,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ))
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ))
                        }
                        fn visit_f32<__E>(
                            self,
                            __value: f32,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ))
                        }
                        fn visit_f64<__E>(
                            self,
                            __value: f64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ))
                        }
                        fn visit_char<__E>(
                            self,
                            __value: char,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ))
                        }
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::Unit,
                            ))
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "valid" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    let __value = _serde::__private::de::Content::String(
                                        _serde::__private::ToString::to_string(__value),
                                    );
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"valid" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    let __value =
                                        _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_str<__E>(
                            self,
                            __value: &'de str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "valid" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    let __value = _serde::__private::de::Content::Str(__value);
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_bytes<__E>(
                            self,
                            __value: &'de [u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"valid" => _serde::__private::Ok(__Field::__field0),
                                _ => {
                                    let __value = _serde::__private::de::Content::Bytes(__value);
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Flag>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Flag;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Flag")
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<bool> =
                                _serde::__private::None;
                            let mut __collect = _serde::__private::Vec::<
                                _serde::__private::Option<(
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                )>,
                            >::new();
                            while let _serde::__private::Some(__key) =
                                _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "valid",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__other(__name) => {
                                        __collect.push(_serde::__private::Some((
                                            __name,
                                            _serde::de::MapAccess::next_value(&mut __map)?,
                                        )));
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("valid")?
                                }
                            };
                            let __field1: Option<Map<String, Value>> =
                                _serde::de::Deserialize::deserialize(
                                    _serde::__private::de::FlatMapDeserializer(
                                        &mut __collect,
                                        _serde::__private::PhantomData,
                                    ),
                                )?;
                            _serde::__private::Ok(Flag {
                                valid: __field0,
                                additional_properties: __field1,
                            })
                        }
                    }
                    _serde::Deserializer::deserialize_map(
                        __deserializer,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Flag>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl Flag {
            pub fn assess<A, E>(&mut self, location: Location) -> Assess<'_, A, E> {
                Assess::Flag(self)
            }
            pub fn is_valid(&self) -> bool {
                self.valid
            }
            pub fn into_owned(self) -> Flag {
                Flag {
                    valid: self.valid,
                    additional_properties: self.additional_properties,
                }
            }
        }
    }
    pub mod basic {
        use super::*;
        pub struct Basic<A, E> {
            valid: bool,
            assessments: Vec<Assessment<A, E>>,
        }
        #[automatically_derived]
        impl<A: ::core::clone::Clone, E: ::core::clone::Clone> ::core::clone::Clone for Basic<A, E> {
            #[inline]
            fn clone(&self) -> Basic<A, E> {
                Basic {
                    valid: ::core::clone::Clone::clone(&self.valid),
                    assessments: ::core::clone::Clone::clone(&self.assessments),
                }
            }
        }
        #[automatically_derived]
        impl<A: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Basic<A, E> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Basic",
                    "valid",
                    &self.valid,
                    "assessments",
                    &&self.assessments,
                )
            }
        }
        impl<A, E> Basic<A, E> {
            pub fn assessments(&self) -> &[Assessment<A, E>] {
                &self.assessments
            }
            pub fn assess(&mut self, location: Location) -> Assess<'_, A, E> {
                ::core::panicking::panic("not yet implemented")
            }
            pub fn is_valid(&self) -> bool {
                self.valid
            }
            /// Returns a reference to the first [`Assessment`] in the list, if any.
            pub fn first(&self) -> Option<&Assessment<A, E>> {
                self.assessments.first()
            }
            /// Returns a mutable reference to the first [`Assessment`] in the list,
            /// if any.
            pub fn first_mut(&mut self) -> Option<&mut Assessment<A, E>> {
                self.assessments.first_mut()
            }
            /// Returns a reference to the `instanceLocation`, in the form of a JSON
            /// [`Pointer`], of the first [`Assessment`] in the list, if any.
            pub fn instance_location(&self) -> Option<&Pointer> {
                self.first().map(|a| a.instance_location())
            }
            /// Returns a reference to the `keywordLocation`, in the form of a JSON
            /// [`Pointer`], of the first [`Assessment`] in the list, if any.
            pub fn keyword_location(&self) -> Option<&Pointer> {
                self.first().map(|a| a.keyword_location())
            }
            /// Returns a reference to the `absoluteKeywordLocation`, in the form of a
            /// [`AbsoluteUri`], of the first [`Assessment`] in the list, if any.
            pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
                self.first().map(|a| a.absolute_keyword_location())
            }
        }
        impl<A, E> Basic<A, E> {
            pub fn new(location: Location) -> Self {
                Self {
                    assessments: <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([Assessment::Annotation {
                            annotation: None,
                            location: location,
                        }]),
                    ),
                    valid: true,
                }
            }
        }
        pub enum Assessment<A, E> {
            Annotation {
                annotation: Option<A>,
                #[serde(flatten)]
                location: Location,
            },
            Error {
                error: Option<E>,
                #[serde(flatten)]
                location: Location,
            },
        }
        #[automatically_derived]
        impl<A: ::core::clone::Clone, E: ::core::clone::Clone> ::core::clone::Clone for Assessment<A, E> {
            #[inline]
            fn clone(&self) -> Assessment<A, E> {
                match self {
                    Assessment::Annotation {
                        annotation: __self_0,
                        location: __self_1,
                    } => Assessment::Annotation {
                        annotation: ::core::clone::Clone::clone(__self_0),
                        location: ::core::clone::Clone::clone(__self_1),
                    },
                    Assessment::Error {
                        error: __self_0,
                        location: __self_1,
                    } => Assessment::Error {
                        error: ::core::clone::Clone::clone(__self_0),
                        location: ::core::clone::Clone::clone(__self_1),
                    },
                }
            }
        }
        #[automatically_derived]
        impl<A: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Assessment<A, E> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Assessment::Annotation {
                        annotation: __self_0,
                        location: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Annotation",
                        "annotation",
                        __self_0,
                        "location",
                        &__self_1,
                    ),
                    Assessment::Error {
                        error: __self_0,
                        location: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f, "Error", "error", __self_0, "location", &__self_1,
                    ),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<A, E> _serde::Serialize for Assessment<A, E>
            where
                A: _serde::Serialize,
                E: _serde::Serialize,
            {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Assessment::Annotation {
                            ref annotation,
                            ref location,
                        } => {
                            #[doc(hidden)]
                            struct __EnumFlatten<'__a, A: '__a, E: '__a>
                            where
                                A: _serde::Serialize,
                                E: _serde::Serialize,
                            {
                                data: (&'__a Option<A>, &'__a Location),
                                phantom: _serde::__private::PhantomData<Assessment<A, E>>,
                            }
                            impl<'__a, A: '__a, E: '__a> _serde::Serialize for __EnumFlatten<'__a, A, E>
                            where
                                A: _serde::Serialize,
                                E: _serde::Serialize,
                            {
                                fn serialize<__S>(
                                    &self,
                                    __serializer: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    let (annotation, location) = self.data;
                                    let mut __serde_state = _serde::Serializer::serialize_map(
                                        __serializer,
                                        _serde::__private::None,
                                    )?;
                                    _serde::ser::SerializeMap::serialize_entry(
                                        &mut __serde_state,
                                        "annotation",
                                        annotation,
                                    )?;
                                    _serde::Serialize::serialize(
                                        &location,
                                        _serde::__private::ser::FlatMapSerializer(
                                            &mut __serde_state,
                                        ),
                                    )?;
                                    _serde::ser::SerializeMap::end(__serde_state)
                                }
                            }
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Assessment",
                                0u32,
                                "Annotation",
                                &__EnumFlatten {
                                    data: (annotation, location),
                                    phantom: _serde::__private::PhantomData::<Assessment<A, E>>,
                                },
                            )
                        }
                        Assessment::Error {
                            ref error,
                            ref location,
                        } => {
                            #[doc(hidden)]
                            struct __EnumFlatten<'__a, A: '__a, E: '__a>
                            where
                                A: _serde::Serialize,
                                E: _serde::Serialize,
                            {
                                data: (&'__a Option<E>, &'__a Location),
                                phantom: _serde::__private::PhantomData<Assessment<A, E>>,
                            }
                            impl<'__a, A: '__a, E: '__a> _serde::Serialize for __EnumFlatten<'__a, A, E>
                            where
                                A: _serde::Serialize,
                                E: _serde::Serialize,
                            {
                                fn serialize<__S>(
                                    &self,
                                    __serializer: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    let (error, location) = self.data;
                                    let mut __serde_state = _serde::Serializer::serialize_map(
                                        __serializer,
                                        _serde::__private::None,
                                    )?;
                                    _serde::ser::SerializeMap::serialize_entry(
                                        &mut __serde_state,
                                        "error",
                                        error,
                                    )?;
                                    _serde::Serialize::serialize(
                                        &location,
                                        _serde::__private::ser::FlatMapSerializer(
                                            &mut __serde_state,
                                        ),
                                    )?;
                                    _serde::ser::SerializeMap::end(__serde_state)
                                }
                            }
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "Assessment",
                                1u32,
                                "Error",
                                &__EnumFlatten {
                                    data: (error, location),
                                    phantom: _serde::__private::PhantomData::<Assessment<A, E>>,
                                },
                            )
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, A, E> _serde::Deserialize<'de> for Assessment<A, E>
            where
                A: _serde::Deserialize<'de>,
                E: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "variant identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                )),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "Annotation" => _serde::__private::Ok(__Field::__field0),
                                "Error" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Err(_serde::de::Error::unknown_variant(
                                    __value, VARIANTS,
                                )),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"Annotation" => _serde::__private::Ok(__Field::__field0),
                                b"Error" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(
                                        __value, VARIANTS,
                                    ))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, A, E>
                    where
                        A: _serde::Deserialize<'de>,
                        E: _serde::Deserialize<'de>,
                    {
                        marker: _serde::__private::PhantomData<Assessment<A, E>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de, A, E> _serde::de::Visitor<'de> for __Visitor<'de, A, E>
                    where
                        A: _serde::Deserialize<'de>,
                        E: _serde::Deserialize<'de>,
                    {
                        type Value = Assessment<A, E>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "enum Assessment")
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field<'de> {
                                        __field0,
                                        __other(_serde::__private::de::Content<'de>),
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                        type Value = __Field<'de>;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result
                                        {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "field identifier",
                                            )
                                        }
                                        fn visit_bool<__E>(
                                            self,
                                            __value: bool,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::Bool(__value),
                                            ))
                                        }
                                        fn visit_i8<__E>(
                                            self,
                                            __value: i8,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I8(__value),
                                            ))
                                        }
                                        fn visit_i16<__E>(
                                            self,
                                            __value: i16,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I16(__value),
                                            ))
                                        }
                                        fn visit_i32<__E>(
                                            self,
                                            __value: i32,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I32(__value),
                                            ))
                                        }
                                        fn visit_i64<__E>(
                                            self,
                                            __value: i64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I64(__value),
                                            ))
                                        }
                                        fn visit_u8<__E>(
                                            self,
                                            __value: u8,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U8(__value),
                                            ))
                                        }
                                        fn visit_u16<__E>(
                                            self,
                                            __value: u16,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U16(__value),
                                            ))
                                        }
                                        fn visit_u32<__E>(
                                            self,
                                            __value: u32,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U32(__value),
                                            ))
                                        }
                                        fn visit_u64<__E>(
                                            self,
                                            __value: u64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U64(__value),
                                            ))
                                        }
                                        fn visit_f32<__E>(
                                            self,
                                            __value: f32,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::F32(__value),
                                            ))
                                        }
                                        fn visit_f64<__E>(
                                            self,
                                            __value: f64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::F64(__value),
                                            ))
                                        }
                                        fn visit_char<__E>(
                                            self,
                                            __value: char,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::Char(__value),
                                            ))
                                        }
                                        fn visit_unit<__E>(
                                            self,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::Unit,
                                            ))
                                        }
                                        fn visit_str<__E>(
                                            self,
                                            __value: &str,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "annotation" => {
                                                    _serde::__private::Ok(__Field::__field0)
                                                }
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::String(
                                                            _serde::__private::ToString::to_string(
                                                                __value,
                                                            ),
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                        fn visit_bytes<__E>(
                                            self,
                                            __value: &[u8],
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"annotation" => {
                                                    _serde::__private::Ok(__Field::__field0)
                                                }
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::ByteBuf(
                                                            __value.to_vec(),
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                        fn visit_borrowed_str<__E>(
                                            self,
                                            __value: &'de str,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "annotation" => {
                                                    _serde::__private::Ok(__Field::__field0)
                                                }
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::Str(
                                                            __value,
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                        fn visit_borrowed_bytes<__E>(
                                            self,
                                            __value: &'de [u8],
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"annotation" => {
                                                    _serde::__private::Ok(__Field::__field0)
                                                }
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::Bytes(
                                                            __value,
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                    }
                                    impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                        #[inline]
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_identifier(
                                                __deserializer,
                                                __FieldVisitor,
                                            )
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de, A, E>
                                    where
                                        A: _serde::Deserialize<'de>,
                                        E: _serde::Deserialize<'de>,
                                    {
                                        marker: _serde::__private::PhantomData<Assessment<A, E>>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de, A, E> _serde::de::Visitor<'de> for __Visitor<'de, A, E>
                                    where
                                        A: _serde::Deserialize<'de>,
                                        E: _serde::Deserialize<'de>,
                                    {
                                        type Value = Assessment<A, E>;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result
                                        {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "struct variant Assessment::Annotation",
                                            )
                                        }
                                        #[inline]
                                        fn visit_map<__A>(
                                            self,
                                            mut __map: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::MapAccess<'de>,
                                        {
                                            let mut __field0: _serde::__private::Option<Option<A>> =
                                                _serde::__private::None;
                                            let mut __collect = _serde::__private::Vec::<
                                                _serde::__private::Option<(
                                                    _serde::__private::de::Content,
                                                    _serde::__private::de::Content,
                                                )>,
                                            >::new(
                                            );
                                            while let _serde::__private::Some(__key) =
                                                _serde::de::MapAccess::next_key::<__Field>(
                                                    &mut __map,
                                                )?
                                            {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private::Option::is_some(
                                                            &__field0,
                                                        ) {
                                                            return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("annotation")) ;
                                                        }
                                                        __field0 = _serde::__private::Some(
                                                            _serde::de::MapAccess::next_value::<
                                                                Option<A>,
                                                            >(
                                                                &mut __map
                                                            )?,
                                                        );
                                                    }
                                                    __Field::__other(__name) => {
                                                        __collect.push(_serde::__private::Some((
                                                            __name,
                                                            _serde::de::MapAccess::next_value(
                                                                &mut __map,
                                                            )?,
                                                        )));
                                                    }
                                                }
                                            }
                                            let __field0 = match __field0 {
                                                _serde::__private::Some(__field0) => __field0,
                                                _serde::__private::None => {
                                                    _serde::__private::de::missing_field(
                                                        "annotation",
                                                    )?
                                                }
                                            };
                                            let __field1: Location =
                                                _serde::de::Deserialize::deserialize(
                                                    _serde::__private::de::FlatMapDeserializer(
                                                        &mut __collect,
                                                        _serde::__private::PhantomData,
                                                    ),
                                                )?;
                                            _serde::__private::Ok(Assessment::Annotation {
                                                annotation: __field0,
                                                location: __field1,
                                            })
                                        }
                                    }
                                    impl<'de, A, E> _serde::de::DeserializeSeed<'de> for __Visitor<'de, A, E>
                                    where
                                        A: _serde::Deserialize<'de>,
                                        E: _serde::Deserialize<'de>,
                                    {
                                        type Value = Assessment<A, E>;
                                        fn deserialize<__D>(
                                            self,
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self::Value, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_map(
                                                __deserializer,
                                                self,
                                            )
                                        }
                                    }
                                    _serde::de::VariantAccess::newtype_variant_seed(
                                        __variant,
                                        __Visitor {
                                            marker: _serde::__private::PhantomData::<
                                                Assessment<A, E>,
                                            >,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    #[allow(non_camel_case_types)]
                                    #[doc(hidden)]
                                    enum __Field<'de> {
                                        __field0,
                                        __other(_serde::__private::de::Content<'de>),
                                    }
                                    #[doc(hidden)]
                                    struct __FieldVisitor;
                                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                        type Value = __Field<'de>;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result
                                        {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "field identifier",
                                            )
                                        }
                                        fn visit_bool<__E>(
                                            self,
                                            __value: bool,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::Bool(__value),
                                            ))
                                        }
                                        fn visit_i8<__E>(
                                            self,
                                            __value: i8,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I8(__value),
                                            ))
                                        }
                                        fn visit_i16<__E>(
                                            self,
                                            __value: i16,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I16(__value),
                                            ))
                                        }
                                        fn visit_i32<__E>(
                                            self,
                                            __value: i32,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I32(__value),
                                            ))
                                        }
                                        fn visit_i64<__E>(
                                            self,
                                            __value: i64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::I64(__value),
                                            ))
                                        }
                                        fn visit_u8<__E>(
                                            self,
                                            __value: u8,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U8(__value),
                                            ))
                                        }
                                        fn visit_u16<__E>(
                                            self,
                                            __value: u16,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U16(__value),
                                            ))
                                        }
                                        fn visit_u32<__E>(
                                            self,
                                            __value: u32,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U32(__value),
                                            ))
                                        }
                                        fn visit_u64<__E>(
                                            self,
                                            __value: u64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::U64(__value),
                                            ))
                                        }
                                        fn visit_f32<__E>(
                                            self,
                                            __value: f32,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::F32(__value),
                                            ))
                                        }
                                        fn visit_f64<__E>(
                                            self,
                                            __value: f64,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::F64(__value),
                                            ))
                                        }
                                        fn visit_char<__E>(
                                            self,
                                            __value: char,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::Char(__value),
                                            ))
                                        }
                                        fn visit_unit<__E>(
                                            self,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            _serde::__private::Ok(__Field::__other(
                                                _serde::__private::de::Content::Unit,
                                            ))
                                        }
                                        fn visit_str<__E>(
                                            self,
                                            __value: &str,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "error" => _serde::__private::Ok(__Field::__field0),
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::String(
                                                            _serde::__private::ToString::to_string(
                                                                __value,
                                                            ),
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                        fn visit_bytes<__E>(
                                            self,
                                            __value: &[u8],
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"error" => {
                                                    _serde::__private::Ok(__Field::__field0)
                                                }
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::ByteBuf(
                                                            __value.to_vec(),
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                        fn visit_borrowed_str<__E>(
                                            self,
                                            __value: &'de str,
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                "error" => _serde::__private::Ok(__Field::__field0),
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::Str(
                                                            __value,
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                        fn visit_borrowed_bytes<__E>(
                                            self,
                                            __value: &'de [u8],
                                        ) -> _serde::__private::Result<Self::Value, __E>
                                        where
                                            __E: _serde::de::Error,
                                        {
                                            match __value {
                                                b"error" => {
                                                    _serde::__private::Ok(__Field::__field0)
                                                }
                                                _ => {
                                                    let __value =
                                                        _serde::__private::de::Content::Bytes(
                                                            __value,
                                                        );
                                                    _serde::__private::Ok(__Field::__other(__value))
                                                }
                                            }
                                        }
                                    }
                                    impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                                        #[inline]
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_identifier(
                                                __deserializer,
                                                __FieldVisitor,
                                            )
                                        }
                                    }
                                    #[doc(hidden)]
                                    struct __Visitor<'de, A, E>
                                    where
                                        A: _serde::Deserialize<'de>,
                                        E: _serde::Deserialize<'de>,
                                    {
                                        marker: _serde::__private::PhantomData<Assessment<A, E>>,
                                        lifetime: _serde::__private::PhantomData<&'de ()>,
                                    }
                                    impl<'de, A, E> _serde::de::Visitor<'de> for __Visitor<'de, A, E>
                                    where
                                        A: _serde::Deserialize<'de>,
                                        E: _serde::Deserialize<'de>,
                                    {
                                        type Value = Assessment<A, E>;
                                        fn expecting(
                                            &self,
                                            __formatter: &mut _serde::__private::Formatter,
                                        ) -> _serde::__private::fmt::Result
                                        {
                                            _serde::__private::Formatter::write_str(
                                                __formatter,
                                                "struct variant Assessment::Error",
                                            )
                                        }
                                        #[inline]
                                        fn visit_map<__A>(
                                            self,
                                            mut __map: __A,
                                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                                        where
                                            __A: _serde::de::MapAccess<'de>,
                                        {
                                            let mut __field0: _serde::__private::Option<Option<E>> =
                                                _serde::__private::None;
                                            let mut __collect = _serde::__private::Vec::<
                                                _serde::__private::Option<(
                                                    _serde::__private::de::Content,
                                                    _serde::__private::de::Content,
                                                )>,
                                            >::new(
                                            );
                                            while let _serde::__private::Some(__key) =
                                                _serde::de::MapAccess::next_key::<__Field>(
                                                    &mut __map,
                                                )?
                                            {
                                                match __key {
                                                    __Field::__field0 => {
                                                        if _serde::__private::Option::is_some(
                                                            &__field0,
                                                        ) {
                                                            return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("error")) ;
                                                        }
                                                        __field0 = _serde::__private::Some(
                                                            _serde::de::MapAccess::next_value::<
                                                                Option<E>,
                                                            >(
                                                                &mut __map
                                                            )?,
                                                        );
                                                    }
                                                    __Field::__other(__name) => {
                                                        __collect.push(_serde::__private::Some((
                                                            __name,
                                                            _serde::de::MapAccess::next_value(
                                                                &mut __map,
                                                            )?,
                                                        )));
                                                    }
                                                }
                                            }
                                            let __field0 = match __field0 {
                                                _serde::__private::Some(__field0) => __field0,
                                                _serde::__private::None => {
                                                    _serde::__private::de::missing_field("error")?
                                                }
                                            };
                                            let __field1: Location =
                                                _serde::de::Deserialize::deserialize(
                                                    _serde::__private::de::FlatMapDeserializer(
                                                        &mut __collect,
                                                        _serde::__private::PhantomData,
                                                    ),
                                                )?;
                                            _serde::__private::Ok(Assessment::Error {
                                                error: __field0,
                                                location: __field1,
                                            })
                                        }
                                    }
                                    impl<'de, A, E> _serde::de::DeserializeSeed<'de> for __Visitor<'de, A, E>
                                    where
                                        A: _serde::Deserialize<'de>,
                                        E: _serde::Deserialize<'de>,
                                    {
                                        type Value = Assessment<A, E>;
                                        fn deserialize<__D>(
                                            self,
                                            __deserializer: __D,
                                        ) -> _serde::__private::Result<Self::Value, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::Deserializer::deserialize_map(
                                                __deserializer,
                                                self,
                                            )
                                        }
                                    }
                                    _serde::de::VariantAccess::newtype_variant_seed(
                                        __variant,
                                        __Visitor {
                                            marker: _serde::__private::PhantomData::<
                                                Assessment<A, E>,
                                            >,
                                            lifetime: _serde::__private::PhantomData,
                                        },
                                    )
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["Annotation", "Error"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "Assessment",
                        VARIANTS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Assessment<A, E>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl<A, E> Assessment<A, E> {
            pub fn instance_location(&self) -> &jsonptr::Pointer {
                match self {
                    Self::Annotation { location, .. } => &location.instance,
                    Self::Error { location, .. } => &location.instance,
                }
            }
            pub fn keyword_location(&self) -> &jsonptr::Pointer {
                match self {
                    Self::Annotation { location, .. } => &location.keyword,
                    Self::Error { location, .. } => &location.keyword,
                }
            }
            pub fn absolute_keyword_location(&self) -> &super::AbsoluteUri {
                match self {
                    Self::Annotation { location, .. } => &location.absolute_keyword,
                    Self::Error { location, .. } => &location.absolute_keyword,
                }
            }
            pub fn annotation(&self) -> Option<&A> {
                let Self::Annotation { annotation: a, .. } = self else {
                    return None;
                };
                a.as_ref()
            }
            pub fn set_annotation(&mut self, annotation: A) -> Option<A> {
                let Self::Annotation { annotation: a, .. } = self else {
                    return None;
                };
                a.replace(annotation)
            }
            pub fn take_annotation(&mut self) -> Option<A> {
                let Self::Annotation { annotation: a, .. } = self else {
                    return None;
                };
                a.take()
            }
            pub fn error(&self) -> Option<&E> {
                let Self::Error { error: e, .. } = self else {
                    return None;
                };
                e.as_ref()
            }
            pub fn set_error(&mut self, error: E) -> Option<E> {
                let Self::Error { error: e, .. } = self else {
                    return None;
                };
                e.replace(error)
            }
            pub fn take_error(&mut self) -> Option<E> {
                let Self::Error { error: e, .. } = self else {
                    return None;
                };
                e.take()
            }
        }
    }
    pub mod verbose {
        use super::AbsoluteUri;
        pub struct Verbose<A, E> {
            #[serde(flatten)]
            pub location: super::Location,
            #[serde(flatten)]
            pub detail: Assessment<A, E>,
        }
        #[automatically_derived]
        impl<A: ::core::clone::Clone, E: ::core::clone::Clone> ::core::clone::Clone for Verbose<A, E> {
            #[inline]
            fn clone(&self) -> Verbose<A, E> {
                Verbose {
                    location: ::core::clone::Clone::clone(&self.location),
                    detail: ::core::clone::Clone::clone(&self.detail),
                }
            }
        }
        #[automatically_derived]
        impl<A: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Verbose<A, E> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Verbose",
                    "location",
                    &self.location,
                    "detail",
                    &&self.detail,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<A, E> _serde::Serialize for Verbose<A, E>
            where
                A: _serde::Serialize,
                E: _serde::Serialize,
            {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state =
                        _serde::Serializer::serialize_map(__serializer, _serde::__private::None)?;
                    _serde::Serialize::serialize(
                        &&self.location,
                        _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                    )?;
                    _serde::Serialize::serialize(
                        &&self.detail,
                        _serde::__private::ser::FlatMapSerializer(&mut __serde_state),
                    )?;
                    _serde::ser::SerializeMap::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, A, E> _serde::Deserialize<'de> for Verbose<A, E>
            where
                A: _serde::Deserialize<'de>,
                E: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field<'de> {
                        __other(_serde::__private::de::Content<'de>),
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field<'de>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_bool<__E>(
                            self,
                            __value: bool,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ))
                        }
                        fn visit_i8<__E>(
                            self,
                            __value: i8,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I8(__value),
                            ))
                        }
                        fn visit_i16<__E>(
                            self,
                            __value: i16,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ))
                        }
                        fn visit_i32<__E>(
                            self,
                            __value: i32,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ))
                        }
                        fn visit_i64<__E>(
                            self,
                            __value: i64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ))
                        }
                        fn visit_u8<__E>(
                            self,
                            __value: u8,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U8(__value),
                            ))
                        }
                        fn visit_u16<__E>(
                            self,
                            __value: u16,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ))
                        }
                        fn visit_u32<__E>(
                            self,
                            __value: u32,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ))
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ))
                        }
                        fn visit_f32<__E>(
                            self,
                            __value: f32,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ))
                        }
                        fn visit_f64<__E>(
                            self,
                            __value: f64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ))
                        }
                        fn visit_char<__E>(
                            self,
                            __value: char,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ))
                        }
                        fn visit_unit<__E>(self) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private::Ok(__Field::__other(
                                _serde::__private::de::Content::Unit,
                            ))
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => {
                                    let __value = _serde::__private::de::Content::String(
                                        _serde::__private::ToString::to_string(__value),
                                    );
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => {
                                    let __value =
                                        _serde::__private::de::Content::ByteBuf(__value.to_vec());
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_str<__E>(
                            self,
                            __value: &'de str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => {
                                    let __value = _serde::__private::de::Content::Str(__value);
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_bytes<__E>(
                            self,
                            __value: &'de [u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                _ => {
                                    let __value = _serde::__private::de::Content::Bytes(__value);
                                    _serde::__private::Ok(__Field::__other(__value))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de, A, E>
                    where
                        A: _serde::Deserialize<'de>,
                        E: _serde::Deserialize<'de>,
                    {
                        marker: _serde::__private::PhantomData<Verbose<A, E>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de, A, E> _serde::de::Visitor<'de> for __Visitor<'de, A, E>
                    where
                        A: _serde::Deserialize<'de>,
                        E: _serde::Deserialize<'de>,
                    {
                        type Value = Verbose<A, E>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Verbose")
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __collect = _serde::__private::Vec::<
                                _serde::__private::Option<(
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                )>,
                            >::new();
                            while let _serde::__private::Some(__key) =
                                _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                            {
                                match __key {
                                    __Field::__other(__name) => {
                                        __collect.push(_serde::__private::Some((
                                            __name,
                                            _serde::de::MapAccess::next_value(&mut __map)?,
                                        )));
                                    }
                                }
                            }
                            let __field0: super::Location = _serde::de::Deserialize::deserialize(
                                _serde::__private::de::FlatMapDeserializer(
                                    &mut __collect,
                                    _serde::__private::PhantomData,
                                ),
                            )?;
                            let __field1: Assessment<A, E> = _serde::de::Deserialize::deserialize(
                                _serde::__private::de::FlatMapDeserializer(
                                    &mut __collect,
                                    _serde::__private::PhantomData,
                                ),
                            )?;
                            _serde::__private::Ok(Verbose {
                                location: __field0,
                                detail: __field1,
                            })
                        }
                    }
                    _serde::Deserializer::deserialize_map(
                        __deserializer,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Verbose<A, E>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[serde(untagged)]
        pub enum Assessment<A, E> {
            Annotation {
                annotations: Vec<Verbose<A, E>>,
                annotation: Option<A>,
            },
            Error {
                errors: Vec<Verbose<A, E>>,
                error: Option<E>,
            },
        }
        #[automatically_derived]
        impl<A: ::core::clone::Clone, E: ::core::clone::Clone> ::core::clone::Clone for Assessment<A, E> {
            #[inline]
            fn clone(&self) -> Assessment<A, E> {
                match self {
                    Assessment::Annotation {
                        annotations: __self_0,
                        annotation: __self_1,
                    } => Assessment::Annotation {
                        annotations: ::core::clone::Clone::clone(__self_0),
                        annotation: ::core::clone::Clone::clone(__self_1),
                    },
                    Assessment::Error {
                        errors: __self_0,
                        error: __self_1,
                    } => Assessment::Error {
                        errors: ::core::clone::Clone::clone(__self_0),
                        error: ::core::clone::Clone::clone(__self_1),
                    },
                }
            }
        }
        #[automatically_derived]
        impl<A: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Assessment<A, E> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Assessment::Annotation {
                        annotations: __self_0,
                        annotation: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Annotation",
                        "annotations",
                        __self_0,
                        "annotation",
                        &__self_1,
                    ),
                    Assessment::Error {
                        errors: __self_0,
                        error: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f, "Error", "errors", __self_0, "error", &__self_1,
                    ),
                }
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<A, E> _serde::Serialize for Assessment<A, E>
            where
                A: _serde::Serialize,
                E: _serde::Serialize,
            {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        Assessment::Annotation {
                            ref annotations,
                            ref annotation,
                        } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                __serializer,
                                "Assessment",
                                0 + 1 + 1,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "annotations",
                                annotations,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "annotation",
                                annotation,
                            )?;
                            _serde::ser::SerializeStruct::end(__serde_state)
                        }
                        Assessment::Error {
                            ref errors,
                            ref error,
                        } => {
                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                __serializer,
                                "Assessment",
                                0 + 1 + 1,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "errors",
                                errors,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "error",
                                error,
                            )?;
                            _serde::ser::SerializeStruct::end(__serde_state)
                        }
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de, A, E> _serde::Deserialize<'de> for Assessment<A, E>
            where
                A: _serde::Deserialize<'de>,
                E: _serde::Deserialize<'de>,
            {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    let __content =
                        <_serde::__private::de::Content as _serde::Deserialize>::deserialize(
                            __deserializer,
                        )?;
                    let __deserializer =
                        _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(
                            &__content,
                        );
                    if let _serde::__private::Ok(__ok) = {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "annotations" => _serde::__private::Ok(__Field::__field0),
                                    "annotation" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"annotations" => _serde::__private::Ok(__Field::__field0),
                                    b"annotation" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de, A, E>
                        where
                            A: _serde::Deserialize<'de>,
                            E: _serde::Deserialize<'de>,
                        {
                            marker: _serde::__private::PhantomData<Assessment<A, E>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de, A, E> _serde::de::Visitor<'de> for __Visitor<'de, A, E>
                        where
                            A: _serde::Deserialize<'de>,
                            E: _serde::Deserialize<'de>,
                        {
                            type Value = Assessment<A, E>;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant Assessment::Annotation",
                                )
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<Verbose<A, E>>> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Option<A>> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("annotations")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<Verbose<A, E>>,
                                                >(
                                                    &mut __map
                                                )?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("annotation")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Option<A>>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("annotations")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("annotation")?
                                    }
                                };
                                _serde::__private::Ok(Assessment::Annotation {
                                    annotations: __field0,
                                    annotation: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["annotations", "annotation"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Assessment<A, E>>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    } {
                        return _serde::__private::Ok(__ok);
                    }
                    if let _serde::__private::Ok(__ok) = {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "errors" => _serde::__private::Ok(__Field::__field0),
                                    "error" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"errors" => _serde::__private::Ok(__Field::__field0),
                                    b"error" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de, A, E>
                        where
                            A: _serde::Deserialize<'de>,
                            E: _serde::Deserialize<'de>,
                        {
                            marker: _serde::__private::PhantomData<Assessment<A, E>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de, A, E> _serde::de::Visitor<'de> for __Visitor<'de, A, E>
                        where
                            A: _serde::Deserialize<'de>,
                            E: _serde::Deserialize<'de>,
                        {
                            type Value = Assessment<A, E>;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct variant Assessment::Error",
                                )
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<Vec<Verbose<A, E>>> =
                                    _serde::__private::None;
                                let mut __field1: _serde::__private::Option<Option<E>> =
                                    _serde::__private::None;
                                while let _serde::__private::Some(__key) =
                                    _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                                {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("errors")) ;
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<Verbose<A, E>>,
                                                >(
                                                    &mut __map
                                                )?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde :: __private :: Err (< __A :: Error as _serde :: de :: Error > :: duplicate_field ("error")) ;
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<Option<E>>(
                                                    &mut __map,
                                                )?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(
                                                &mut __map
                                            )?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("errors")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("error")?
                                    }
                                };
                                _serde::__private::Ok(Assessment::Error {
                                    errors: __field0,
                                    error: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["errors", "error"];
                        _serde::Deserializer::deserialize_any(
                            __deserializer,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Assessment<A, E>>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    } {
                        return _serde::__private::Ok(__ok);
                    }
                    _serde::__private::Err(_serde::de::Error::custom(
                        "data did not match any variant of untagged enum Assessment",
                    ))
                }
            }
        };
        impl<A, E> Verbose<A, E> {
            pub fn is_valid(&self) -> bool {
                match self.detail {
                    Assessment::Annotation { .. } => true,
                    _ => false,
                }
            }
            /// Sets the annotation of the current assessment and previous
            /// assessment, if it existed.
            ///
            /// If the current assessment is not `Asssessment::Annotation` then
            /// `annotation` will be ignored.
            pub fn set_annotation(&mut self, annotation: A) -> Option<A> {
                if let Assessment::Annotation { annotation: a, .. } = &mut self.detail {
                    a.replace(annotation)
                } else {
                    None
                }
            }
            pub fn set_error(&mut self, error: E) -> Option<E> {
                if let Assessment::Error { error: e, .. } = &mut self.detail {
                    e.replace(error)
                } else {
                    self.detail = Assessment::Error {
                        errors: Vec::new(),
                        error: Some(error),
                    };
                    None
                }
            }
            pub fn instance_location(&self) -> &jsonptr::Pointer {
                ::core::panicking::panic("not yet implemented")
            }
            pub fn keyword_location(&self) -> &jsonptr::Pointer {
                &self.location.keyword
            }
            pub fn absolute_keyword_location(&self) -> &AbsoluteUri {
                &self.location.absolute_keyword
            }
            pub fn assess<'r>(&'r self, location: super::Location) -> super::Assess<'r, A, E> {
                ::core::panicking::panic("not yet implemented")
            }
        }
    }
    /// A trait implemented by types that can be converted into an owned type.
    pub trait IntoOwned {
        /// The owned type.
        type Owned: 'static;
        /// Consumes `self`, returning `Self::Owned`.
        fn into_owned(self) -> Self::Owned;
    }
    /// A keyword location within a [`Report`]
    pub struct Location {
        /// The location of the instance within the JSON document.
        #[serde(rename = "instanceLocation")]
        pub instance: jsonptr::Pointer,
        /// The location of the keyword within the JSON Schema.
        #[serde(rename = "keywordLocation")]
        pub keyword: jsonptr::Pointer,
        /// The absolute location of the keyword within the JSON Schema.
        #[serde(rename = "absoluteKeywordLocation")]
        pub absolute_keyword: AbsoluteUri,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Location {
        #[inline]
        fn clone(&self) -> Location {
            Location {
                instance: ::core::clone::Clone::clone(&self.instance),
                keyword: ::core::clone::Clone::clone(&self.keyword),
                absolute_keyword: ::core::clone::Clone::clone(&self.absolute_keyword),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Location {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Location",
                "instance",
                &self.instance,
                "keyword",
                &self.keyword,
                "absolute_keyword",
                &&self.absolute_keyword,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Location {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Location",
                    false as usize + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "instanceLocation",
                    &self.instance,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "keywordLocation",
                    &self.keyword,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "absoluteKeywordLocation",
                    &self.absolute_keyword,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Location {
            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "field identifier")
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "instanceLocation" => _serde::__private::Ok(__Field::__field0),
                            "keywordLocation" => _serde::__private::Ok(__Field::__field1),
                            "absoluteKeywordLocation" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"instanceLocation" => _serde::__private::Ok(__Field::__field0),
                            b"keywordLocation" => _serde::__private::Ok(__Field::__field1),
                            b"absoluteKeywordLocation" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Location>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Location;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(__formatter, "struct Location")
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<jsonptr::Pointer>(
                            &mut __seq,
                        )? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Location with 3 elements",
                                ))
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<jsonptr::Pointer>(
                            &mut __seq,
                        )? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Location with 3 elements",
                                ))
                            }
                        };
                        let __field2 =
                            match _serde::de::SeqAccess::next_element::<AbsoluteUri>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Location with 3 elements",
                                        ),
                                    )
                                }
                            };
                        _serde::__private::Ok(Location {
                            instance: __field0,
                            keyword: __field1,
                            absolute_keyword: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<jsonptr::Pointer> =
                            _serde::__private::None;
                        let mut __field1: _serde::__private::Option<jsonptr::Pointer> =
                            _serde::__private::None;
                        let mut __field2: _serde::__private::Option<AbsoluteUri> =
                            _serde::__private::None;
                        while let _serde::__private::Some(__key) =
                            _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                        {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "instanceLocation",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<jsonptr::Pointer>(
                                            &mut __map,
                                        )?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "keywordLocation",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<jsonptr::Pointer>(
                                            &mut __map,
                                        )?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "absoluteKeywordLocation",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<AbsoluteUri>(
                                            &mut __map,
                                        )?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("instanceLocation")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("keywordLocation")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("absoluteKeywordLocation")?
                            }
                        };
                        _serde::__private::Ok(Location {
                            instance: __field0,
                            keyword: __field1,
                            absolute_keyword: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "instanceLocation",
                    "keywordLocation",
                    "absoluteKeywordLocation",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Location",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Location>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl Location {
        /// Returns the instance location as a JSON Pointer.
        pub fn instance(&self) -> &jsonptr::Pointer {
            &self.instance
        }
        /// Returns the keyword location as a JSON Pointer.
        pub fn keyword(&self) -> &jsonptr::Pointer {
            &self.keyword
        }
        /// Returns the absolute keyword location as an [`AbsoluteUri`].
        pub fn absolute_keyword(&self) -> &AbsoluteUri {
            &self.absolute_keyword
        }
    }
    /// A mutable reference to a node in a [`Report`].
    pub enum Assess<'r, A, E> {
        /// A concise structure which only contains a single `"valid"` `bool` field.
        ///
        /// - [JSON Schema Core 2020-12 # 12.4.1
        ///   `Flag`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-flag)
        Flag(&'r mut Flag),
        /// A flat list of output units.
        ///
        /// - [JSON Schema Core 2020-12 # 12.4.2
        ///   Basic](https://json-schema.org/draft/2020-12/json-schema-core#name-basic)
        Basic(&'r mut basic::Assessment<A, E>),
        /// A tree structure of output units.
        ///
        /// - [JSON Schema Core 2020-12 # 12.4.4
        ///   Verbose](https://json-schema.org/draft/2020-12/json-schema-core#name-verbose)
        Verbose(&'r mut Verbose<A, E>),
    }
    impl<'r, A, E> Assess<'r, A, E> {
        /// Depending on the variant, may set the [`Annotation`] of the current assessment
        /// and return the previous value, if present.
        ///
        /// - [`Basic`] and [`Verbose`]: sets the [`Annotation`]
        /// - [`Flag`]: discards `annotation`
        pub fn annotate(&mut self, annotation: A) -> Option<A> {
            match self {
                Assess::Flag(_) => None,
                Assess::Basic(b) => b.set_annotation(annotation),
                Assess::Verbose(v) => v.set_annotation(annotation),
            }
        }
        /// For all variants, sets `valid` to `false`. Depending on the variant, may also
        /// set the [`Error`] of the current assessment and the previous value, if present.
        ///
        /// - [`Flag`]: discards `error`
        /// - [`Basic`]: sets the [`Error`]
        /// - [`Verbose`]: sets the [`Error`]
        pub fn fail(&mut self, error: E) {
            match self {
                Assess::Flag(flag) => {
                    flag.valid = false;
                }
                Assess::Basic(basic) => {
                    basic.set_error(error);
                }
                Assess::Verbose(verbose) => {
                    verbose.set_error(error);
                }
            }
        }
    }
    pub trait Translate<E> {
        fn translate(&self, error: E) -> E;
    }
    pub struct Translated<'t, T, A, E> {
        pub report: Report<A, E>,
        pub translator: &'t T,
    }
}
pub mod schema {
    pub mod dialect {
        use std::{ops::ControlFlow, sync::Arc};
        use grill_uri::AbsoluteUri;
        use jsonptr::Pointer;
        use serde_json::Value;
        pub struct Dialect<W, E> {
            pub uri: AbsoluteUri,
            pub keywords: Vec<W>,
            pub sources: Vec<(AbsoluteUri, Arc<Value>)>,
            pub embedded_schema_paths: Vec<E>,
        }
        #[automatically_derived]
        impl<W: ::core::fmt::Debug, E: ::core::fmt::Debug> ::core::fmt::Debug for Dialect<W, E> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Dialect",
                    "uri",
                    &self.uri,
                    "keywords",
                    &self.keywords,
                    "sources",
                    &self.sources,
                    "embedded_schema_paths",
                    &&self.embedded_schema_paths,
                )
            }
        }
        #[automatically_derived]
        impl<W: ::core::clone::Clone, E: ::core::clone::Clone> ::core::clone::Clone for Dialect<W, E> {
            #[inline]
            fn clone(&self) -> Dialect<W, E> {
                Dialect {
                    uri: ::core::clone::Clone::clone(&self.uri),
                    keywords: ::core::clone::Clone::clone(&self.keywords),
                    sources: ::core::clone::Clone::clone(&self.sources),
                    embedded_schema_paths: ::core::clone::Clone::clone(&self.embedded_schema_paths),
                }
            }
        }
        #[automatically_derived]
        impl<W, E> ::core::marker::StructuralPartialEq for Dialect<W, E> {}
        #[automatically_derived]
        impl<W: ::core::cmp::PartialEq, E: ::core::cmp::PartialEq> ::core::cmp::PartialEq
            for Dialect<W, E>
        {
            #[inline]
            fn eq(&self, other: &Dialect<W, E>) -> bool {
                self.uri == other.uri
                    && self.keywords == other.keywords
                    && self.sources == other.sources
                    && self.embedded_schema_paths == other.embedded_schema_paths
            }
        }
        #[automatically_derived]
        impl<W: ::core::cmp::Eq, E: ::core::cmp::Eq> ::core::cmp::Eq for Dialect<W, E> {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<AbsoluteUri>;
                let _: ::core::cmp::AssertParamIsEq<Vec<W>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<(AbsoluteUri, Arc<Value>)>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<E>>;
            }
        }
        /// A result from `anchor` of [`Keyword`]
        pub struct FoundAnchor<'v> {
            /// path of the anchor
            pub path: Pointer,
            /// anchor value
            pub anchor: &'v str,
            /// keyword of the anchor
            pub keyword: &'static str,
        }
    }
    use grill_core::{lang, Key};
    /// A JSON Schema.
    pub struct Schema<'i, K> {
        key: K,
        _marker: std::marker::PhantomData<&'i K>,
    }
    impl<'i, K: Key> lang::schema::Schema<'i, K> for Schema<'i, K> {
        fn key(&self) -> K {
            self.key
        }
    }
    pub struct CompiledSchema<W, K> {
        key: K,
        keywords: Box<[W]>,
    }
    #[automatically_derived]
    impl<W: ::core::fmt::Debug, K: ::core::fmt::Debug> ::core::fmt::Debug for CompiledSchema<W, K> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CompiledSchema",
                "key",
                &self.key,
                "keywords",
                &&self.keywords,
            )
        }
    }
    #[automatically_derived]
    impl<W: ::core::clone::Clone, K: ::core::clone::Clone> ::core::clone::Clone
        for CompiledSchema<W, K>
    {
        #[inline]
        fn clone(&self) -> CompiledSchema<W, K> {
            CompiledSchema {
                key: ::core::clone::Clone::clone(&self.key),
                keywords: ::core::clone::Clone::clone(&self.keywords),
            }
        }
    }
    #[automatically_derived]
    impl<W, K> ::core::marker::StructuralPartialEq for CompiledSchema<W, K> {}
    #[automatically_derived]
    impl<W: ::core::cmp::PartialEq, K: ::core::cmp::PartialEq> ::core::cmp::PartialEq
        for CompiledSchema<W, K>
    {
        #[inline]
        fn eq(&self, other: &CompiledSchema<W, K>) -> bool {
            self.key == other.key && self.keywords == other.keywords
        }
    }
    #[automatically_derived]
    impl<W: ::core::cmp::Eq, K: ::core::cmp::Eq> ::core::cmp::Eq for CompiledSchema<W, K> {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<K>;
            let _: ::core::cmp::AssertParamIsEq<Box<[W]>>;
        }
    }
    impl<W, K: 'static + Key> lang::schema::CompiledSchema<K> for CompiledSchema<W, K> {
        type Schema<'i> = Schema<'i, K>;
        fn set_key(&mut self, key: K) {
            self.key = key;
        }
        fn as_schema<'i>(&self, sources: &lang::Sources) -> Self::Schema<'i> {
            ::core::panicking::panic("not yet implemented")
        }
    }
}
use std::fmt::Display;
use grill_core::{lang::Init, Key, Language, Resolve};
use report::{Annotation, Error, IntoOwned};
use schema::CompiledSchema;
use serde::{de::DeserializeOwned, Serialize};
pub use {
    compile::CompileError,
    report::{Output, Report},
};
/// A trait implemented by types which are capable of evaluating a specification
/// of JSON Schema.
pub trait Specification<K: Key> {
    /// The error type that can be returned when initializing the dialect.
    type InitError;
    /// The error type that can be returned when compiling a schema.
    type CompileError: for<'v> From<CompileError<<Self::Error<'v> as IntoOwned>::Owned>>;
    type EvaluateError: for<'v> From<EvaluateError<K>>;
    /// The context type supplied to `evaluate`.
    type Evaluate: keyword::Evaluate<K>;
    type Compile: keyword::Compile<K>;
    type Keyword: keyword::Keyword<Self, K>;
    /// The annotation type to be used in [`Report`s](report::Report).
    ///
    /// Even if an annotation is not used for a keyword, it is helpful to have
    /// unit struct as an annotation for analysis pre-serialization.
    ///
    ///[`ShouldSerialize`] is used by the `Report` to determine which annotations
    /// should be serialized.
    type Annotation<'v>: From<Annotation<'v>> + Serialize + ShouldSerialize + DeserializeOwned;
    /// The error type to be used in [`Report`s](report::Report).
    type Error<'v>: From<Error<'v>> + IntoOwned + Display;
    /// Initializes the specification.
    fn init(
        &mut self,
        init: Init<'_, CompiledSchema<Self::Keyword, K>, K>,
    ) -> Result<(), Self::InitError> {
        Ok(())
    }
    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, CompiledSchema<Self::Keyword, K>, R, K>,
    ) -> Result<Self::Compile, Self::CompileError>;
    fn evaluate<'i, 'v>(
        &'i self,
        eval: grill_core::Evaluate<'i, 'v, Self::CompiledSchema, Self::Evaluate, K>,
    ) -> Result<Report<Self::Annotation<'v>, Self::Error<'v>>, Self::EvaluateError>;
}
pub(crate) mod alias {
    use super::{Report, Specification};
    pub(super) type InitError<S, K> = <S as Specification<K>>::InitError;
    pub(super) type CompileError<S, K> = <S as Specification<K>>::CompileError;
    pub(super) type EvaluateError<S, K> = <S as Specification<K>>::EvaluateError;
    pub(super) type Evaluate<S, K> = <S as Specification<K>>::Evaluate;
    pub(super) type Compile<S, K> = <S as Specification<K>>::Compile;
    pub(super) type Annotation<'v, S, K> = <S as Specification<K>>::Annotation<'v>;
    pub(super) type Error<'v, S, K> = <S as Specification<K>>::Error<'v>;
    pub(super) type TypedReport<'v, S, K> = Report<Annotation<'v, S, K>, Error<'v, S, K>>;
    pub(super) type EvaluateResult<'v, S, K> = Result<TypedReport<'v, S, K>, EvaluateError<S, K>>;
}
pub trait ShouldSerialize {
    fn should_serialize(&self) -> bool;
}
impl<S, K> Language<K> for S
where
    S: Specification<K>,
    K: Key + Send,
{
    /// The [`CompiledSchema`](schema::CompiledSchema) of this language.
    type CompiledSchema = CompiledSchema<K, S::Keyword>;
    /// The error type possibly returned from [`compile`](Language::compile).
    type CompileError = alias::CompileError<S, K>;
    /// The result type returned from [`evaluate`](Language::evaluate).
    type EvaluateResult<'v> = alias::EvaluateResult<'v, S, K>;
    /// Context type supplied to `evaluate`.
    type Context = alias::Evaluate<S, K>;
    /// The error type that can be returned when initializing the language.
    type InitError = alias::InitError<S, K>;
    /// Initializes the language with the given [`Init`] request.
    fn init(&mut self, init: Init<'_, Self::CompiledSchema, K>) -> Result<(), Self::InitError> {
        Specification::init(self, init)
    }
    /// Compiles a schema for the given [`Compile`] request and returns the key,
    /// if successful.
    ///
    /// This method is `async` to allow for languages that need to fetch schemas
    /// during compilation.
    ///
    /// # Errors
    /// Returns [`Self::CompileError`] if the schema could not be compiled.
    async fn compile<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile: grill_core::lang::Compile<'i, Self::CompiledSchema, R, K>,
    ) -> Result<K, Self::CompileError> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Compiles all schemas for the given [`CompileAll`] request and returns the
    /// keys, if successful.
    async fn compile_all<'i, R: Resolve + Send + Sync>(
        &'i mut self,
        compile_all: grill_core::lang::CompileAll<'i, Self::CompiledSchema, R, K>,
    ) -> Result<Vec<K>, Self::CompileError> {
        ::core::panicking::panic("not yet implemented")
    }
    /// Evaluates a schema for the given [`Evaluate`] request.
    fn evaluate<'i, 'v>(
        &'i self,
        eval: Evaluate<'i, 'v, Self::CompiledSchema, Self::Context, K>,
    ) -> Self::EvaluateResult<'v> {
        ::core::panicking::panic("not yet implemented")
    }
}
pub struct EvaluateError<K> {
    pub key: K,
}
pub struct InitError {}
pub struct JsonSchema {}
impl<K: Key + Send> Specification<K> for JsonSchema {
    type InitError = InitError;
    type CompileError = CompileError<Error<'static>>;
    type EvaluateError = EvaluateError<K>;
    type Evaluate = keyword::eval::Context;
    type Compile = keyword::compile::Context<K>;
    type Annotation<'v> = Annotation<'v>;
    type Error<'v> = Error<'v>;
}
