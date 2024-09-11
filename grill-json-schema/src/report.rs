use crate::{
    spec::{self, ShouldSerialize},
    IntoOwned,
};

pub use self::{basic::Basic, flag::Flag, verbose::Verbose};
use grill_uri::AbsoluteUri;
use jsonptr::{Pointer, PointerBuf};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{
    borrow::Cow,
    fmt::{self, Debug},
};

// /// Set of keywords to check which disable short-circuiting
// pub const DISABLING_KEYWORDS: [&'static str; 2] = [UNEVALUATED_PROPERTIES, UNEVALUATED_ITEMS];

// if Self::ENABLING_STRUCTURES.contains(ctx.structure().into()) {
//     ctx.enable_short_circuiting();
// }
// pub const ENABLING_STRUCTURES: Structures = Structures::FLAG;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Output                                ║
║                                ¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// The output structure of a [`Report`].
///
/// [JSON Schema Core 2020-12 #12.4 Output
/// Structure](https://json-schema.org/draft/2020-12/json-schema-core#name-output-structure)
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, strum::Display,
)]
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                              Annotation                               ║
║                             ¯¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

///  
#[derive(Clone, Debug, serde::Serialize)]
#[serde(untagged)]
pub enum Annotation<'v> {
    Schema(AbsoluteUri),
    Unknown(Cow<'v, Value>),
}
impl ShouldSerialize for Annotation<'_> {
    fn should_serialize(&self) -> bool {
        todo!()
    }
}
impl From<Value> for Annotation<'_> {
    fn from(v: Value) -> Self {
        Self::Unknown(Cow::Owned(v))
    }
}

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Error                                  ║
║                               ¯¯¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error<'v> {
    X(Cow<'v, str>),
    Unknown(String),
}
impl IntoOwned for Error<'_> {
    type Owned = Error<'static>;
    fn into_owned(self) -> Self::Owned {
        match self {
            Error::X(x) => Error::X(x.into_owned().into()),
            Error::Unknown(u) => Error::Unknown(u),
        }
    }
}
impl From<String> for Error<'_> {
    fn from(s: String) -> Self {
        Error::Unknown(s)
    }
}
impl std::error::Error for Error<'_> {}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Report                                ║
║                                ¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub type DefaultReport<'v> = Report<Annotation<'v>, Error<'v>>;

#[derive(Debug, Clone)]
pub enum Report<A, E> {
    Flag(Flag),
    Basic(Basic<A, E>),
    Verbose(Verbose<A, E>),
}
impl<A, E> Report<A, E> {
    pub fn as_flag(&self) -> Option<&Flag> {
        match self {
            Report::Flag(f) => Some(f),
            _ => None,
        }
    }
    pub fn as_basic(&self) -> Option<&Basic<A, E>> {
        match self {
            Report::Basic(b) => Some(b),
            _ => None,
        }
    }
    pub fn as_verbose(&self) -> Option<&Verbose<A, E>> {
        match self {
            Report::Verbose(v) => Some(v),
            _ => None,
        }
    }
}

impl<A, E> Serialize for Report<A, E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'de, 'v, A, E> Deserialize<'de> for Report<A, E>
where
    A: From<Value>,
    E: From<String>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl<'v, A, E> std::fmt::Display for Report<A, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'v, A, E> std::error::Error for Report<A, E>
where
    A: Debug,
    E: Debug,
{
}
impl<'v, A, E> spec::Report<'v, A, E> for Report<A, E>
where
    A: 'v + From<Annotation<'v>> + From<Value> + Debug + Send,
    E: 'v + From<Error<'v>> + From<String> + Debug + Send,
{
    type Assess<'val> = Assess<'val, A, E> where Self: 'val;

    fn new(output: Output, location: Location) -> Self {
        todo!()
    }
    fn is_valid(&self) -> bool {
        match self {
            Report::Flag(f) => f.is_valid(),
            Report::Basic(b) => b.is_valid(),
            Report::Verbose(v) => v.is_valid(),
        }
    }
    fn assess(&mut self, location: Location) -> Assess<'_, A, E> {
        match self {
            Report::Flag(f) => f.assess(location),
            Report::Basic(b) => b.assess(location),
            Report::Verbose(v) => v.assess(location),
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Flag                                  ║
║                                ¯¯¯¯¯¯                                 ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub mod flag {
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};

    use super::{Assess, Location};

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Flag {
        /// The validity of the schema.
        pub valid: bool,
        /// Additional properties.
        #[serde(default, flatten)]
        pub additional_properties: Option<Map<String, Value>>,
    }

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Basic                                 ║
║                                ¯¯¯¯¯¯¯                                ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub mod basic {

    use super::*;

    #[derive(Clone, Debug)]
    pub struct Basic<A, E> {
        valid: bool,
        assessments: Vec<Assessment<A, E>>,
    }
    impl<A, E> Basic<A, E> {
        pub fn assessments(&self) -> &[Assessment<A, E>] {
            &self.assessments
        }
        pub fn assess(&mut self, location: Location) -> Assess<'_, A, E> {
            todo!()
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
                assessments: vec![Assessment::Annotation {
                    annotation: None,
                    location,
                }],
                valid: true,
            }
        }
    }
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Verbose                                ║
║                               ¯¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub mod verbose {
    use super::AbsoluteUri;

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Verbose<A, E> {
        #[serde(flatten)]
        pub location: super::Location,
        #[serde(flatten)]
        pub detail: Assessment<A, E>,
    }

    impl<A, E> Verbose<A, E> {
        pub fn location(&self) -> &super::Location {
            &self.location
        }
        pub fn is_valid(&self) -> bool {
            matches!(self.detail, Assessment::Annotation { .. })
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
            &self.location.instance
        }
        pub fn keyword_location(&self) -> &jsonptr::Pointer {
            &self.location.keyword
        }
        pub fn absolute_keyword_location(&self) -> &AbsoluteUri {
            &self.location.absolute_keyword
        }

        pub fn assess<'val>(&'val self, location: super::Location) -> super::Assess<'val, A, E> {
            todo!()
        }
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                Location                               ║
║                               ¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A keyword location within a [`Report`]
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    /// The location of the instance within the JSON document.
    #[serde(rename = "instanceLocation")]
    pub instance: PointerBuf,

    /// The location of the keyword within the JSON Schema.
    #[serde(rename = "keywordLocation")]
    pub keyword: PointerBuf,

    /// The absolute location of the keyword within the JSON Schema.
    #[serde(rename = "absoluteKeywordLocation")]
    pub absolute_keyword: AbsoluteUri,
}

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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Assess                                ║
║                                ¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

/// A mutable reference to a node in a [`Report`].
pub enum Assess<'rpt, A, E> {
    /// A concise structure which only contains a single `"valid"` `bool` field.
    ///
    /// - [JSON Schema Core 2020-12 # 12.4.1
    ///   `Flag`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-flag)
    Flag(&'rpt mut Flag),

    /// A flat list of output units.
    ///
    /// - [JSON Schema Core 2020-12 # 12.4.2
    ///   Basic](https://json-schema.org/draft/2020-12/json-schema-core#name-basic)
    Basic(&'rpt mut basic::Assessment<A, E>),

    /// A tree structure of output units.
    ///
    /// - [JSON Schema Core 2020-12 # 12.4.4
    ///   Verbose](https://json-schema.org/draft/2020-12/json-schema-core#name-verbose)
    Verbose(&'rpt mut Verbose<A, E>),
}

impl<'rpt, A, E> spec::Assess<'rpt, A, E> for Assess<'rpt, A, E> {
    /// Depending on the variant, may set the [`Annotation`] of the current assessment
    /// and return the previous value, if present.
    ///
    /// - [`Basic`] and [`Verbose`]: sets the [`Annotation`]
    /// - [`Flag`]: discards `annotation`
    fn annotate(&mut self, annotation: A) -> Option<A> {
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
    fn fail(&mut self, error: E) {
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

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Translate                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub trait Translate<E> {
    fn translate(&self, error: E) -> String;
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               Translated                              ║
║                              ¯¯¯¯¯¯¯¯¯¯¯¯                             ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

pub struct Translated<'t, T, R> {
    pub report: R,
    pub translator: &'t T,
}
