use std::collections::HashMap;

use jsonptr::Pointer;
use once_cell::sync::OnceCell;

use crate::{schema::CompiledSubschema, Location};
/// Contains state and location information for a given keyword pertaining
/// to an evaluation.
pub struct Scope<'a> {
    location: Location,
    pub dynamic_anchors: &'a mut HashMap<String, OnceCell<CompiledSubschema>>,
}

impl<'a> Scope<'a> {
    pub fn new(
        location: Location,
        dynamic_anchors: &'a mut HashMap<String, OnceCell<CompiledSubschema>>,
    ) -> Self {
        Self {
            location,
            dynamic_anchors,
        }
    }

    /// Returns the location of the keyword.
    #[must_use]
    pub fn location(&self) -> &Location {
        &self.location
    }

    #[must_use]
    pub fn absolute_keyword_lcoation(&self) -> Option<&str> {
        self.location.absolute_keyword_location.as_deref()
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
        let absolute_keyword_location = match absolute_keyword_location {
            Some(absolute_keyword_location) => Some(absolute_keyword_location),
            None => {
                if let Some(v) = self.absolute_keyword_lcoation().map(String::from) {
                    let (uri, ptr) = v.split_once('#').unwrap_or((&v, ""));
                    let mut ptr: Pointer = ptr.try_into()?;
                    ptr.push_back(keyword.into());
                    Some(format!("{uri}#{ptr}"))
                } else {
                    None
                }
            }
        };
        let mut instance_location = self.instance_location().clone();
        instance_location.push_back(instance.into());
        Ok(Scope {
            location: Location {
                keyword_location,
                absolute_keyword_location,
                instance_location,
            },
            dynamic_anchors: self.dynamic_anchors,
        })
    }
}
