use jsonptr::Pointer;
use num_rational::BigRational;
use serde_json::{Number, Value};

use crate::{output::Annotation, Location, State};
/// Contains state and location information for a given keyword pertaining
/// to an evaluation.
pub struct Scope<'a> {
    pub state: &'a mut State,
    location: Location,
    number: Option<BigRational>,
}

impl<'a> Scope<'a> {
    pub fn new(location: Location, state: &'a mut State) -> Self {
        Self {
            state,
            location,
            number: None,
        }
    }
    #[must_use]
    pub fn annotate<'v>(&self, keyword: &'static str, value: &'v Value) -> Annotation<'v> {
        let mut location = self.location.clone();
        location.push_keyword_location(keyword);
        Annotation::new(location, value)
    }

    /// Returns the location of the keyword.
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn number(&mut self, number: &Number) -> &BigRational {
        self.number.get_or_insert_with(|| {
            big_rational_str::str_to_big_rational(&number.to_string())
                .expect("serde_json::Number should always parse into a num_rational::BigRational")
        })
    }

    #[must_use]
    pub fn absolute_keyword_lcoation(&self) -> &str {
        &self.location.absolute_keyword_location
    }
    #[must_use]
    pub fn keyword_location(&self) -> &Pointer {
        &self.location.keyword_location
    }
    #[must_use]
    pub fn instance_location(&self) -> &Pointer {
        &self.location.instance_location
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
        instance: &str,
        keyword: &str,
        absolute_keyword_location: Option<String>,
    ) -> Result<Scope, jsonptr::Error> {
        let mut keyword_location = self.keyword_location().clone();
        keyword_location.push_back(keyword.into());
        let absolute_keyword_location =
            if let Some(absolute_keyword_location) = absolute_keyword_location {
                absolute_keyword_location
            } else {
                let v = self.location.absolute_keyword_location.clone();
                let (uri, ptr) = v.split_once('#').unwrap_or((&v, ""));
                let mut ptr: Pointer = ptr.try_into()?;
                ptr.push_back(keyword.into());
                format!("{uri}#{ptr}")
            };
        let mut instance_location = self.instance_location().clone();
        instance_location.push_back(instance.into());
        Ok(Scope {
            location: Location {
                keyword_location,
                absolute_keyword_location,
                instance_location,
            },
            state: self.state,
            number: None,
        })
    }
}
