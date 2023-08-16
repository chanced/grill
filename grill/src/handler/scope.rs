use num_rational::BigRational;
use serde_json::{Number, Value};
use slotmap::SlotMap;

use crate::{error::NumberError, output::Node, schema::Location, SchemaKey};

use super::State;
/// Contains state and location information needed to perform an
/// [`evaluation`](`crate::Interrogator::evaluate`).
pub struct Scope<'s> {
    pub state: &'s mut State,
    location: Location,
    number: Option<BigRational>,
}

impl<'s> Scope<'s> {
    pub fn new(
        location: Location,
        state: &'s mut State,
        _schemas: SlotMap<SchemaKey, Value>,
    ) -> Self {
        Self {
            state,
            location,
            number: None,
        }
    }
    #[must_use]
    pub fn annotate<'v>(&self, keyword: &'static str, value: &'v Value) -> Node<'v> {
        let mut location = self.location.clone();
        location.push_keyword_location(keyword);
        Node::new(location, value)
    }

    /// # Errors
    /// Returns a [`ParseError`](`big_rational_str::ParseError`) if `number` cannot be parsed as a [`BigRational`].
    #[allow(clippy::missing_panics_doc)]
    pub fn number(&mut self, number: &Number) -> Result<&BigRational, NumberError> {
        let n = &mut self.number;
        if let Some(number) = n {
            Ok(number)
        } else {
            // let number = big_rational_str::str_to_big_rational(&number.to_string())?;
            // n.replace(number);
            // Ok(n.as_ref().unwrap())
            todo!()
        }
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
    ) -> Result<Scope, jsonptr::MalformedPointerError> {
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
