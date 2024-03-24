// //! # Json Schema Keywords

// use grill_core::{
//     error::{AnchorError, AnchorInvalidLeadChar},
//     keyword::Context,
// };

// pub mod additional_properties;
// pub mod all_of;
// pub mod anchor;
// pub mod any_of;
// pub mod comment;
// pub mod const_;
// pub mod defs;
// pub mod dynamic_anchor;
// pub mod dynamic_ref;
// pub mod enum_;
// pub mod id;
// pub mod if_then_else;
// pub mod not;
// pub mod one_of;
// pub mod pattern;
// pub mod pattern_properties;
// pub mod properties;
// pub mod read_only;
// pub mod ref_;
// pub mod schema;
// pub mod type_;
// pub mod write_only;

use grill_core::{criterion::Criterion, Key};

#[derive(Debug, Clone)]
pub enum Keyword {
    // Schema(schema::Schema),
    // Id(id::Id),
    // Ref(ref_::Ref),
    // Defs(defs::Defs),
    // Comment(comment::Comment),
    // DynamicRef(dynamic_ref::DynamicRef),
    // DynamicAnchor(dynamic_anchor::DynamicAnchor),
    // Anchor(anchor::Anchor),
    // AdditionalProperties(additional_properties::AdditionalProperties),
    // AllOf(all_of::AllOf),
    // AnyOf(any_of::AnyOf),
    // OneOf(one_of::OneOf),
    // Then(if_then_else::Then),
    // If(if_then_else::If),
    // Else(if_then_else::Else),
    // Not(not::Not),
    // Properties(properties::Properties),
    // Type(type_::Type),
    // Enum(enum_::Enum),
    // Const(const_::Const),
    // Pattern(pattern::Pattern),
    // PatternProperties(pattern_properties::PatternProperties),
    // WriteOnly(write_only::WriteOnly),
    // ReadOnly(read_only::ReadOnly),
    // AnyOf
    // OneOf
    // Then
    // If
    // Else
    // Not
    // Properties
    // Type
    // Enum
    // Const
    // Pattern
    // PatternProperties
    // WriteOnly
    // ReadOnly
    // AdditionalProperties
    // DependentSchemas
    // PropertyNames
    // Items
    // PrefixItems
    // Contains
    // MinLength
    // MaxLength
    // ExclusiveMaximum
    // MultipleOf
    // ExclusiveMinimum
    // Maximum
    // Minimum
    // DependentRequired
    // MaxProperties
    // MinProperties
    // Required
    // MaxItems
    // MinItems
    // MaxContains
    // MinContains
    // UniqueItems
    // Title
    // Description
    // Default
    // Examples
    // Deprecated
    // UnevaluatedProperties
    // UnevaluatedItems
    // ContentSchema
    // ContentMediaType
    // ContentEncoding
    // Format
}

macro_rules! delegate {
    () => {};
}

impl<C, K> grill_core::criterion::Keyword<C, K> for Keyword
where
    C: Criterion<K>,
    K: Key,
{
    fn kind(&self) -> grill_core::criterion::Kind {}

    fn compile<'i>(
        &mut self,
        compile: &mut C::Compile,
        schema: grill_core::Schema<'i, C, K>,
    ) -> Result<std::ops::ControlFlow<()>, grill_core::error::CompileError<C, K>> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut C::Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<C::Report>, grill_core::error::EvaluateError<K>> {
        todo!()
    }
}

// impl Keyword {
//     #[must_use]
//     pub fn as_schema(&self) -> Option<&schema::Schema> {
//         if let Self::Schema(v) = self {
//             Some(v)
//         } else {
//             None
//         }
//     }
// }

// impl grill_core::keyword::Keyword for Keyword {
//     type Context = Context;
//     type Compile;
//     type Evaluation;
//     type ValidationError;
//     type CompileError;
//     type EvaluateError;
//     fn kind(&self) -> grill_core::keyword::Kind {
//         todo!()
//     }

//     fn compile<'i>(
//         &mut self,
//         compile: &mut grill_core::keyword::Compile<'i>,
//         schema: grill_core::Schema<'i>,
//     ) -> Result<bool, grill_core::error::CompileError> {
//         todo!()
//     }

//     fn evaluate<'i, 'v>(
//         &'i self,
//         ctx: &'i mut Self::Context,
//         value: &'v serde_json::Value,
//     ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
//         todo!()
//     }
// }

// /// Validates the value of `anchor` by ensuring that it start with a letter
// /// (`[A-Za-z]`) or underscore (`'_'`), followed by any number of letters, digits
// /// (`[0-9]`), hyphens (`"-"`), underscores (`'_'`), and periods (`'.'`)
// pub fn validate_anchor(keyword: &'static str, anchor: &str) -> Result<(), AnchorError> {
//     if anchor.is_empty() {
//         return Err(AnchorError::Empty(keyword));
//     }
//     let mut chars = anchor.chars();
//     let first = chars.next().unwrap();
//     if !first.is_ascii_alphabetic() && first != '_' {
//         return Err(AnchorInvalidLeadChar {
//             char: first,
//             value: anchor.to_string(),
//             keyword,
//         }
//         .into());
//     }
//     for c in chars {
//         if !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '.' {
//             return Err(AnchorInvalidLeadChar {
//                 char: c,
//                 value: anchor.to_string(),
//                 keyword,
//             }
//             .into());
//         }
//     }
//     Ok(())
// }

// /// Context for compilation of the [`Keyword`]
// #[derive(Debug)]
// pub struct Compile<'i> {
//     pub(crate) absolute_uri: &'i AbsoluteUri,
//     pub(crate) schemas: &'i Schemas,
//     pub(crate) numbers: &'i mut Numbers,
//     pub(crate) value_cache: &'i mut Values,
// }

// impl<'i> Compile<'i> {
//     #[must_use]
//     /// The [`AbsoluteUri`] of the [`Schema`]
//     pub fn absolute_uri(&self) -> &AbsoluteUri {
//         self.absolute_uri
//     }

//     /// Parses a [`Number`] into a [`BigRational`], stores it and returns an
//     /// `Arc` to it.
//     ///
//     /// # Errors
//     /// Returns `NumberError` if the number fails to parse
//     pub fn number(&mut self, num: &Number) -> Result<Arc<BigRational>, NumberError> {
//         self.numbers.get_or_insert_arc(num)
//     }
//     /// Caches a [`Value`] and returns an `Arc` to it.
//     pub fn value(&mut self, value: &Value) -> Arc<Value> {
//         self.value_cache.value(value)
//     }

//     /// Resolves a schema `Key` by URI
//     ///
//     /// # Errors
//     /// - `CompileError::SchemaNotFound` if the schema is not found
//     /// - `CompileError::UriParsingFailed` if the URI is invalid
//     pub fn schema(&self, uri: &str) -> Result<Key, CompileError> {
//         let uri: Uri = uri.parse()?;
//         let uri = self.absolute_uri.with_fragment(None)?.resolve(&uri)?;
//         self.schemas
//             .get_key(&uri)
//             .ok_or(CompileError::SchemaNotFound(uri))
//     }

//     /// Returns the [`Key`] of a schema at the specified `path` relative to
//     /// the current schema.
//     ///
//     /// # Errors
//     /// Returns a [`CompileError`] if the schema cannot be found.
//     pub fn subschema(&self, path: &Pointer) -> Result<Key, CompileError> {
//         let mut uri = self.absolute_uri().clone();

//         if let Some(fragment) = uri.fragment_decoded_lossy() {
//             let mut ptr = fragment.parse::<Pointer>()?;
//             ptr.append(path);
//             uri.set_fragment(Some(&ptr))?;
//         } else {
//             uri.set_fragment(Some(path))?;
//         }
//         self.schemas
//             .get_key(&uri)
//             .ok_or(CompileError::SchemaNotFound(uri))
//     }
// }

// /*
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// ╔═══════════════════════════════════════════════════════════════════════╗
// ║                                                                       ║
// ║                                Context                                ║
// ║                                ¯¯¯¯¯¯¯                                ║
// ╚═══════════════════════════════════════════════════════════════════════╝
// ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
// */
// /// Contains global state, evaluation level state, schemas, and location
// /// information needed to [`evaluate`](`crate::Interrogator::evaluate`) a
// /// schema.
// pub struct Context<'i> {
//     pub(crate) absolute_keyword_location: &'i AbsoluteUri,
//     pub(crate) keyword_location: Pointer,
//     pub(crate) instance_location: Pointer,
//     pub(crate) structure: Structure,
//     pub(crate) schemas: &'i Schemas,
//     pub(crate) sources: &'i Sources,
//     pub(crate) global_numbers: &'i Numbers,
//     pub(crate) eval_numbers: &'i mut Numbers,
// }

// impl<'s> Context<'s> {
//     /// Evaluates `value` against the schema with the given `key` for the
//     /// `keyword` and produces an [`Output`]
//     pub fn evaluate<'v>(
//         &mut self,
//         key: Key,
//         instance: Option<&str>,
//         keyword: &Pointer,
//         value: &'v Value,
//     ) -> Result<Output<'v>, EvaluateError> {
//         if self.absolute_keyword_location().host().as_deref() != Some("json-schema.org") {
//             // println!("{}", self.absolute_keyword_location());
//             // println!("{}", serde_json::to_string_pretty(&value).unwrap());
//         }
//         let mut instance_location = self.instance_location.clone();
//         if let Some(instance) = instance {
//             instance_location.push_back(instance.into());
//         }
//         self.evaluated.insert(instance_location.to_string());
//         let mut keyword_location = self.keyword_location.clone();
//         keyword_location.append(keyword);
//         self.schemas.evaluate(
//             self.structure,
//             key,
//             value,
//             instance_location,
//             keyword_location,
//             self.sources,
//             self.evaluated,
//             self.global_state,
//             self.eval_state,
//             self.global_numbers,
//             self.eval_numbers,
//         )
//     }
//     /// Either returns a reference to a previously parsed [`BigRational`] or
//     /// parses, stores the [`BigRational`] as an [`Arc`] (per eval) and returns
//     /// a reference to the [`BigRational`].
//     ///
//     /// # Errors
//     /// Returns [`NumberError`] if the number fails to parse
//     pub fn number_ref(&mut self, number: &Number) -> Result<&BigRational, NumberError> {
//         if let Some(n) = self.global_numbers.get_ref(number) {
//             return Ok(n);
//         }
//         self.eval_numbers.get_or_insert_ref(number)
//     }
//     /// Either returns a [`Arc`] to a previously parsed [`BigRational`] or
//     /// parses, stores (per eval) and returns an [`Arc`] to the [`BigRational`].
//     ///
//     /// # Errors
//     /// Returns [`NumberError`] if the number fails to parse
//     pub fn number_arc(&mut self, number: &Number) -> Result<Arc<BigRational>, NumberError> {
//         if let Some(n) = self.global_numbers.get_arc(number) {
//             return Ok(n);
//         }
//         self.eval_numbers.get_or_insert_arc(number)
//     }
//     #[must_use]
//     pub fn absolute_keyword_location(&self) -> &AbsoluteUri {
//         self.absolute_keyword_location
//     }

//     /// Evaluates `value` against the schema with the given `key` but does not
//     /// mark the instance as evaluated.
//     ///
//     /// This is intended for use with `if` but may be used
//     /// in other cases.
//     pub fn probe<'v>(
//         &mut self,
//         key: Key,
//         instance: Option<&str>,
//         keyword: &Pointer,
//         value: &'v Value,
//     ) -> Result<Output<'v>, EvaluateError> {
//         let mut instance_location = self.instance_location.clone();
//         if let Some(instance) = instance {
//             instance_location.push_back(instance.into());
//         }
//         let mut keyword_location = self.keyword_location.clone();
//         keyword_location.append(keyword);
//         self.schemas.evaluate(
//             self.structure,
//             key,
//             value,
//             instance_location,
//             keyword_location,
//             self.sources,
//             self.evaluated,
//             self.global_state,
//             self.eval_state,
//             self.global_numbers,
//             self.eval_numbers,
//         )
//     }

//     /// Mutable reference to the eval local state [`AnyMap`].
//     ///
//     /// This does not include the [`global_state`](`Context::global_state`).
//     #[must_use]
//     pub fn global_state(&self) -> &AnyMap {
//         self.global_state
//     }

//     /// Mutable reference to the eval local state [`AnyMap`].
//     ///
//     /// This does not include the [`global_state`](`Context::global_state`).
//     pub fn eval_state(&mut self) -> &mut AnyMap {
//         self.eval_state
//     }

//     /// creates a valid [`Output`] with the given `keyword` and `annotation`
//     #[must_use]
//     pub fn annotate<'v>(
//         &mut self,
//         keyword: Option<&'static str>,
//         annotation: Option<Annotation<'v>>,
//     ) -> Output<'v> {
//         self.create_output(keyword, Ok(annotation), false)
//     }

//     /// Creates an invalid [`Output`] with the given `keyword` and `error`
//     pub fn error<'v>(
//         &mut self,
//         keyword: Option<&'static str>,
//         error: Option<BoxedError<'v>>,
//     ) -> Output<'v> {
//         self.create_output(keyword, Err(error), false)
//     }

//     /// Creates a transient [`Output`] with the given `keyword` and `nodes`
//     ///
//     /// A transient `Output` is one which is not included in the final output
//     /// but accumulates errors and annotations, which are then flattened into a
//     /// series of `Output`s which are included in the final output without
//     /// having their conjunction considered.
//     ///
//     /// Essentially, a transient `Output` is a pseudo node which has its state
//     /// determined by the `Keyword` rather than the result of it's children.
//     ///
//     /// The transient `Output` is removed from the final output, promoting the
//     /// `nodes` to the same level as the transient `Output`.
//     pub fn transient<'v>(
//         &mut self,
//         is_valid: bool,
//         nodes: impl IntoIterator<Item = Output<'v>>,
//     ) -> Output<'v> {
//         let op = if is_valid { Ok(None) } else { Err(None) };
//         let mut output = self.create_output(None, op, true);
//         output.append(nodes.into_iter());
//         output.set_valid(is_valid);
//         output
//     }

//     fn create_output<'v>(
//         &mut self,
//         keyword: Option<&'static str>,
//         annotation_or_error: AnnotationOrError<'v>,
//         is_transient: bool,
//     ) -> Output<'v> {
//         let mut keyword_location = self.keyword_location.clone();
//         let mut absolute_keyword_location = self.absolute_keyword_location.clone();

//         if let Some(keyword) = keyword {
//             let tok: Token = keyword.into();
//             keyword_location.push_back(tok.clone());
//             if let Ok(mut ptr) = absolute_keyword_location
//                 .fragment_decoded_lossy()
//                 .unwrap_or_default()
//                 .parse::<Pointer>()
//             {
//                 ptr.push_back(tok);
//                 absolute_keyword_location.set_fragment(Some(&ptr)).unwrap();
//             }
//         }
//         Output::new(
//             self.structure,
//             absolute_keyword_location,
//             keyword_location,
//             self.instance_location.clone(),
//             annotation_or_error,
//             is_transient,
//         )
//     }

//     /// Returns `true` if the instance location has been evaluated.
//     #[must_use]
//     pub fn has_evaluated(&self, instance: &str) -> bool {
//         let mut instance_location = self.instance_location.clone();
//         instance_location.push_back(instance.into());
//         self.evaluated.contains(&self.instance_location.to_string())
//     }

//     /// Returns `true` if enabling short-circuiting was successful or if it
//     /// was previously set to `true`.
//     pub fn enable_short_circuiting(&mut self) -> bool {
//         if let Some(should_short_circuit) = self.should_short_circuit {
//             should_short_circuit
//         } else {
//             self.should_short_circuit = Some(true);
//             true
//         }
//     }
//     /// Disables short-circuiting
//     pub fn disable_short_circuiting(&mut self) {
//         self.should_short_circuit = Some(false);
//     }

//     /// Returns `true` if the evaluation should short-circuit, as determined
//     /// by the [`ShortCircuit`](grill_json_schema::keyword::short_circuit::ShortCircuit) keyword handler
//     #[must_use]
//     pub fn should_short_circuit(&self) -> bool {
//         self.should_short_circuit.unwrap_or(false)
//     }

//     /// Returns the desired [`Structure`] of the evaluation
//     #[must_use]
//     pub fn structure(&self) -> Structure {
//         self.structure
//     }
// }

/// ## `$id`
///
/// The value of `$id` is a URI-reference without a fragment that resolves
/// against the Retrieval URI. The resulting URI is the base URI for the
/// schema.
///
/// - [JSON Schema Core 2020-12 # 8.2.1. The `$id` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-id-keyword)
/// - [Understanding JSON Schema # Structuring a complex schema: `$id`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=id#id)
pub const ID: &str = "$id";

/// ## `id`
///
/// The value of `id` is a URI-reference without a fragment that resolves
/// against the Retrieval URI. The resulting URI is the base URI for the
/// schema.
///
/// # Deprecated
/// Renamed to `$id` in JSON Schema 07
pub const ID_LEGACY: &str = "id";

/// ## `$schema`
///
/// The `$schema` keyword is both used as a JSON Schema dialect identifier
/// and as the identifier of a resource which is itself a JSON Schema, which
/// describes the set of valid schemas written for this particular dialect.
///
/// - [JSON Schema Core 2020-12 # 8.1.1. The `$schema` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.1)
/// - [Draft 2019-09 Core # 8.1.1. The `$schema` Keyword](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.1.1)
/// - [Draft 7 # 7. The `$schema` Keyword](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-01#section-7)
pub const SCHEMA: &str = "$schema";

/// ## `$comment`
///
/// The `$comment` keyword is strictly intended for adding comments to a
/// schema. Its value must always be a string. Unlike the annotations title,
/// description, and examples, JSON schema implementations aren’t allowed to
/// attach any meaning or behavior to it whatsoever, and may even strip them
/// at any time. Therefore, they are useful for leaving notes to future
/// editors of a JSON schema, but should not be used to communicate to users
/// of the schema.
///
/// - [Understanding JSON Schema # Generic keywords:
///   Comments](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#comments)
pub const COMMENT: &str = "$comment";

/// ## `$vocabulary`
///
/// The `$vocabulary` keyword is used in meta-schemas to identify the
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
/// The `$vocabulary` keyword SHOULD be used in the root schema of any
/// schema document intended for use as a meta-schema. It MUST NOT appear in
/// subschemas.
///
/// The `$vocabulary` keyword MUST be ignored in schema documents that are
/// not being processed as a meta-schema. This allows validating a
/// meta-schema M against its own meta-schema M' without requiring the
/// validator to understand the vocabularies declared by M.
///
/// - [JSON Schema Core 2020-12 # 8.1.2. The `$vocabulary` Keyword](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8.1.2)
pub const VOCABULARY: &str = "$vocabulary";
/// ## `title`
///
/// A title can be used to decorate a user interface with information about
/// the data produced by this user interface.
///
/// - [JSON Schema Validation 2020-12 # 9.1 `title` and "description"](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#section-9.1)
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
/// The value of this keyword MUST be a boolean. When multiple occurrences
/// of this keyword are applicable to a single sub-instance, applications
/// SHOULD consider the instance location to be deprecated if any occurrence
/// specifies a true value.
///
/// If `deprecated` has a value of boolean true, it indicates that
/// applications SHOULD refrain from usage of the declared property. It MAY
/// mean the property is going to be removed in the future.
///
/// A root schema containing `deprecated` with a value of true indicates
/// that the entire resource being described MAY be removed in the future.
///
/// The `deprecated` keyword applies to each instance location to which the
/// schema object containing the keyword successfully applies. This can
/// result in scenarios where every array item or object property is
/// deprecated even though the containing array or object is not.
///
/// Omitting this keyword has the same behavior as a value of false.
///
/// - [JSON Schema Validation 2020-12 # 9.3 `deprecated`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-deprecated)
pub const DEPRECATED: &str = "deprecated";

/// ## `readOnly`
///
///  When multiple occurrences of these keywords are applicable to a single
///  sub-instance, the resulting behavior SHOULD be as for a `true` value if
///  any occurrence specifies a `true` value, and SHOULD be as for a `false`
///  value otherwise.
///
/// If `readOnly` has a value of boolean `true`, it indicates that the value
/// of the instance is managed exclusively by the owning authority, and
/// attempts by an application to modify the value of this property are
/// expected to be ignored or rejected by that owning authority.
///
/// An instance document that is marked as `readOnly` for the entire
/// document MAY be ignored if sent to the owning authority, or MAY result
/// in an error, at the authority's discretion.
///
/// This keyword can be used to assist in user interface instance
/// generation.  The `readOnly` keyword does not imply how a server handles
/// writes to a value in the case of a conflict - e.g., whether it rejects
/// them or whether it attempts to resolve the conflict in some manner.
///
/// - [JSON Schema Validation 2020-12 # 9.4 `readOnly` and `writeOnly`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-readOnly-and-writeonly)
pub const READ_ONLY: &str = "readOnly";

/// ## `writeOnly`
///
/// When multiple occurrences of these keywords are applicable to a single
/// sub-instance, the resulting behavior SHOULD be as for a `true` value if
/// any occurrence specifies a `true` value, and SHOULD be as for a `false`
/// value otherwise.
///
/// If `writeOnly` has a value of boolean true, it indicates that the value
/// is never present when the instance is retrieved from the owning
/// authority. It can be present when sent to the owning authority to update
/// or create the document (or the resource it represents), but it will not
/// be included in any updated or newly created version of the instance.
///
/// An instance document that is marked as `writeOnly` for the entire
/// document MAY be returned as a blank document of some sort, or MAY
/// produce an error upon retrieval, or have the retrieval request ignored,
/// at the authority's discretion.
/// - [JSON Schema Validation 2020-12 # 9.4 `readOnly` and `writeOnly`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-readOnly-and-writeonly)
pub const WRITE_ONLY: &str = "writeOnly";

/// ## `$anchor`
///
/// Using JSON Pointer fragments requires knowledge of the structure of the
/// schema. When writing schema documents with the intention to provide
/// re-usable schemas, it may be preferable to use a plain name fragment
/// that is not tied to any particular structural location. This allows a
/// subschema to be relocated without requiring JSON Pointer references to
/// be updated.
///
/// The `$anchor` and `$dynamicAnchor` keywords are used to specify such
/// fragments. They are identifier keywords that can only be used to create
/// plain name fragments, rather than absolute URIs as seen with `$id`.
///
/// The base URI to which the resulting fragment is appended is the
/// canonical URI of the schema resource containing the `$anchor` or
/// `$dynamicAnchor` in question. As discussed in the previous section,
/// this is either the nearest `$id` in the same or parent schema object,
/// or the base URI for the document as determined according to RFC 3986.
///
/// Separately from the usual usage of URIs, `$dynamicAnchor` indicates
/// that the fragment is an extension point when used with the
/// `$dynamicRef` keyword. This low-level, advanced feature makes it
/// easier to extend recursive schemas such as the meta-schemas, without
/// imposing any particular semantics on that extension. See the section on
/// `$dynamicRef` [(Section
/// 8.2.3.2)](https://json-schema.org/draft/2020-12/json-schema-core.html#dynamic-ref)
/// for details.
///
/// In most cases, the normal fragment behavior both suffices and is more
/// intuitive. Therefore it is RECOMMENDED that `$anchor` be used to
/// create plain name fragments unless there is a clear need for
/// `$dynamicAnchor`.
///
/// If present, the value of this keyword MUST be a string and MUST start
/// with a letter (`[A-Za-z]`) or underscore `'_'`, followed by any number
/// of letters, digits (`[0-9]`), hyphens `'-'`, underscores `'_'`, and
/// periods `'.'`. This matches the US-ASCII part of XML's `NCName` production
/// [xml-names]. Note that the anchor string does not include the `'#'`
/// character, as it is not a URI-reference. An `"$anchor": "foo"` becomes
/// the fragment `"#foo"` when used in a URI. See below for full examples.

///
/// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
/// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with `$dynamicRef`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
/// - [Draft 2019-09 Core # 8.2.3. Defining location-independent identifiers with `$anchor`](https://json-schema.org/draft/2019-09/json-schema-core.html#rfc.section.8.2.3)
/// - [Understanding JSON Schema # `$anchor`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=anchor#anchor)
pub const ANCHOR: &str = "$anchor";

/// ## `$dynamicAnchor`
///
/// A `$dynamicAnchor` can be thought of like a normal $anchor except that
/// it can be referenced across schemas rather than just in the schema where
/// it was defined. You can think of the old `$recursiveAnchor` as working
/// the same way except that it only allowed you to create one anchor per
/// schema, it had to be at the root of the schema, and the anchor name is
/// always empty.
///
/// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with `$dynamicRef`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
/// - [JSON Schema Core 2020-12 # 8.2.2. Defining location-independent identifiers](https://json-schema.org/draft/2020-12/json-schema-core.html#name-defining-location-independe)
/// - [JSON Schema Core 2020-12 Release Notes # `$dynamicRef` and `$dynamicAnchor`](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
pub const DYNAMIC_ANCHOR: &str = "$dynamicAnchor";

/// ## `$dynamicRef`
///
/// The `$dynamicRef` keyword is an applicator that allows for deferring the
/// full resolution until runtime, at which point it is resolved each time
/// it is encountered while evaluating an instance.
///
/// Together with `$dynamicAnchor`, `$dynamicRef` implements a cooperative
/// extension mechanism that is primarily useful with recursive schemas
/// (schemas that reference themselves). Both the extension point and the
/// runtime-determined extension target are defined with `$dynamicAnchor`,
/// and only exhibit runtime dynamic behavior when referenced with
/// `$dynamicRef`.
///
/// The value of the `$dynamicRef` property MUST be a string which is a
/// URI-Reference. Resolved against the current URI base, it produces the
/// URI used as the starting point for runtime resolution. This initial
/// resolution is safe to perform on schema load.
///
/// - [JSON Schema Core 2020-12 # 8.2.3.2. Dynamic References with `$dynamicRef`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-dynamic-references-with-dyn)
/// - [JSON Schema Core 2020-12 Release Notes # `$dynamicRef` and `$dynamicAnchor`](https://json-schema.org/draft/2020-12/release-notes.html#dynamicref-and-dynamicanchor)
pub const DYNAMIC_REF: &str = "$dynamicRef";

/// ## `$ref`
///
/// The `$ref` keyword is an applicator that is used to reference a
/// statically identified schema. Its results are the results of the
/// referenced schema. [CREF5]
///
/// The value of the `$ref` keyword MUST be a string which is a
/// URI-Reference. Resolved against the current URI base, it produces the URI
/// of the schema to apply. This resolution is safe to perform on schema
/// load, as the process of evaluating an instance cannot change how the
/// reference resolves.
///
/// - [JSON Schema Core 2020-12 # 8.2.3.1. Direct References with `$ref`](https://json-schema.org/draft/2020-12/json-schema-core.html#ref)
/// - [Understanding JSON Schema # Structuring a complex schema `$ref`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=ref#ref)
pub const REF: &str = "$ref";

/// ## `$recursiveAnchor`
///
/// `$recursiveAnchor` is used to dynamically
/// identify a base URI at runtime for `"$recursiveRef"` by marking where
/// such a calculation can start, and where it stops.  This keyword MUST NOT
/// affect the base URI of other keywords, unless they are explicitly
/// defined to rely on it.
///
/// - [Draft 2019-09 Core # 8.2.4.2.2.  Enabling Recursion with `$recursiveAnchor`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-8.2.4.2.2)
pub const RECURSIVE_ANCHOR: &str = "$recursiveAnchor";

/// ## `type`
///
/// The `type` keyword is fundamental to JSON Schema. It specifies the
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
pub const TYPE: &str = "type";
/// ## `format`
/// The format keyword allows for basic semantic identification of certain
/// kinds of string values that are commonly used. For example, because JSON
/// doesn’t have a `DateTime` type, dates need to be encoded as strings.
/// format allows the schema author to indicate that the string value should
/// be interpreted as a date. By default, format is just an annotation and
/// does not effect validation.
///
/// - [JSON Schema Core 2020-12 # 7. Vocabularies for Semantic Content With `format`](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c)
/// - [Understanding Json Schema # string Built-in Formats](https://json-schema.org/understanding-json-schema/reference/string.html#id7)
/// - [OpenAPI 3.1 Specification # 4.2 Format](https://spec.openapis.org/oas/v3.1.0#format)
pub const FORMAT: &str = "format";

/// ## `const`
///
/// The `const` keyword is used to restrict a value to a single value.
///
/// - [JSON Schema Validation 2020-12 # 6.1.3. `const`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-const)
/// - [Understanding JSON Schema - Constant values](https://json-schema.org/understanding-json-schema/reference/generic.html?highlight=const#constant-values)
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
/// - [JSON Schema Core 2020-12 # 8.2.4. Schema Re-Use With `$defs`](https://json-schema.org/draft/2020-12/json-schema-core.html#defs)
/// - [Understanding JSON Schema # `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
pub const DEFS: &str = "$defs";

/// ## `definitions`
///
/// Legacy definitions from Draft 07 and earlier.
///
/// ## Note
/// If using JSON Schema 07 or earlier, use this field instead of [`$defs`](`Keyword::DEFS`).
///
/// - [Understanding JSON Schema # `$defs`](https://json-schema.org/understanding-json-schema/structuring.html?highlight=$defs#defs)
pub const DEFINITIONS_LEGACY: &str = "definitions";

/// ## `enum`
///
/// An instance validates successfully against this keyword if its value is
/// equal to one of the elements in this keyword's array value.
///
/// Elements in the array might be of any type, including null.
///
/// - [JSON Schema Validation 2020-12 # 6.1.2. `enum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-enum)
pub const ENUM: &str = "enum";

/// ## `allOf`
///
/// This keyword's value MUST be a non-empty array. Each item of the array
/// MUST be a valid JSON Schema.
///
/// The `allOf` keyword acts as an `AND` where each subschema must be
/// valid
///
/// An instance validates successfully against this keyword if it validates
/// successfully against all schemas defined by this keyword's value.
///
/// - [JSON Schema 2020-12 # 10.2.1.1. `allOf`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-allof)
/// - [Understanding JSON Schema # Schema Composition `allOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=allOf#allOf)
pub const ALL_OF: &str = "allOf";

/// ## `anyOf`
///
/// This keyword's value MUST be a non-empty array. Each item of the array
/// MUST be a valid JSON Schema.
///
/// The `anyOf` keyword acts as an `OR` where at least one of the
/// subschemas must be valid
///
/// An instance validates successfully against this keyword if it validates
/// successfully against at least one schema defined by this keyword's
/// value. Note that when annotations are being collected, all subschemas
/// MUST be examined so that annotations are collected from each subschema
/// that validates successfully.
///
/// - [JSON Schema Core 2020-12 # 10.2.1.2. `anyOf`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.1.2)
/// - [Understanding JSON Schema # Schema Composition `anyOf`](https://json-schema.org/understanding-json-schema/reference/combining.html?highlight=anyof#anyOf)
pub const ANY_OF: &str = "anyOf";

/// ## `oneOf`
///
/// This keyword's value MUST be a non-empty array. Each item of the array
/// MUST be a valid JSON Schema.
///
/// The `oneOf` keyword acts as an `XOR` where exactly one of the
/// subschemas must be valid
///
/// - [JSON Schema Core 2020-12 # 10.2.1.3. `oneOf`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-oneof)
/// - [Understanding JSON Schema # Schema Composition `oneOf`](https://json-schema.org/understanding-json-schema/reference/combining.html#oneof)
pub const ONE_OF: &str = "oneOf";

/// ## `not`
///
/// This keyword's value MUST be a valid JSON Schema.
///
/// The not keyword declares that an instance validates if it doesn’t
/// validate against the given subschema.
///
/// An instance is valid against this keyword if it fails to validate
/// successfully against the schema defined by this keyword.
///
/// - [JSON Schema Core 2020-12 #](https://json-schema.org/draft/2020-12/json-schema-core.html#name-not)
/// - [Understanding JSON Schema # Schema Composition `not`](https://json-schema.org/understanding-json-schema/reference/combining.html?#id8)
pub const NOT: &str = "not";

/// ## `if`
///
/// This keyword's value MUST be a valid JSON Schema.
///
/// This validation outcome of this keyword's subschema has no direct effect
/// on the overall validation result. Rather, it controls which of the
/// `then` or `else` keywords are evaluated. Instances that successfully
/// validate against this keyword's subschema MUST also be valid against the
/// subschema value of the `then` keyword, if present.
///
/// Instances that fail to validate against this keyword's subschema MUST
/// also be valid against the subschema value of the `else` keyword, if
/// present.
///
/// - [JSON Schema Core 2020-12 # 10.2.2.1. `if`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.2.1)
pub const IF: &str = "if";

/// ## `then`
///
/// This keyword's value MUST be a valid JSON Schema.
///
/// When `if` is present, and the instance successfully validates against
/// its subschema, then validation succeeds against this keyword if the
/// instance also successfully validates against this keyword's subschema.
///
/// This keyword has no effect when `if` is absent, or when the instance
/// fails to validate against its subschema. Implementations MUST NOT
/// evaluate the instance against this keyword, for either validation or
/// annotation collection purposes, in such cases.
///
/// - [JSON Schema Core 2020-12 # 10.2.2.2. `then`](https://json-schema.org/draft/2020-12/json-schema-core.html#section-10.2.2.2)
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
/// - [JSON Schema Core 2020-12 # 10.2.2.3. `else`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-else)
pub const ELSE: &str = "else";

/// ## `dependentSchemas`
///
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
pub const DEPENDENT_SCHEMAS: &str = "dependentSchemas";

/// ## `prefixItems`
///
/// The value of `prefixItems` MUST be a non-empty array of valid JSON Schemas.
///
/// Validation succeeds if each element of the instance validates against
/// the schema at the same position, if any. This keyword does not constrain
/// the length of the array. If the array is longer than this keyword's
/// value, this keyword validates only the prefix of matching length.
///
/// This keyword produces an annotation value which is the largest index to
/// which this keyword applied a subschema. The value MAY be a boolean true
/// if a subschema was applied to every index of the instance, such as is
/// produced by the `items` keyword. This annotation affects the behavior of
/// `items` and `unevaluatedItems`.
///
/// Omitting this keyword has the same assertion behavior as an empty array.
///
/// - [JSON Schema Core 2020-12 # 10.3.1.1. `prefixItems`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-prefixitems)
pub const PREFIX_ITEMS: &str = "prefixItems";

/// ## `items`
///
/// The value of `items` MUST be a valid JSON Schema for 2020-12 and the
/// either a non-empty array of valid JSON Schemas or a valid JSON Schema
/// for 2019-09, 07.
///
/// This keyword applies its subschema to all instance elements at indexes
/// greater than the length of the `prefixItems` array in the same schema
/// object, as reported by the annotation result of that `prefixItems`
/// keyword. If no such annotation result exists, `items` applies its
/// subschema to all instance array elements. Note that the behavior of
/// `items` without `prefixItems` is identical to that of the schema form of
/// `items` in prior drafts. When `prefixItems` is present, the behavior of
/// `items` is identical to the former `additionalItems` keyword.
///
/// ## For Draft 2019, 07:
/// If `items` is a schema, validation succeeds if all elements in the array
/// successfully validate against that schema.
///
/// If `items` is an array of schemas, validation succeeds if each element
/// of the instance validates against the schema at the same position, if
/// any.
/// - [JSON Schema Core 2020-12 # 10.3.1.2. `items`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-items)
/// - [JSON Schema Validation 07 # 6.4.1 `items`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-validation-01#section-6.4.1)
pub const ITEMS: &str = "items";

/// ## `additionalItems`
///
/// The value of `additionalItems` MUST be a valid JSON Schema.
///
/// The behavior of this keyword depends on the presence and annotation
/// result of `items` within the same schema object.  If `items` is
/// present, and its annotation result is a number, validation succeeds
/// if every instance element at an index greater than that number
/// validates against `additionalItems`.
///
/// Otherwise, if `items` is absent or its annotation result is the
/// boolean true, `additionalItems` MUST be ignored.
///
/// If the `additionalItems` subschema is applied to any positions within
/// the instance array, it produces an annotation result of boolean true,
///
///
/// analogous to the single schema behavior of `items`.  If any
/// `additionalItems` keyword from any subschema applied to the same
/// instance location produces an annotation value of true, then the
/// combined result from these keywords is also true.
///
/// Omitting this keyword has the same assertion behavior as an empty
/// schema.
///
/// Implementations MAY choose to implement or optimize this keyword in
/// another way that produces the same effect, such as by directly
/// checking for the presence and size of an `items` array.
/// Implementations that do not support annotation collection MUST do so.
///
/// - [JSON Schema Core 2019-09 # 9.3.1.2.  `additionalItems`](https://datatracker.ietf.org/doc/html/draft-handrews-json-schema-02#section-9.3.1.2)
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
/// This keyword produces an annotation value which is an array of the
/// indexes to which this keyword validates successfully when applying its
/// subschema, in ascending order. The value MAY be a boolean "true" if the
/// subschema validates successfully when applied to every index of the
/// instance. The annotation MUST be present if the instance array to which
/// this keyword's schema applies is empty.
///
/// This annotation affects the behavior of `unevaluatedItems` in the
/// Unevaluated vocabulary, and MAY also be used to implement the
/// `minContains` and `maxContains` keywords in the Validation
/// vocabulary.
///
/// The subschema MUST be applied to every array element even after the
/// first match has been found, in order to collect annotations for use by
/// other keywords. This is to ensure that all possible annotations are
/// collected.
///
///  - [JSON Schema Core 2020-12 # 10.3.1.3. `contains`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-contains)
pub const CONTAINS: &str = "contains";

/// ## `properties`
///
/// The value of `properties` MUST be an object. Each value of this object
/// MUST be a valid JSON Schema.
///
/// Validation succeeds if, for each name that appears in both the instance
/// and as a name within this keyword's value, the child instance for that
/// name successfully validates against the corresponding schema.
///
/// The annotation result of this keyword is the set of instance property
/// names matched by this keyword. This annotation affects the behavior of
/// `additionalProperties` (in this vocabulary) and `unevaluatedProperties`
/// in the Unevaluated vocabulary.
///
/// Omitting this keyword has the same assertion behavior as an empty
/// object.
///
/// - [JSON Schema Core 2020-12 # 10.3.2.1. properties](https://json-schema.org/draft/2020-12/json-schema-core.html#name-properties)
pub const PROPERTIES: &str = "properties";

/// ## `patternProperties`
///
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
/// `additionalProperties` (in this vocabulary) and `unevaluatedProperties`
/// (in the Unevaluated vocabulary).
///
/// Omitting this keyword has the same assertion behavior as an empty
/// object.
///
/// - [JSON Schema Core 2020-12 # 10.3.2.2. `patternProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-patternproperties)
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
/// - [JSON Schema Core 2020-12 # 10.3.2.3.`additionalProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-additionalproperties)
pub const ADDITIONAL_PROPERTIES: &str = "additionalProperties";

/// ## `propertyNames`
///
/// The value of `propertyNames` MUST be a valid JSON Schema.
///
/// If the instance is an object, this keyword validates if every property
/// name in the instance validates against the provided schema. Note the
/// property name that the schema is testing will always be a string.
///
/// - [JSON Schema Core 2020-12 # 10.3.2.4.`propertyNames`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-propertynames)
pub const PROPERTY_NAMES: &str = "propertyNames";

/// ## `unevaluatedItems`
///
/// The value of `unevaluatedItems` MUST be a valid JSON Schema.
///
/// The behavior of this keyword depends on the annotation results of
/// adjacent keywords that apply to the instance location being validated.
/// Specifically, the annotations from `prefixItems`, `items`, and
/// `contains`, which can come from those keywords when they are adjacent to
/// the `unevaluatedItems` keyword. Those three annotations, as well as
/// `unevaluatedItems`, can also result from any and all adjacent in-place
/// applicator (Section 10.2) keywords. This includes but is not limited to
/// the in-place applicators defined in this document.
///
/// If no relevant annotations are present, the `unevaluatedItems` subschema
/// MUST be applied to all locations in the array. If a boolean true value
/// is present from any of the relevant annotations, `unevaluatedItems` MUST
/// be ignored. Otherwise, the subschema MUST be applied to any index
/// greater than the largest annotation value for `prefixItems`, which does
/// not appear in any annotation value for `contains`.
///
/// This means that `prefixItems`, `items`, `contains`, and all in-place
/// applicators MUST be evaluated before this keyword can be evaluated.
/// Authors of extension keywords MUST NOT define an in-place applicator
/// that would need to be evaluated after this keyword.
///
/// If the `unevaluatedItems` subschema is applied to any positions within
/// the instance array, it produces an annotation result of boolean true,
/// analogous to the behavior of `items`. This annotation affects the
/// behavior of `unevaluatedItems` in parent schemas.
///
/// Omitting this keyword has the same assertion behavior as an empty
/// schema.
///
/// - [JSON Schema Core 2020-12 # 11.2. `unevaluatedItems`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluateditems)
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
/// - [JSON Schema Core 2020-12 # 11.3. `unevaluatedProperties`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-unevaluatedproperties)
pub const UNEVALUATED_PROPERTIES: &str = "unevaluatedProperties";

/// ## `multipleOf`
///
/// The value of `multipleOf` MUST be a number, strictly greater than 0.
///
/// A numeric instance is valid only if division by this keyword's value
/// results in an integer.
///
/// - [JSON Schema Validation 2020-12 # 6.2.1. `multipleOf`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-multipleof)
pub const MULTIPLE_OF: &str = "multipleOf";

/// #`maximum`
///
/// The value of `maximum` MUST be a number, representing an inclusive upper
/// limit for a numeric instance.
///
/// If the instance is a number, then this keyword validates only if the
/// instance is less than or exactly equal to `maximum`.
/// - [JSON Schema Validation 2020-12 # 6.2.2. `maximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maximum)
pub const MAXIMUM: &str = "maximum";

/// ## `exclusiveMaximum`
///
/// For JSON Schema drafts 7 and higher, the value of `exclusiveMaximum` MUST be a number, representing an
/// exclusive upper limit for a numeric instance. For JSON Schema Draft 4, the value of `exclusiveMaximum` MUST
/// be a boolean.
///
///
/// If the instance is a number, then the instance is valid only if it has a
/// value strictly less than (not equal to) `exclusiveMaximum`.
/// - [JSON Schema Validation 2020-12 # 6.2.3. `exclusiveMaximum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusivemaximum)
pub const EXCLUSIVE_MAXIMUM: &str = "exclusiveMaximum";

/// ## `minimum`
///
/// The value of `minimum` MUST be a number, representing an inclusive
/// lower limit for a numeric instance.
///
/// If the instance is a number, then this keyword validates only if the
/// instance is greater than or exactly equal to `minimum`.
/// - [JSON Schema Validation 2020-12 # 6.2.4. `minimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minimum)
pub const MINIMUM: &str = "minimum";

/// ## `exclusiveMinimum`
///
/// For JSON Schema drafts 7 and higher, the value of `exclusiveMinimum` MUST be a number, representing an
/// exclusive lower limit for a numeric instance.
///
/// If the instance is a number, then the instance is valid only if it has a
/// value strictly greater than (not equal to) `exclusiveMinimum`.
///
/// - [JSON Schema Validation 2020-12 # 6.2.5. `exclusiveMinimum`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-exclusiveminimum)
pub const EXCLUSIVE_MINIMUM: &str = "exclusiveMinimum";

/// ## `maxLength`
///
/// The value of `maxLength` MUST be a non-negative integer.
///
/// A string instance is valid against this keyword if its length is less
/// than, or equal to, the value of this keyword.
///
/// - [JSON Schema Validation 2020-12 # 6.3.1. `maxLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxlength)
pub const MAX_LENGTH: &str = "maxLength";

/// ## `minLength`
///
/// The value of this keyword MUST be a non-negative integer.
///
/// A string instance is valid against this keyword if its length is greater
/// than, or equal to, the value of this keyword.
///
/// - [JSON Schema Validation 2020-12 # 6.3.2. `minLength`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minlength)
pub const MIN_LENGTH: &str = "minLength";

/// ## `pattern`
///
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
pub const PATTERN: &str = "pattern";

/// ## `maxItems`
///
/// The value of this keyword MUST be a non-negative integer.
///
/// An array instance is valid against `maxItems` if its size is less than, or equal to, the value of this keyword.
///
/// - [JSON Schema Validation 2020-12 # 6.3.3. `maxItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxitems)
pub const MAX_ITEMS: &str = "maxItems";

/// ## `minItems`
///
/// The value of this keyword MUST be a non-negative integer.
///
/// An array instance is valid against `minItems` if its size is greater than,
/// or equal to, the value of this keyword.
///
/// Omitting this keyword has the same behavior as a value of 0.
/// - [JSON Schema Validation 2020-12 # 6.3.3. `minItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minitems)
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
/// - [JSON Schema Validation 2020-12 # 6.4.3. `uniqueItems`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-uniqueitems)
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
/// - [JSON Schema Validation 2020-12 # 6.4.4. `maxContains`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxcontains)
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
/// - [JSON Schema Validation 2020-12 # 6.4.4. `minContains`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-mincontains)
pub const MIN_CONTAINS: &str = "minContains";

/// ## `maxProperties`
///
/// The value of this keyword MUST be a non-negative integer.
///
/// An object instance is valid against `maxProperties` if its number of properties is less than, or equal to, the value of this keyword.
///
/// - [JSON Schema Validation 2020-12 # 6.5.1 `maxProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-maxproperties)
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
/// - [JSON Schema Validation 2020-12 # 6.5.2 `minProperties`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-minproperties)
pub const MIN_PROPERTIES: &str = "minProperties";

/// ## `required`
///
/// The value of this keyword MUST be an array. Elements of this array, if
/// any, MUST be strings, and MUST be unique.
///
/// An object instance is valid against this keyword if every item in the
/// array is the name of a property in the instance.
///
/// Omitting this keyword has the same behavior as an empty array.
///
/// - [JSON Schema Validation 2020-12 # 6.5.3 `required`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-required)
pub const REQUIRED: &str = "required";

/// ## `dependentRequired`
///
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
pub const DEPENDENT_REQUIRED: &str = "dependentRequired";

/// ## `contentEncoding`
///
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
/// If this keyword is absent, but `contentMediaType` is present, this
/// indicates that the encoding is the identity encoding, meaning that no
/// transformation was needed in order to represent the content in a UTF-8
/// string.
///
/// The value of this property MUST be a string.

/// - [JSON Schema Validation 2020-12 # 8.3. `contentEncoding`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentencoding)
/// - [Understanding JSON Schema # Media: string-encoding non-JSON data - `contentEncoding`](https://json-schema.org/understanding-json-schema/reference/non_json_data.html#id2)
pub const CONTENT_ENCODING: &str = "contentEncoding";

/// ## `contentMediaType`
///
/// If the instance is a string, this property indicates the media type of
/// the contents of the string. If `contentEncoding` is present, this
/// property describes the decoded string.
///
/// The value of this property MUST be a string, which MUST be a media type,
/// as defined by [RFC 2046](https://www.rfc-editor.org/info/rfc2046).
/// - [JSON Schema Validation 2020-12 # 8.4. `contentMediaType`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentmediatype)
pub const CONTENT_MEDIA_TYPE: &str = "contentMediaType";

/// ## `contentSchema`
///
/// If the instance is a string, and if `contentMediaType` is present, this
/// property contains a schema which describes the structure of the string.
///
/// This keyword MAY be used with any media type that can be mapped into
/// JSON Schema's data model.
///
/// The value of this property MUST be a valid JSON schema. It SHOULD be
/// ignored if `contentMediaType` is not present
///
/// - [JSON Schema Validation 2020-12 # 8.5. `contentSchema`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-contentschema)
pub const CONTENT_SCHEMA: &str = "contentSchema";

/// ## `examples`
///
/// The value of this keyword MUST be an array. There are no restrictions placed on the values within the array. When multiple occurrences of this keyword are applicable to a single sub-instance, implementations MUST provide a flat array of all values rather than an array of arrays.
///
/// This keyword can be used to provide sample JSON values associated with a particular schema, for the purpose of illustrating usage. It is RECOMMENDED that these values be valid against the associated schema.
///
/// Implementations MAY use the value(s) of "default", if present, as an
/// additional example. If `examples` is absent, "default" MAY still be used
/// in this manner.
///
/// - [JSON Schema Validation 2020-12 # 9.5 `examples`](https://datatracker.ietf.org/doc/html/draft-bhutton-json-schema-validation-01#name-examples)
pub const EXAMPLES: &str = "examples";
