pub use self::{basic::Basic, flag::Flag, verbose::Verbose};
use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{borrow::Cow, fmt::Debug};

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

#[derive(Debug, Clone)]
pub enum Report<A, E> {
    Flag(Flag),
    Basic(Basic<A, E>),
    Verbose(Verbose<A, E>),
}
impl<A, E> std::fmt::Display for Report<A, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
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
        todo!()
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
    use grill_uri::AbsoluteUri;
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
                    location: location,
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
    impl<A, E> Verbose<A, E> {
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
            todo!()
        }
        pub fn keyword_location(&self) -> &jsonptr::Pointer {
            &self.location.keyword
        }
        pub fn absolute_keyword_location(&self) -> &AbsoluteUri {
            &self.location.absolute_keyword
        }

        pub fn assess<'r>(&'r self, location: super::Location) -> super::Assess<'r, A, E> {
            todo!()
        }
    }
}

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                               IntoOwned                               ║
║                              ¯¯¯¯¯¯¯¯¯¯¯                              ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/
/// A trait implemented by types that can be converted into an owned type.
pub trait IntoOwned {
    /// The owned type.
    type Owned: 'static;
    /// Consumes `self`, returning `Self::Owned`.
    fn into_owned(self) -> Self::Owned;
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
    pub instance: jsonptr::Pointer,

    /// The location of the keyword within the JSON Schema.
    #[serde(rename = "keywordLocation")]
    pub keyword: jsonptr::Pointer,

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
    fn translate(&self, error: E) -> E;
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

pub struct Translated<'t, T, A, E> {
    pub report: Report<A, E>,
    pub translator: &'t T,
}
