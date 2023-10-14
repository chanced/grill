use crate::{anymap::AnyMap, AbsoluteUri, Uri};
use dyn_clone::{clone_trait_object, DynClone};
use jsonptr::Pointer;
use serde::{
    de::{self, Unexpected},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize,
};
use serde_json::{Map, Value};
use std::{
    any::Any,
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
    fmt::{self},
    ops::Deref,
    sync::Arc,
};

const ERROR_MSG: &str = "one or more validation errors occurred";
const SUCCESS_MSG: &str = "validation passed";
const EXPECTED_FMT: &str = "a string equal to \"flag\", \"basic\", \"detailed\", or \"verbose\"";
const INSTANCE_LOCATION: &str = "instanceLocation";
const ABSOLUTE_KEYWORD_LOCATION: &str = "absoluteKeywordLocation";
const KEYWORD_LOCATION: &str = "keywordLocation";
const ANNOTATIONS: &str = "annotations";
const ANNOTATION: &str = "annotation";
const ERROR: &str = "error";
const ERRORS: &str = "errors";
const VALID: &str = "valid";
const FMT: &str = "fmt";
const FLAG: &str = "flag";
const BASIC: &str = "basic";
const DETAILED: &str = "detailed";
const VERBOSE: &str = "verbose";

const KEYS: [&str; 7] = [
    ABSOLUTE_KEYWORD_LOCATION,
    ANNOTATIONS,
    ERROR,
    ERRORS,
    INSTANCE_LOCATION,
    KEYWORD_LOCATION,
    VALID,
];

pub type BoxedError<'v> = Box<dyn 'v + Send + Sync + Error<'v>>;

/// An validation error, used as the value of `"error"` in [`Output`](`crate::Output`).
///
///
/// - <https://json-schema.org/draft/2020-12/json-schema-core.html#name-output-formatting>
pub trait Error<'v>: DynClone + std::fmt::Display + std::fmt::Debug + Send + Sync {
    fn make_owned(self: Box<Self>) -> Box<dyn Error<'static>>;
    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        lang: &Translator,
    ) -> std::fmt::Result;
}

clone_trait_object!(<'v> Error<'v>);

impl<'v> Serialize for Box<dyn 'v + Send + Sync + Error<'v>> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Box<dyn Error<'static>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Box::new(String::deserialize(deserializer)?))
    }
}

impl Error<'_> for String {
    fn make_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        self
    }
    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _translator: &Translator,
    ) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl<'v> Error<'v> for &'v str {
    fn make_owned(self: Box<Self>) -> Box<dyn Error<'static>> {
        Box::new(self.to_string())
    }

    fn translate_error(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _translator: &Translator,
    ) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

pub type AnnotationOrError<'v> = Result<Option<Annotation<'v>>, Option<BoxedError<'v>>>;

#[derive(Debug, Clone, Serialize, strum::Display)]
#[serde(untagged)]
pub enum Annotation<'v> {
    Cow(Cow<'v, Value>),
    Arc(Arc<Value>),
}

impl<'de> Deserialize<'de> for Annotation<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(Self::Arc(Arc::from(value)))
    }
}
impl<'v> From<&'v Value> for Annotation<'v> {
    fn from(value: &'v Value) -> Self {
        Self::Cow(Cow::Borrowed(value))
    }
}
impl From<Value> for Annotation<'static> {
    fn from(value: Value) -> Self {
        Self::Cow(Cow::Owned(value))
    }
}
impl From<Arc<Value>> for Annotation<'_> {
    fn from(value: Arc<Value>) -> Self {
        Self::Arc(value)
    }
}
impl From<&Arc<Value>> for Annotation<'_> {
    fn from(value: &Arc<Value>) -> Self {
        Self::Arc(value.clone())
    }
}

impl Deref for Annotation<'_> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Cow(value) => value,
            Self::Arc(value) => value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Translator {
    lang: String,
    map: AnyMap,
}

impl Translator {
    #[must_use]
    pub fn new(lang: String) -> Self {
        Self {
            lang,
            map: AnyMap::new(),
        }
    }

    #[must_use]
    pub fn lang(&self) -> &str {
        &self.lang
    }

    pub fn insert<T>(&mut self, translate: T)
    where
        T: 'static + Clone + std::fmt::Debug + Send + Sync,
    {
        self.map.insert(translate);
    }
    #[must_use]
    pub fn get<T>(&self) -> Option<&T>
    where
        T: Any + std::fmt::Debug + Clone + Send + Sync,
    {
        self.map.get()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, strum::Display,
)]
#[serde(rename_all = "lowercase")]
pub enum Structure {
    /// A concise [`Output`] [`Structure`] which only contains a single
    /// `"valid"` `bool` field.
    ///
    /// This `Structure` may have a positive impact on
    /// performance as [`Keyword`]s are expected to short circuit and return errors as
    /// soon as possible.
    ///
    /// # Example
    /// ```json
    /// { "valid": false }
    /// ```
    ///
    /// - [JSON Schema Core 2020-12 # 12.4.1
    ///   `Flag`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-flag)
    Flag,
    /// The `Basic` structure is a flat list of output units.
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
    Basic,
    /// The `Detailed` structure is based on the schema and can be more readable
    /// for both humans and machines. Having the structure organized this way
    /// makes associations between the errors more apparent. For example, the
    /// fact that the missing "y" property and the extra "z" property both stem
    /// from the same location in the instance is not immediately obvious in the
    /// "Basic" structure. In a hierarchy, the correlation is more easily
    /// identified.
    ///
    /// The following rules govern the construction of the results object:
    ///
    /// - All applicator keywords (`"*Of"`, `"$ref"`, `"if"`/`"then"`/`"else"`,
    ///   etc.) require a node.
    /// - Nodes that have no children are removed.
    /// - Nodes that have a single child are replaced by the child.
    /// - Branch nodes do not require an error message or an annotation.
    ///
    /// # Example
    ///
    /// ## Schema:
    /// ```json
    /// {
    ///   "$id": "https://example.com/polygon",
    ///   "$schema": "https://json-schema.org/draft/2020-12/schema",
    ///   "$defs": {
    ///     "point": {
    ///       "type": "object",
    ///       "properties": {
    ///         "x": { "type": "number" },
    ///         "y": { "type": "number" }
    ///       },
    ///       "additionalProperties": false,
    ///       "required": [ "x", "y" ]
    ///     }
    ///   },
    ///   "type": "array",
    ///   "items": { "$ref": "#/$defs/point" },
    ///   "minItems": 3
    /// }
    /// ```
    /// ## Instance:
    /// ```json
    /// [ { "x": 2.5, "y": 1.3 }, { "x": 1, "z": 6.7 } ]
    /// ```
    /// ## Output:
    ///
    /// ```json
    /// {
    ///   "valid": false,
    ///   "keywordLocation": "",
    ///   "instanceLocation": "",
    ///   "errors": [
    ///     {
    ///       "valid": false,
    ///       "keywordLocation": "/items/$ref",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point",
    ///       "instanceLocation": "/1",
    ///       "errors": [
    ///         {
    ///           "valid": false,
    ///           "keywordLocation": "/items/$ref/required",
    ///           "absoluteKeywordLocation":
    ///             "https://example.com/polygon#/$defs/point/required",
    ///           "instanceLocation": "/1",
    ///           "error": "Required property 'y' not found."
    ///         },
    ///         {
    ///           "valid": false,
    ///           "keywordLocation": "/items/$ref/additionalProperties",
    ///           "absoluteKeywordLocation":
    ///             "https://example.com/polygon#/$defs/point/additionalProperties",
    ///           "instanceLocation": "/1/z",
    ///           "error": "Additional property 'z' found but was invalid."
    ///         }
    ///       ]
    ///     },
    ///     {
    ///       "valid": false,
    ///       "keywordLocation": "/minItems",
    ///       "instanceLocation": "",
    ///       "error": "Expected at least 3 items but found 2"
    ///     }
    ///   ]
    /// }
    ///
    Detailed,
    Verbose,
}

impl Structure {
    /// Returns `true` if the structure is [`Flag`].
    ///
    /// [`Flag`]: Structure::Flag
    #[must_use]
    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag)
    }

    /// Returns `true` if the structure is [`Basic`].
    ///
    /// [`Basic`]: Structure::Basic
    #[must_use]
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic)
    }

    /// Returns `true` if the structure is [`Detailed`].
    ///
    /// [`Detailed`]: Structure::Detailed
    #[must_use]
    pub fn is_detailed(&self) -> bool {
        matches!(self, Self::Detailed)
    }

    /// Returns `true` if the structure is [`Verbose`].
    ///
    /// [`Verbose`]: Structure::Verbose
    #[must_use]
    pub fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose)
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Output                                 ║
║                                ¯¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone)]
pub enum Output<'v> {
    Flag(Flag<'v>),
    Basic(Basic<'v>),
    Detailed(Detailed<'v>),
    Verbose(Verbose<'v>),
}

impl<'v> Output<'v> {
    #[must_use]
    pub fn new(
        structure: Structure,
        absolute_keyword_location: AbsoluteUri,
        keyword_location: Pointer,
        instance_location: Pointer,
        annotation_or_error: AnnotationOrError<'v>,
        is_transient: bool,
    ) -> Self {
        match structure {
            Structure::Flag => Flag::new(annotation_or_error).into(),
            Structure::Basic => Basic::new(
                absolute_keyword_location,
                keyword_location,
                instance_location,
                annotation_or_error,
                is_transient,
            )
            .into(),
            Structure::Detailed => Detailed::new(
                absolute_keyword_location,
                keyword_location,
                instance_location,
                annotation_or_error,
                is_transient,
            )
            .into(),
            Structure::Verbose => Verbose::new(
                absolute_keyword_location,
                keyword_location,
                instance_location,
                annotation_or_error,
                is_transient,
            )
            .into(),
        }
    }
    pub(crate) fn append(&mut self, nodes: impl Iterator<Item = Output<'v>>) {
        for node in nodes {
            self.push(node);
        }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match self {
            Output::Flag(flag) => flag.is_valid,
            Output::Basic(basic) => basic.is_valid,
            Output::Detailed(detailed) => detailed.is_valid,
            Output::Verbose(verbose) => verbose.is_valid,
        }
    }

    /// Appends a node to the output and updates `is_valid` based on the validity
    /// of `output`.
    ///
    /// # Panics
    /// Panics if `output` does not match the variant of `self`. For
    /// evaluations, use [`evaluate`](crate::keyword::Context::evaluate)
    /// [`error`](crate::keyword::Context::error),
    /// [`annotate`](crate::keyword::Context::annotate), or
    /// [`transitive`](crate::keyword::Context::transitive) methods of
    /// [`Context`](crate::keyword::Context) to construct the `Output` to ensure
    /// the variants align.
    pub(crate) fn add(&mut self, output: Output<'v>) {
        let structure = output.structure();
        match self {
            Output::Flag(flag) => flag.add(output.try_into_flag().unwrap_or_else(|_| {
                panic!("Output variant mismatch; expected `Flag`, found {structure}")
            })),
            Output::Basic(basic) => basic.add(output.try_into_basic().unwrap_or_else(|_| {
                panic!("Output variant mismatch; expected `Basic`, found {structure}")
            })),
            Output::Detailed(detailed) => {
                detailed.add(output.try_into_detailed().unwrap_or_else(|_| {
                    panic!("Output variant mismatch; expected `Detailed`, found {structure}")
                }));
            }
            Output::Verbose(verbose) => {
                verbose.add(output.try_into_verbose().unwrap_or_else(|_| {
                    panic!("Output variant mismatch; expected `Verbose`, found {structure}")
                }));
            }
        }
    }
    pub(crate) fn push(&mut self, output: Output<'v>) {
        let structure = output.structure();
        match self {
            Output::Flag(flag) => flag.push(output.try_into_flag().unwrap_or_else(|_| {
                panic!("Output variant mismatch; expected `Flag`, found {structure}")
            })),
            Output::Basic(basic) => basic.push(output.try_into_basic().unwrap_or_else(|_| {
                panic!("Output variant mismatch; expected `Basic`, found {structure}")
            })),
            Output::Detailed(detailed) => {
                detailed.push(output.try_into_detailed().unwrap_or_else(|_| {
                    panic!("Output variant mismatch; expected `Detailed`, found {structure}")
                }));
            }
            Output::Verbose(verbose) => {
                verbose.push(output.try_into_verbose().unwrap_or_else(|_| {
                    panic!("Output variant mismatch; expected `Verbose`, found {structure}")
                }));
            }
        }
    }
    #[must_use]
    pub fn structure(&self) -> Structure {
        match self {
            Output::Flag(_) => Structure::Flag,
            Output::Basic(_) => Structure::Basic,
            Output::Detailed(_) => Structure::Detailed,
            Output::Verbose(_) => Structure::Verbose,
        }
    }
    fn fmt(&self) -> &'static str {
        match self {
            Output::Flag(_) => FLAG,
            Output::Basic(_) => BASIC,
            Output::Detailed(_) => DETAILED,
            Output::Verbose(_) => VERBOSE,
        }
    }

    /// Returns `true` if the output is [`Flag`].
    ///
    /// [`Flag`]: Output::Flag
    #[must_use]
    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag(..))
    }

    #[must_use]
    pub fn as_flag(&self) -> Option<&Flag<'v>> {
        if let Self::Flag(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_flag(self) -> Result<Flag<'v>, Self> {
        if let Self::Flag(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the output is [`Basic`].
    ///
    /// [`Basic`]: Output::Basic
    #[must_use]
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic(..))
    }

    #[must_use]
    pub fn as_basic(&self) -> Option<&Basic<'v>> {
        if let Self::Basic(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_basic(self) -> Result<Basic<'v>, Self> {
        if let Self::Basic(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the output is [`Detailed`].
    ///
    /// [`Detailed`]: Output::Detailed
    #[must_use]
    pub fn is_detailed(&self) -> bool {
        matches!(self, Self::Detailed(..))
    }

    #[must_use]
    pub fn as_detailed(&self) -> Option<&Detailed<'v>> {
        if let Self::Detailed(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn try_into_detailed(self) -> Result<Detailed<'v>, Self> {
        if let Self::Detailed(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    /// Returns `true` if the output is [`Verbose`].
    ///
    /// [`Verbose`]: Output::Verbose
    #[must_use]
    pub fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose(..))
    }

    #[must_use]
    pub fn as_verbose(&self) -> Option<&Verbose<'v>> {
        if let Self::Verbose(v) = self {
            Some(v)
        } else {
            None
        }
    }
    pub fn try_into_verbose(self) -> Result<Verbose<'v>, Self> {
        if let Self::Verbose(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl<'de> Deserialize<'de> for Output<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const EXPECTED: &str = "a JSON Schema Output object";
        let v = Value::deserialize(deserializer)?;

        let obj = match v {
            Value::Object(obj) => Ok(obj),
            Value::Null => Err(de::Error::invalid_value(Unexpected::Option, &EXPECTED)),
            Value::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(b), &EXPECTED)),
            Value::Number(_) => Err(de::Error::invalid_type(
                Unexpected::Other("number"),
                &EXPECTED,
            )),
            Value::String(s) => Err(de::Error::invalid_type(Unexpected::Str(&s), &EXPECTED)),
            Value::Array(_) => Err(de::Error::invalid_type(Unexpected::Seq, &EXPECTED)),
        }?;

        let fmt = determine_fmt(&obj)?;

        match fmt {
            FLAG => Ok(Output::Flag(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            BASIC => Ok(Output::Basic(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            DETAILED => Ok(Output::Detailed(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            VERBOSE => Ok(Output::Verbose(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            _ => unreachable!(),
        }
    }
}

impl Serialize for Output<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(None)?;
        s.serialize_entry(FMT, self.fmt())?;
        match self {
            Output::Flag(flag) => serialize_flag(&mut s, flag),
            Output::Basic(basic) => serialize_basic(&mut s, basic),
            Output::Detailed(detailed) => serialize_detailed(&mut s, detailed),
            Output::Verbose(verbose) => serialize_verbose(&mut s, verbose),
        }?;
        s.end()
    }
}

impl fmt::Display for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Flag(v) => v.fmt(f),
            Output::Basic(v) => v.fmt(f),
            Output::Detailed(v) => v.fmt(f),
            Output::Verbose(v) => v.fmt(f),
        }
    }
}

impl<'v> From<Flag<'v>> for Output<'v> {
    fn from(flag: Flag<'v>) -> Self {
        Self::Flag(flag)
    }
}
impl<'v> From<Basic<'v>> for Output<'v> {
    fn from(basic: Basic<'v>) -> Self {
        Self::Basic(basic)
    }
}
impl<'v> From<Detailed<'v>> for Output<'v> {
    fn from(detailed: Detailed<'v>) -> Self {
        Self::Detailed(detailed)
    }
}
impl<'v> From<Verbose<'v>> for Output<'v> {
    fn from(verbose: Verbose<'v>) -> Self {
        Self::Verbose(verbose)
    }
}

impl<'v, E> From<Output<'v>> for Result<Option<Output<'v>>, E> {
    fn from(value: Output<'v>) -> Self {
        Ok(Some(value))
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Flag                                  ║
║                                 ¯¯¯¯                                  ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░

*/

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag<'v> {
    #[serde(rename = "valid")]
    pub is_valid: bool,
    #[serde(flatten)]
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
}

impl<'v> Flag<'v> {
    #[must_use]
    pub fn new(err_or_annotation: Result<Option<Annotation>, Option<BoxedError<'v>>>) -> Self {
        match err_or_annotation {
            Ok(_) => Self {
                is_valid: true,
                additional_props: BTreeMap::new(),
            },
            Err(_) => Self {
                is_valid: false,
                additional_props: BTreeMap::new(),
            },
        }
    }

    /// Updates `is_valid` based on the validity
    /// of `node` and merges `additional_props` of `node`.
    pub fn add(&mut self, node: Flag<'v>) {
        self.is_valid &= node.is_valid;
        self.push(node);
    }
    /// Appends `node` to the output but does *not* update `is_valid`
    pub fn push(&mut self, mut node: Flag<'v>) {
        self.additional_props.append(&mut node.additional_props);
    }
}

impl Serialize for Flag<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(None)?;
        serialize_flag(&mut s, self)?;
        s.end()
    }
}

impl fmt::Display for Flag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid {
            write!(f, "{SUCCESS_MSG}")
        } else {
            write!(f, "{ERROR_MSG}")
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Basic                                 ║
║                                 ¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Clone, Debug)]
pub struct BasicNode<'v> {
    pub is_valid: bool,
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub absolute_keyword_location: Uri,
    pub annotation_or_error: AnnotationOrError<'v>,
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
}

impl<'v> Serialize for BasicNode<'v> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(None)?;
        s.serialize_entry(INSTANCE_LOCATION, &self.instance_location)?;
        s.serialize_entry(VALID, &self.is_valid)?;
        s.serialize_entry(KEYWORD_LOCATION, &self.keyword_location)?;
        s.serialize_entry(ABSOLUTE_KEYWORD_LOCATION, &self.absolute_keyword_location)?;
        serialize_annotation_or_error(&mut s, self.annotation_or_error.as_ref())?;
        serialize_additional_props(&mut s, self.additional_props.iter())?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for BasicNode<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data<'v> {
            instance_location: Pointer,
            keyword_location: Pointer,
            absolute_keyword_location: Uri,
            #[serde(alias = "annotation", alias = "error")]
            annotation_or_error: Option<Value>,
            #[serde(rename = "valid")]
            is_valid: bool,
            #[serde(flatten)]
            additional_props: BTreeMap<String, Cow<'v, Value>>,
        }
        let Data {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            annotation_or_error,
            is_valid,
            additional_props,
        } = Data::deserialize(deserializer)?;
        let annotation_or_error =
            deserialize_annotation_or_error::<D>(annotation_or_error, is_valid)?;
        Ok(BasicNode {
            is_valid,
            instance_location,
            keyword_location,
            absolute_keyword_location,
            annotation_or_error,
            additional_props,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Basic<'v> {
    pub is_valid: bool,
    pub nodes: Vec<BasicNode<'v>>,
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
    pub is_transient: bool,
}

impl<'v> Basic<'v> {
    #[must_use]
    pub fn new(
        absolute_keyword_location: AbsoluteUri,
        keyword_location: Pointer,
        instance_location: Pointer,
        annotation_or_error: AnnotationOrError<'v>,
        is_transient: bool,
    ) -> Self {
        let is_valid = annotation_or_error.is_ok();
        let absolute_keyword_location = absolute_keyword_location.into();
        let additional_props = BTreeMap::default();
        let nodes = if is_transient {
            Vec::new()
        } else {
            vec![BasicNode {
                instance_location,
                keyword_location,
                absolute_keyword_location,
                annotation_or_error,
                is_valid,
                additional_props: BTreeMap::default(),
            }]
        };

        Self {
            is_valid,
            nodes,
            additional_props,
            is_transient,
        }
    }

    /// Appends nodes of `node` to the output and updates `is_valid` based on
    /// the validity.
    pub fn add(&mut self, node: Basic<'v>) {
        self.is_valid &= node.is_valid;
    }

    /// Appends `node` to the output but does *not* update `is_valid`
    pub(crate) fn push(&mut self, mut other: Basic<'v>) {
        self.nodes.append(&mut other.nodes);
    }
}

impl<'de> Deserialize<'de> for Basic<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data<'v> {
            #[serde(rename = "valid")]
            is_valid: bool,
            #[serde(alias = "annotations", alias = "errors", default)]
            nodes: Vec<BasicNode<'static>>,
            #[serde(flatten)]
            additional_props: BTreeMap<String, Cow<'v, Value>>,
        }
        let Data {
            is_valid,
            nodes,
            additional_props,
        } = Data::deserialize(deserializer)?;
        Ok(Basic {
            is_valid,
            nodes,
            additional_props,
            is_transient: false,
        })
    }
}
impl Serialize for Basic<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(None)?;
        serialize_basic(&mut s, self)?;
        s.end()
    }
}

impl fmt::Display for Basic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid {
            write!(f, "{SUCCESS_MSG}")
        } else {
            write!(f, "{ERROR_MSG}")
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Detailed                                ║
║                               ¯¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone)]
pub struct Detailed<'v> {
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub absolute_keyword_location: Option<Uri>,
    pub is_valid: bool,
    annotation_or_error: AnnotationOrError<'v>,
    pub nodes: Vec<Detailed<'v>>,
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
    /// Indicates that this node is not part of the final output and is only
    /// used to store intermediate results.
    ///
    /// This is primarily for `if` / `then` / `else` branches but may be relevant
    /// for future or external keywords.
    pub is_transient: bool,
}

impl<'v> Detailed<'v> {
    #[must_use]
    pub fn new(
        absolute_keyword_location: AbsoluteUri,
        keyword_location: Pointer,
        instance_location: Pointer,
        annotation_or_error: AnnotationOrError<'v>,
        is_transient: bool,
    ) -> Self {
        let is_valid = annotation_or_error.is_ok();
        let absolute_keyword_location = Some(absolute_keyword_location.into());
        Self {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            is_valid,
            annotation_or_error,
            nodes: Vec::new(),
            additional_props: BTreeMap::default(),
            is_transient,
        }
    }
    /// Appends `node` to the output and updates `is_valid` based on the validity
    /// of `output`.
    pub fn add(&mut self, node: Detailed<'v>) {
        self.is_valid &= node.is_valid;
        self.push(node);
    }

    /// Appends `node` to the output but does *not* update `is_valid`.
    pub(crate) fn push(&mut self, node: Detailed<'v>) {
        if node.is_transient {
            self.nodes.extend(node.nodes);
            self.additional_props.extend(node.additional_props);
        } else {
            self.nodes.push(node);
        }
    }
}

impl Serialize for Detailed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // if len is > 0, it has no impact on serialization
        let mut s = serializer.serialize_map(None)?;
        serialize_detailed(&mut s, self)?;
        s.end()
    }
}

impl<'de> Deserialize<'de> for Detailed<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Data<'v> {
            instance_location: Pointer,
            keyword_location: Pointer,
            absolute_keyword_location: Option<Uri>,
            #[serde(alias = "annotation", alias = "error")]
            annotation_or_error: Option<Value>,
            #[serde(alias = "annotations", alias = "errors", default)]
            nodes: Vec<Detailed<'static>>,
            #[serde(rename = "valid")]
            is_valid: bool,
            #[serde(flatten)]
            additional_props: BTreeMap<String, Cow<'v, Value>>,
        }
        let Data {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            annotation_or_error: detail,
            nodes,
            is_valid,
            additional_props,
        } = Data::deserialize(deserializer)?;
        let annotation_or_error =
            deserialize_annotation_or_error::<D>(detail, is_valid).map_err(de::Error::custom)?;
        Ok(Detailed {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            annotation_or_error,
            nodes,
            is_valid,
            additional_props,
            is_transient: false,
        })
    }
}

impl fmt::Display for Detailed<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.annotation_or_error {
            Ok(Some(v)) => write!(f, "{v}"),
            Err(Some(v)) => write!(f, "{v}"),
            Ok(_) => write!(f, "{SUCCESS_MSG}"),
            Err(_) => write!(f, "{ERROR_MSG}"),
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Verbose                                 ║
║                               ¯¯¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone)]
pub struct Verbose<'v> {
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub absolute_keyword_location: Option<Uri>,
    pub annotation_or_error: AnnotationOrError<'v>,
    pub nodes: Vec<Verbose<'v>>,
    pub is_valid: bool,
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
    /// Indicates that this node is not part of the final output and is only
    /// used to store intermediate results.
    ///
    /// This is primarily for `if` / `then` / `else` branches but may be relevant
    /// for future or external keywords.
    pub is_transient: bool,
}
impl<'v> Verbose<'v> {
    #[must_use]
    pub fn new(
        absolute_keyword_location: AbsoluteUri,
        keyword_location: Pointer,
        instance_location: Pointer,
        annotation_or_error: AnnotationOrError<'v>,
        is_transient: bool,
    ) -> Self {
        let is_valid = annotation_or_error.is_ok();
        let absolute_keyword_location = Some(absolute_keyword_location.into());
        Self {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            is_valid,
            annotation_or_error,
            nodes: Vec::new(),
            additional_props: BTreeMap::default(),
            is_transient,
        }
    }
    pub fn add(&mut self, node: Verbose<'v>) {
        self.is_valid &= node.is_valid;
        self.push(node);
    }
    pub(crate) fn push(&mut self, node: Verbose<'v>) {
        if node.is_transient {
            self.nodes.extend(node.nodes);
            self.additional_props.extend(node.additional_props);
        } else {
            self.nodes.push(node);
        }
    }
}

impl<'v> fmt::Display for Verbose<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        serde_json::to_string_pretty(self)
            .map_err(|_| fmt::Error)?
            .fmt(f)
    }
}

impl<'de> Deserialize<'de> for Verbose<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Data {
            instance_location: Pointer,
            keyword_location: Pointer,
            absolute_keyword_location: Option<Uri>,
            #[serde(alias = "annotation", alias = "error")]
            annotation_or_error: Option<Value>,
            #[serde(alias = "annotations", alias = "errors", default)]
            nodes: Vec<Verbose<'static>>,
            #[serde(rename = "valid")]
            is_valid: bool,
            #[serde(flatten)]
            additional_props: BTreeMap<String, Cow<'static, Value>>,
        }

        let Data {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            annotation_or_error,
            nodes,
            is_valid,
            additional_props,
        } = Data::deserialize(deserializer)?;
        let annotation_or_error =
            deserialize_annotation_or_error::<D>(annotation_or_error, is_valid)?;
        Ok(Verbose {
            instance_location,
            keyword_location,
            absolute_keyword_location,
            annotation_or_error,
            nodes,
            is_valid,
            additional_props,
            is_transient: false,
        })
    }
}

impl Serialize for Verbose<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_map(None)?;
        serialize_verbose(&mut s, self)?;
        s.end()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                           Internal Functions                          ║
║                           ¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯                          ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

fn serialize_additional_props<'a, S: SerializeMap>(
    s: &mut S,
    additional_props: impl Iterator<Item = (&'a String, &'a Cow<'a, Value>)>,
) -> Result<(), S::Error> {
    let additional_props =
        additional_props.filter(|(key, _)| KEYS.binary_search(&key.as_str()).is_err());

    for (key, value) in additional_props {
        s.serialize_entry(key, value)?;
    }

    Ok(())
}

fn serialize_option<S: SerializeMap>(
    s: &mut S,
    key: &'static str,
    value: Option<&impl Serialize>,
) -> Result<(), S::Error> {
    if let Some(value) = value {
        s.serialize_entry(key, value)?;
    }
    Ok(())
}
fn serialize_nodes<S: SerializeMap>(
    s: &mut S,
    value: &[impl Serialize],
    valid: bool,
) -> Result<(), S::Error> {
    if value.is_empty() {
        return Ok(());
    }
    let key = if valid { "annotations" } else { "errors" };
    s.serialize_entry(key, value)
}

#[allow(clippy::unnecessary_wraps)]
fn hierarchical_fmt<E: de::Error>(obj: &Map<String, Value>) -> Result<&'_ str, E> {
    if contains_mixed(obj) {
        return Ok(VERBOSE);
    }
    Ok(DETAILED)
}

fn fmt_from_str<E>(v: &Value) -> Result<&'_ str, E>
where
    E: de::Error,
{
    let fmt = fmt_as_str(v)?;
    match fmt {
        FLAG | BASIC | DETAILED | VERBOSE => Ok(fmt),
        _ => {
            return Err(de::Error::invalid_value(
                Unexpected::Str(fmt),
                &EXPECTED_FMT,
            ))
        }
    }
}

fn fmt_as_str<E>(v: &Value) -> Result<&'_ str, E>
where
    E: de::Error,
{
    match v {
        Value::String(s) => Ok(s),
        Value::Null => Err(de::Error::invalid_value(Unexpected::Option, &EXPECTED_FMT)),
        Value::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(*b), &EXPECTED_FMT)),
        Value::Number(n) => Err(de::Error::invalid_type(
            Unexpected::Other(&format!("number {n}")),
            &EXPECTED_FMT,
        )),
        Value::Array(_) => Err(de::Error::invalid_type(Unexpected::Seq, &EXPECTED_FMT)),
        Value::Object(_) => Err(de::Error::invalid_type(Unexpected::Map, &EXPECTED_FMT)),
    }
}

fn has_nodes(obj: &Map<String, Value>) -> bool {
    get_nodes(obj).is_some()
}
fn get_nodes(obj: &Map<String, Value>) -> Option<&Vec<Value>> {
    obj.get(ERRORS)
        .or_else(|| obj.get(ANNOTATIONS))
        .and_then(Value::as_array)
}

fn is_hierarchical(obj: &Map<String, Value>) -> bool {
    let Some(nodes) = get_nodes(obj) else {
        return false;
    };
    nodes
        .iter()
        .any(|v| v.get(ERRORS).or_else(|| v.get(ANNOTATIONS)).is_some())
}

fn contains_mixed(obj: &Map<String, Value>) -> bool {
    let mut has_errors = false;
    let mut has_annotations = false;
    let mut queue = VecDeque::new();
    queue.push_back(obj);
    while !queue.is_empty() {
        let obj = queue.pop_front().unwrap();
        if obj.contains_key(ERROR) || obj.contains_key(ERRORS) {
            has_errors = true;
        }
        if obj.contains_key(ANNOTATION) || obj.contains_key(ANNOTATIONS) {
            has_annotations = true;
        }
        if has_annotations && has_errors {
            return true;
        }
        if let Some(errors) = obj.get(ERRORS).and_then(Value::as_array) {
            queue.extend(errors.iter().filter_map(Value::as_object));
            continue;
        }
        if let Some(annotations) = obj.get(ANNOTATIONS).and_then(Value::as_array) {
            queue.extend(annotations.iter().filter_map(Value::as_object));
            continue;
        }
    }
    false
}
#[allow(clippy::type_complexity)]
fn deserialize_annotation_or_error<'de, D: Deserializer<'de>>(
    annotation_or_error: Option<Value>,
    is_valid: bool,
) -> Result<AnnotationOrError<'static>, D::Error> {
    if is_valid {
        return Ok(Ok(annotation_or_error.map(Into::into)));
    }
    if annotation_or_error.is_none() {
        if is_valid {
            return Ok(Ok(None));
        }
        return Ok(Err(None));
    }
    let annotation_or_error = annotation_or_error.unwrap();
    match annotation_or_error {
        Value::String(s) => {
            if is_valid {
                Ok(Ok(Some(Arc::new(Value::String(s)).into())))
            } else {
                Ok(Err(Some(Box::new(s))))
            }
        }
        Value::Null => {
            if is_valid {
                return Ok(Ok(None));
            }
            Ok(Err(None))
        }
        Value::Bool(b) => {
            if is_valid {
                Ok(Ok(Some(Arc::new(Value::Bool(b)).into())))
            } else {
                Err(de::Error::invalid_type(
                    Unexpected::Bool(b),
                    &"a valid error message",
                ))
            }
        }
        Value::Number(n) => {
            if is_valid {
                Ok(Ok(Some(Arc::new(Value::Number(n)).into())))
            } else {
                Err(de::Error::invalid_type(
                    Unexpected::Other(&format!("number {n}")),
                    &"a valid error message",
                ))
            }
        }
        Value::Array(arr) => {
            if is_valid {
                Ok(Ok(Some(Arc::new(Value::Array(arr)).into())))
            } else {
                Err(de::Error::invalid_type(
                    Unexpected::Seq,
                    &"a valid error message",
                ))
            }
        }
        Value::Object(obj) => {
            if is_valid {
                Ok(Ok(Some(Arc::new(Value::Object(obj)).into())))
            } else {
                Err(de::Error::invalid_type(
                    Unexpected::Map,
                    &"a valid error message",
                ))
            }
        }
    }
}

fn serialize_annotation_or_error<'v, S: SerializeMap>(
    s: &mut S,
    annotation_or_error: Result<&Option<Annotation<'v>>, &Option<BoxedError<'v>>>,
) -> Result<(), S::Error> {
    match annotation_or_error {
        Ok(Some(annotation)) => s.serialize_entry(ANNOTATION, &annotation)?,
        Err(Some(error)) => s.serialize_entry(ERROR, error)?,
        _ => {}
    };
    Ok(())
}

fn serialize_flag<S: SerializeMap>(s: &mut S, flag: &Flag<'_>) -> Result<(), S::Error> {
    s.serialize_entry(VALID, &flag.is_valid)?;
    serialize_additional_props(s, flag.additional_props.iter())?;
    Ok(())
}

fn serialize_basic<S: SerializeMap>(s: &mut S, basic: &Basic<'_>) -> Result<(), S::Error> {
    s.serialize_entry(VALID, &basic.is_valid)?;
    serialize_nodes(s, &basic.nodes, basic.is_valid)?;
    serialize_additional_props(s, basic.additional_props.iter())?;
    Ok(())
}

fn serialize_detailed<S: SerializeMap>(s: &mut S, detailed: &Detailed<'_>) -> Result<(), S::Error> {
    s.serialize_entry(VALID, &detailed.is_valid)?;
    serialize_annotation_or_error(s, detailed.annotation_or_error.as_ref())?;
    s.serialize_entry(INSTANCE_LOCATION, &detailed.instance_location)?;
    s.serialize_entry(KEYWORD_LOCATION, &detailed.keyword_location)?;
    serialize_option(
        s,
        ABSOLUTE_KEYWORD_LOCATION,
        detailed.absolute_keyword_location.as_ref(),
    )?;
    serialize_nodes(s, &detailed.nodes, detailed.is_valid)?;
    serialize_additional_props(s, detailed.additional_props.iter())?;
    Ok(())
}
fn serialize_verbose<S: SerializeMap>(s: &mut S, verbose: &Verbose<'_>) -> Result<(), S::Error> {
    s.serialize_entry(VALID, &verbose.is_valid)?;
    serialize_annotation_or_error(s, verbose.annotation_or_error.as_ref())?;
    s.serialize_entry(INSTANCE_LOCATION, &verbose.instance_location)?;
    s.serialize_entry(KEYWORD_LOCATION, &verbose.keyword_location)?;
    serialize_option(
        s,
        ABSOLUTE_KEYWORD_LOCATION,
        verbose.absolute_keyword_location.as_ref(),
    )?;
    serialize_nodes(s, &verbose.nodes, verbose.is_valid)?;
    serialize_additional_props(s, verbose.additional_props.iter())?;
    Ok(())
}

fn determine_fmt<E: de::Error>(obj: &Map<String, Value>) -> Result<&'_ str, E> {
    let fmt = obj.get(FMT);
    if let Some(fmt) = fmt {
        return fmt_from_str(fmt);
    }
    if is_hierarchical(obj) {
        return hierarchical_fmt(obj);
    }
    if has_nodes(obj) {
        return Ok(BASIC);
    }
    Ok(FLAG)
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Tests                                 ║
║                                 ¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize_verbose() {
        let json = r#"
        {
            "valid": false,
            "keywordLocation": "",
            "instanceLocation": "",
            "errors": [
              {
                "valid": true,
                "keywordLocation": "/$defs",
                "instanceLocation": ""
              },
              {
                "valid": true,
                "keywordLocation": "/type",
                "instanceLocation": ""
              },
              {
                "valid": false,
                "keywordLocation": "/items",
                "instanceLocation": "",
                "errors": [
                  {
                    "valid": true,
                    "keywordLocation": "/items/$ref",
                    "absoluteKeywordLocation":
                      "https://example.com/polygon#/items/$ref",
                    "instanceLocation": "/0",
                    "annotations": [
                      {
                        "valid": true,
                        "keywordLocation": "/items/$ref",
                        "absoluteKeywordLocation":
                          "https://example.com/polygon#/$defs/point",
                        "instanceLocation": "/0",
                        "annotations": [
                          {
                            "valid": true,
                            "keywordLocation": "/items/$ref/type",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/type",
                            "instanceLocation": "/0"
                          },
                          {
                            "valid": true,
                            "keywordLocation": "/items/$ref/properties",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/properties",
                            "instanceLocation": "/0"
                          },
                          {
                            "valid": true,
                            "keywordLocation": "/items/$ref/required",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/required",
                            "instanceLocation": "/0"
                          },
                          {
                            "valid": true,
                            "keywordLocation": "/items/$ref/additionalProperties",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/additionalProperties",
                            "instanceLocation": "/0"
                          }
                        ]
                      }
                    ]
                  },
                  {
                    "valid": false,
                    "keywordLocation": "/items/$ref",
                    "absoluteKeywordLocation":
                      "https://example.com/polygon#/items/$ref",
                    "instanceLocation": "/1",
                    "errors": [
                      {
                        "valid": false,
                        "keywordLocation": "/items/$ref",
                        "absoluteKeywordLocation":
                          "https://example.com/polygon#/$defs/point",
                        "error": "an example error",
                        "instanceLocation": "/1",
                        "errors": [
                          {
                            "valid": true,
                            "keywordLocation": "/items/$ref/type",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/type",
                            "instanceLocation": "/1"
                          },
                          {
                            "valid": true,
                            "keywordLocation": "/items/$ref/properties",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/properties",
                            "annotation": {"v": 1},
                            "instanceLocation": "/1"
                          },
                          {
                            "valid": false,
                            "keywordLocation": "/items/$ref/required",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/required",
                            "instanceLocation": "/1"
                          },
                          {
                            "valid": false,
                            "keywordLocation": "/items/$ref/additionalProperties",
                            "absoluteKeywordLocation":
                              "https://example.com/polygon#/$defs/point/additionalProperties",
                            "instanceLocation": "/1",
                            "errors": [
                              {
                                "valid": false,
                                "keywordLocation": "/items/$ref/additionalProperties",
                                "absoluteKeywordLocation":
                                  "https://example.com/polygon#/$defs/point/additionalProperties",
                                "instanceLocation": "/1/z"
                              }
                            ]
                          }
                        ]
                      }
                    ]
                  }
                ]
              },
              {
                "valid": false,
                "keywordLocation": "/minItems",
                "instanceLocation": ""
              }
            ]
          }
        "#;

        let op: Output = serde_json::from_str(json).unwrap();
        assert!(matches!(op, Output::Verbose(_)));
    }
}
