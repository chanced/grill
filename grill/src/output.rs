//! Output formats, annotations, and errors
//!
mod node;
mod structure;
mod traverse;
mod validation_error;

pub mod basic;
pub mod complete;
pub mod detailed;
pub mod flag;
pub mod verbose;

pub use basic::Basic;
pub use complete::Complete;
pub use detailed::Detailed;
pub use flag::Flag;
pub use node::Node;
pub use structure::Structure;
pub use validation_error::ValidationError;
pub use verbose::Verbose;

use crate::Uri;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Output<'v> {
    Flag(Flag),
    Basic(Basic<'v>),
    Detailed(Detailed<'v>),
    Verbose(Verbose<'v>),
    Complete(Complete<'v>),
}

impl<'v> Output<'v> {
    #[must_use]
    pub fn new(structure: Structure, node: Node<'v>) -> Output {
        match structure {
            Structure::Flag => Flag::new(node).into(),
            Structure::Basic => Basic::new(node).into(),
            Structure::Detailed => Detailed::new(node).into(),
            Structure::Verbose => Verbose::new(node).into(),
            Structure::Complete => Complete::new(node).into(),
        }
    }
    pub fn structure(&self) -> Structure {
        match self {
            Output::Flag(_) => Structure::Flag,
            Output::Basic(_) => Structure::Basic,
            Output::Detailed(_) => Structure::Detailed,
            Output::Verbose(_) => Structure::Verbose,
            Output::Complete(_) => Structure::Complete,
        }
    }
    pub fn absolute_keyword_location(&self) -> Option<&Uri> {
        match self {
            Output::Flag(flag) => None,
            Output::Basic(basic) => todo!(),
            Output::Detailed(detailed) => todo!(),
            Output::Verbose(verbose) => todo!(),
            Output::Complete(complete) => todo!(),
        }
    }
    #[must_use]
    pub fn is_valid(&self) -> bool {
        match self {
            Output::Flag(flag) => flag.is_valid(),
            Output::Basic(basic) => basic.is_valid(),
            Output::Detailed(detailed) => detailed.is_valid(),
            Output::Verbose(verbose) => verbose.is_valid(),
            Output::Complete(complete) => complete.is_valid(),
        }
    }
}

impl From<Flag> for Output<'_> {
    fn from(flag: Flag) -> Self {
        Self::Flag(flag)
    }
}
impl From<Basic<'_>> for Output<'_> {
    fn from(basic: Basic<'_>) -> Self {
        Self::Basic(basic)
    }
}

impl From<Detailed<'_>> for Output<'_> {
    fn from(detailed: Detailed<'_>) -> Self {
        Self::Detailed(detailed)
    }
}
impl From<Verbose<'_>> for Output<'_> {
    fn from(verbose: Verbose<'_>) -> Self {
        Self::Verbose(verbose)
    }
}

impl From<Complete<'_>> for Output<'_> {
    fn from(complete: Complete<'_>) -> Self {
        Self::Complete(complete)
    }
}

impl Display for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, collections::BTreeMap};

    use jsonptr::Pointer;

    use crate::Location;

    use super::*;

    // #[test]
    // fn test_annotiation_serde() {
    //     let mut additional_props = BTreeMap::new();
    //     additional_props.insert("example".into(), 34.into());

    //     let a = Annotation::Invalid(Node {
    //         additional_props,
    //         location: Location {
    //             keyword_location: "/".try_into().unwrap(),
    //             instance_location: "/".try_into().unwrap(),
    //             absolute_keyword_location: "http://example.com".to_string(),
    //         },
    //         error: None,
    //         valid: vec![Annotation::Valid(Node {
    //             valid: vec![],
    //             invalid: vec![],
    //             location: Location {
    //                 instance_location: Pointer::new(["bad-data"]),
    //                 keyword_location: Pointer::new(["error-keyword"]),
    //                 ..Default::default()
    //             },
    //             error: Some(Box::new(String::from("bad data"))),
    //             ..Default::default()
    //         })],
    //         invalid: vec![Annotation::Invalid(Node {
    //             valid: vec![],
    //             invalid: vec![],
    //             error: Some(Box::new(String::from("nested error"))),
    //             location: Location {
    //                 absolute_keyword_location: "http://example.com".to_string(),
    //                 ..Default::default()
    //             },

    //             ..Default::default()
    //         })],
    //     });

    //     let s = serde_json::to_string(&a).unwrap();
    //     let des_val: Annotation = serde_json::from_str(&s).unwrap();
    //     let des_str = serde_json::to_string(&des_val).unwrap();

    //     assert_eq!(s, des_str);
    // }
}
