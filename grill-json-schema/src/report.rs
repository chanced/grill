pub use self::{basic::Basic, flag::Flag, verbose::Verbose};
use grill_core::{criterion, Schema};
use grill_uri::AbsoluteUri;
use jsonptr::Pointer;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::borrow::Cow;

/*
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
╔═══════════════════════════════════════════════════════════════════════╗
║                                                                       ║
║                                 Output                                ║
║                                ¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

// /// Set of keywords to check which disable short-circuiting
// pub const DISABLING_KEYWORDS: [&'static str; 2] = [UNEVALUATED_PROPERTIES, UNEVALUATED_ITEMS];

// if Self::ENABLING_STRUCTURES.contains(ctx.structure().into()) {
//     ctx.enable_short_circuiting();
// }
// pub const ENABLING_STRUCTURES: Structures = Structures::FLAG;
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, strum::Display,
)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Output {
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
    Flag = 1,
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
    Basic = 2,
    Verbose = 8,
}

impl criterion::Output for Output {
    fn verbose() -> Self {
        Self::Verbose
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
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(rename = "instanceLocation")]
    pub instance: jsonptr::Pointer,
    #[serde(rename = "keywordLocation")]
    pub keyword: jsonptr::Pointer,
    #[serde(rename = "absoluteKeywordLocation")]
    pub absolute_keyword: AbsoluteUri,
}
impl Location {
    pub fn instance(&self) -> &jsonptr::Pointer {
        &self.instance
    }
    pub fn keyword(&self) -> &jsonptr::Pointer {
        &self.keyword
    }
    pub fn absolute_keyword(&self) -> &AbsoluteUri {
        &self.absolute_keyword
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

pub enum Assess<'v, 'r> {
    Flag(&'r mut Flag<'v>),
    Basic(&'r mut basic::Assessment<'v>),
    Verbose(&'r mut Verbose<'v>),
}
impl<'v, 'r> Assess<'v, 'r> {
    /// Depending on the variant, may set the [`Annotation`] of the current assessment
    /// and return the previous value, if present.
    ///
    /// - [`Basic`] and [`Verbose`]: sets the [`Annotation`]
    /// - [`Flag`]: discards `annotation`
    pub fn annotate(&mut self, annotation: Annotation<'v>) -> Option<Annotation<'v>> {
        match self {
            Assess::Flag(flag) => None,
            Assess::Basic(b) => b.set_annotation(annotation),
            Assess::Verbose(v) => v.set_annotation(annotation),
        }
    }
    /// For all variants, sets `valid` to `false`. Depending on the variant, may also
    /// set the [`Error`] of the current assessment and the previous value, if present.
    ///
    /// - [`Basic`] and [`Verbose`]: sets the [`Error`]
    /// - [`Flag`]: discards `error`
    pub fn fail(&mut self, error: Error<'v>) {
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
║                                 Report                                ║
║                                ¯¯¯¯¯¯¯¯                               ║
╚═══════════════════════════════════════════════════════════════════════╝
░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
*/

#[derive(Debug, Clone)]
pub enum Report<'v> {
    Flag(Flag<'v>),
    Basic(Basic<'v>),
    Verbose(Verbose<'v>),
}

impl std::error::Error for Report<'_> {}

impl<'v> Report<'v> {
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

    pub fn assess<'r>(&'r mut self, location: Location) -> Assess<'v, 'r> {
        match self {
            Report::Flag(f) => f.assess(location),
            Report::Basic(b) => b.assess(location),
            Report::Verbose(v) => v.assess(location),
        }
    }
}

impl<'v> criterion::Report<'v> for Report<'v> {
    type Output = Output;
    type Owned = Report<'static>;

    fn is_valid(&self) -> bool {
        match self {
            Report::Flag(f) => f.is_valid(),
            Report::Basic(b) => b.is_valid(),
            Report::Verbose(v) => v.is_valid(),
        }
    }

    fn into_owned(self) -> Self::Owned {
        match self {
            Report::Flag(f) => Report::Flag(f.into_owned()),
            Report::Basic(b) => Report::Basic(b.into_owned()),
            Report::Verbose(v) => Report::Verbose(v.into_owned()),
        }
    }

    fn new<'i, C, K>(output: Self::Output, schema: &Schema<'i, C, K>) -> Self
    where
        C: criterion::Criterion<K>,
        K: 'static + grill_core::Key,
    {
        todo!()
    }
}
impl<'de> Deserialize<'de> for Report<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Report<'static>, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl Serialize for Report<'_> {
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
    use serde_json::{Map, Value};

    use super::{Annotation, Assess, Error, Location};

    #[derive(Clone, Debug)]
    pub struct Flag<'v> {
        pub valid: bool,
        pub absolute_keyword_location: Option<AbsoluteUri>,
        pub additional_properties: Map<String, Value>,
        marker: std::marker::PhantomData<&'v ()>,
    }

    impl<'v> Flag<'v> {
        pub fn assess<'r>(&'r mut self, location: Location) -> Assess<'v, 'r> {
            Assess::Flag(self)
        }

        pub fn is_valid(&self) -> bool {
            self.valid
        }
        pub fn into_owned(self) -> Flag<'static> {
            Flag {
                valid: self.valid,
                additional_properties: self.additional_properties,
                absolute_keyword_location: self.absolute_keyword_location,
                marker: Default::default(),
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
    use super::{Annotation, Assess, Error, Location, Pointer};

    #[derive(Clone, Debug)]
    pub struct Basic<'v> {
        assessments: Vec<Assessment<'v>>,
    }
    impl<'v> Basic<'v> {
        pub fn assessments(&self) -> &[Assessment<'v>] {
            &self.assessments
        }
        pub fn assess<'r>(&mut self, location: Location) -> Assess<'v, 'r> {
            todo!()
        }
        pub fn is_valid(&self) -> bool {
            todo!()
        }
        pub fn into_owned(self) -> Basic<'static> {
            todo!()
        }

        /// Returns a reference to the first [`Assessment`] in the list, if any.
        pub fn first(&self) -> Option<&Assessment<'v>> {
            self.assessments.first()
        }

        /// Returns a mutable reference to the first [`Assessment`] in the list,
        /// if any.
        pub fn first_mut(&mut self) -> Option<&mut Assessment<'v>> {
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
        pub fn absolute_keyword_location(&self) -> Option<&grill_uri::AbsoluteUri> {
            self.first().map(|a| a.absolute_keyword_location())
        }
    }

    impl<'v> Basic<'v> {
        pub fn new(location: Location) -> Self {
            Self {
                assessments: vec![Assessment::Annotation {
                    annotation: None,
                    location: location,
                }],
            }
        }
    }
    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub enum Assessment<'v> {
        Annotation {
            annotation: Option<super::Annotation<'v>>,
            #[serde(flatten)]
            location: Location,
        },
        Error {
            error: Option<super::Error<'v>>,
            #[serde(flatten)]
            location: Location,
        },
    }
    impl<'v> Assessment<'v> {
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

        pub fn set_annotation(&mut self, annotation: Annotation<'v>) -> Option<Annotation<'v>> {
            let Self::Annotation { annotation: a, .. } = self else {
                return None;
            };
            a.replace(annotation)
        }

        pub fn take_annotation(&mut self) -> Option<Annotation<'v>> {
            let Self::Annotation { annotation: a, .. } = self else {
                return None;
            };
            a.take()
        }
        pub fn annotation(&self) -> Option<&Annotation<'v>> {
            let Self::Annotation { annotation: a, .. } = self else {
                return None;
            };
            a.as_ref()
        }
        pub fn set_error(&mut self, error: Error<'v>) -> Option<Error<'v>> {
            let Self::Error { error: a, .. } = self else {
                return None;
            };
            a.replace(error)
        }
        pub fn take_error(&mut self) -> Option<Error<'v>> {
            let Self::Error { error: a, .. } = self else {
                return None;
            };
            a.take()
        }
        pub fn error(&self) -> Option<&Error<'v>> {
            let Self::Error { error: a, .. } = self else {
                return None;
            };
            a.as_ref()
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
    use super::{AbsoluteUri, Annotation, Error};

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    pub struct Verbose<'v> {
        #[serde(flatten)]
        pub location: super::Location,
        #[serde(flatten)]
        pub detail: Assessment<'v>,
    }

    #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    pub enum Assessment<'v> {
        Annotation {
            annotations: Vec<Verbose<'v>>,
            annotation: Option<Annotation<'v>>,
        },
        Error {
            errors: Vec<Verbose<'v>>,
            error: Option<Error<'v>>,
        },
    }
    impl<'v> Verbose<'v> {
        pub fn is_valid(&self) -> bool {
            matches!(self.detail, Assessment::Annotation { .. })
        }
        pub fn into_owned(self) -> Verbose<'static> {
            todo!()
        }
        /// Sets the annotation of the current assessment and previous
        /// assessment, if it existed.
        ///
        /// If the current assessment is not `Asssessment::Annotation` then
        /// `annotation` will be ignored.
        pub fn set_annotation(&mut self, annotation: Annotation<'v>) -> Option<Annotation<'v>> {
            if let Assessment::Annotation { annotation: a, .. } = &mut self.detail {
                a.replace(annotation)
            } else {
                None
            }
        }
        pub fn set_error(&mut self, error: Error<'v>) -> Option<Error<'v>> {
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

        pub fn assess<'r>(&'r self, location: super::Location) -> super::Assess<'v, 'r> {
            todo!()
        }
    }
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

macro_rules! impl_from {
    ($($typ:ident),*) => {
        $(impl<'v> From<$typ<'v>> for Report<'v> {
            fn from(val: $typ<'v>) -> Self {
                Self::$typ(val)
            }
        })*
    };
}
macro_rules! impl_try_from {
    ($($typ:ident),*) => {
        $(impl<'v> TryFrom<Report<'v>> for $typ<'v> {
            type Error = Report<'v>;
            fn try_from(report: Report<'v>) -> Result<Self, Self::Error> {
                if let Report::$typ(v) = report {
                    Ok(v)
                } else {
                    Err(report)
                }
            }
        })*
    };
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

impl_from!(Flag, Basic, Verbose);
impl_try_from!(Flag, Basic, Verbose);
