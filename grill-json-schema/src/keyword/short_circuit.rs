//! Determines whether or not an evaluation can be short-circuited

use bitflags::bitflags;
use grill_core::{
    keyword::{Keyword, Kind},
    Structure,
};

/// Determines whether or not an evaluation can be short-circuited
/// based on the target `Structure` and whether or not any of the
/// supplied keywords are present in the schema.
#[derive(Clone, Debug)]
pub struct ShortCircuit {
    /// `Structure`s which would enable short-circuiting
    pub enabling_structures: Structures,
    /// The set of keywords to check that disable short-circuiting
    pub disabling_keywords: Vec<&'static str>,
}

impl Keyword for ShortCircuit {
    fn kind(&self) -> Kind {
        Kind::Logic
    }

    fn compile<'i>(
        &mut self,
        compile: &mut grill_core::keyword::Compile<'i>,
        schema: grill_core::Schema<'i>,
    ) -> Result<bool, grill_core::error::CompileError> {
        todo!()
    }

    fn evaluate<'i, 'v>(
        &'i self,
        ctx: &'i mut grill_core::keyword::Context,
        value: &'v serde_json::Value,
    ) -> Result<Option<grill_core::output::Output<'v>>, grill_core::error::EvaluateError> {
        todo!()
    }
}

bitflags! {
    /// Represents a set of `Structure`s.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Structures: u8 {
        /// [`Structure::Flag`]
        const FLAG = Structure::Flag as u8;
        /// [`Structure::Basic`]
        const BASIC = Structure::Basic as u8;
        /// [`Structure::Detailed`]
        const DETAILED = Structure::Detailed as u8;
        /// [`Structure::Verbose`]
        const VERBOSE = Structure::Verbose as u8;
    }
}
