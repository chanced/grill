use num_rational::BigRational;
use serde_json::{Number, Value};
use slotmap::SlotMap;

use crate::{error::NumberError, output::Node, schema::Schemas, Key};

use super::{BigInts, BigRationals, State, Values};
/// Contains global and evaluation level state, schemas, and location
/// information needed to perform an
/// [`evaluation`](`crate::Interrogator::evaluate`).
pub struct Context<'i> {
    /// global state to the interrogator
    pub(crate) global_state: &'i mut State,
    /// per-evaluation state
    pub(crate) eval_state: &'i mut State,
    pub(crate) schemas: &'i Schemas,
    pub(crate) ints: &'i BigInts,
    pub(crate) rationals: &'i BigRationals,
    pub(crate) values: &'i Values,
}

impl<'s> Context<'s> {
    #[must_use]
    pub fn evalute<'v>(&self, key: Key) -> Node<'v> {
        todo!()
    }

    /// Returns a new, nested [`Scope`], where `instance` should be the name of
    /// field or index within the value being evaluated and `keyword` is the
    /// keyword being executed.
    ///
    /// # Errors
    /// Returns a [`jsonptr::Error`](`jsonptr::Error`) if the
    /// `absolute_keyword_location`'s pointer is malformed.
    pub fn nested(
        &mut self,
        _instance: &str,
        _keyword: &str,
        _absolute_keyword_location: Option<String>,
    ) -> Result<Context, jsonptr::MalformedPointerError> {
        // let mut keyword_location = self.keyword_location().clone();
        // keyword_location.push_back(keyword.into());
        // let absolute_keyword_location =
        //     if let Some(absolute_keyword_location) = absolute_keyword_location {
        //         absolute_keyword_location
        //     } else {
        //         let v = self.location.absolute_keyword_location.clone();
        //         let (uri, ptr) = v.split_once('#').unwrap_or((&v, ""));
        //         let mut ptr: Pointer = Pointer::try_from(ptr)?;
        //         ptr.push_back(keyword.into());
        //         format!("{uri}#{ptr}")
        //     };
        // let mut instance_location = self.instance_location().clone();
        // instance_location.push_back(instance.into());
        // Ok(Scope {
        //     location: Location {
        //         keyword_location,
        //         absolute_keyword_location,
        //         instance_location,
        //     },
        //     state: self.state,
        //     number: None,
        // })
        todo!()
    }
}

fn x(r: impl std::io::Read) -> Result<Option<String>, serde_json::Error> {
    serde_json::from_reader(r).map(Option::Some)
}
