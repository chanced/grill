pub use self::{basic::Basic, flag::Flag, verbose::Verbose};
use grill_core::criterion;
use grill_uri::AbsoluteUri;
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

macro_rules! delegate {
    ($self:ident.$fn:ident($($param:ident),*);) => {
        match $self {
            Report::Flag(flag) => flag.$fn($($param),*),
            Report::Basic(basic) => basic.$fn($($param),*),
            Report::Verbose(verbose) => verbose.$fn($($param),*),
        }
    };
}

impl<'v> Report<'v> {
    pub fn push_annotation(&mut self, annotation: Annotation<'v>) {
        delegate! {
            self.push_annotation(annotation);
        }
    }
    pub fn push_error(&mut self, error: Error<'v>) {
        delegate! {
            self.push_error(error);
        }
    }
    pub fn instance_location(&self) -> &jsonptr::Pointer {
        delegate! {
            self.instance_location();
        }
    }
    pub fn keyword_location(&self) -> &jsonptr::Pointer {
        delegate! {
            self.keyword_location();
        }
    }
    pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
        delegate! {
            self.absolute_keyword_location();
        }
    }
}

impl<'v> criterion::Report<'v> for Report<'v> {
    type Output = Output;
    type Owned = Report<'static>;

    fn is_valid(&self) -> bool {
        delegate! {
            self.is_valid();
        }
    }

    fn into_owned(self) -> Self::Owned {
        todo!()
    }

    fn new(
        _output: Self::Output,
        _absolute_keyword_location: &AbsoluteUri,
        _keyword_location: jsonptr::Pointer,
        _instance_location: jsonptr::Pointer,
    ) -> Self {
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

    use super::{Annotation, Error};

    #[derive(Clone, Debug)]
    pub struct Flag<'v> {
        pub valid: bool,
        pub absolute_keyword_location: Option<AbsoluteUri>,
        pub additional_properties: Map<String, Value>,
        marker: std::marker::PhantomData<&'v ()>,
    }

    impl<'v> Flag<'v> {
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
        /// Noop
        pub fn push_annotation(&mut self, _annotation: Annotation<'v>) {}

        /// Sets `valid` to `false`
        pub fn push_error(&mut self, _error: Error<'v>) {
            self.valid = false
        }

        pub fn instance_location(&self) -> &jsonptr::Pointer {
            &jsonptr::Pointer::default()
        }
        pub fn keyword_location(&self) -> &jsonptr::Pointer {
            &jsonptr::Pointer::default()
        }
        pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
            None
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
    use std::marker::PhantomData;

    use grill_uri::AbsoluteUri;

    use crate::{Annotation, Error};

    #[derive(Clone, Debug)]
    pub struct Basic<'v> {
        marker: PhantomData<&'v str>,
    }
    impl<'v> Basic<'v> {
        pub fn is_valid(&self) -> bool {
            todo!()
        }
        pub fn into_owned(self) -> Basic<'static> {
            todo!()
        }
        pub fn push_annotation(&mut self, annotation: Annotation<'v>) {
            todo!()
        }
        pub fn push_error(&mut self, error: Error<'v>) {
            todo!()
        }
        pub fn instance_location(&self) -> &jsonptr::Pointer {
            todo!()
        }
        pub fn keyword_location(&self) -> &jsonptr::Pointer {
            todo!()
        }
        pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
            todo!()
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
    use grill_uri::AbsoluteUri;

    use super::{Annotation, Error};

    #[derive(Clone, Debug)]
    pub struct Verbose<'v> {
        pub detail: Assessment<'v>,
    }

    #[derive(Clone, Debug)]
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
        pub fn push_annotation(&mut self, annotation: Annotation<'v>) {
            if let Assessment::Annotation { annotations, .. } = &mut self.detail {
                annotations.push(Verbose {
                    detail: Assessment::Annotation {
                        annotations: Vec::new(),
                        annotation: Some(annotation),
                    },
                });
            }
        }
        /// Set the annotation of the current assessment. If the current assessment
        /// is not `Asssessment::Annotation` then `annotation` will be ignored.
        pub fn set_annotation(&mut self, annotation: Option<Annotation<'v>>) {
            if let Assessment::Annotation { annotation: a, .. } = &mut self.detail {
                *a = annotation;
            }
        }
        /// Push an error to the current assessment. If the current assessment
        /// is not an error, it will be converted to an error.
        pub fn push_error(&mut self, error: Error<'v>) {
            if let Assessment::Error { errors, .. } = &mut self.detail {
                errors.push(Verbose {
                    detail: Assessment::Error {
                        errors: Vec::new(),
                        error: Some(error),
                    },
                });
            } else {
                self.detail = Assessment::Error {
                    errors: Vec::new(),
                    error: Some(error),
                };
            }
        }
        /// Set the error of the current assessment. If the current assessment
        /// is not an error, it will be converted to an error
        pub fn set_error(&mut self, error: Option<Error<'v>>) {
            if let Assessment::Error { error: e, .. } = &mut self.detail {
                *e = error;
            } else {
                self.detail = Assessment::Error {
                    errors: Vec::new(),
                    error,
                };
            }
        }
        pub fn instance_location(&self) -> &jsonptr::Pointer {
            todo!()
        }
        pub fn keyword_location(&self) -> &jsonptr::Pointer {
            todo!()
        }
        pub fn absolute_keyword_location(&self) -> Option<&AbsoluteUri> {
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

#[derive(Clone, Debug)]
pub enum Annotation<'v> {
    Schema(crate::keyword::schema::Annotation),
    Unknown(Cow<'v, Value>),
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
