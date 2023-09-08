use std::{borrow::Cow, str::Split};

use crate::{big::usize_to_u32, error::OverflowError};

/// A single query parameter key value pair.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct QueryParameter<'a> {
    full: Cow<'a, str>,
    eq_index: Option<u32>,
}
impl<'a> QueryParameter<'a> {
    pub fn new(full: &'a str) -> Result<Self, OverflowError<usize, { u32::MAX as u64 }>> {
        usize_to_u32(full.len())?;
        let eq_index = full.find('=').map(|i| i.try_into().unwrap());
        let full = full.into();
        Ok(Self { full, eq_index })
    }

    /// Converts this `QueryParameter` into an owned version.
    #[must_use]
    pub fn into_owned(self) -> QueryParameter<'static> {
        QueryParameter {
            full: self.full.into_owned().into(),
            eq_index: self.eq_index,
        }
    }

    /// Returns the full query parameter string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.full.as_ref()
    }
    /// Returns the key, i.e. anything to the left of `'='`, of the query
    /// parameter.
    #[must_use]
    pub fn key(&self) -> &str {
        self.full[..self.eq_index().unwrap_or(self.full.len())].as_ref()
    }
    /// Returns the value, i.e. anything to the right of `'='`, of the query
    /// parameter, if it exists.
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.eq_index().map(|i| &self.full[i + 1..])
    }

    fn eq_index(&self) -> Option<usize> {
        self.eq_index.map(|i| i as usize)
    }
}

impl<'a> TryFrom<&'a str> for QueryParameter<'a> {
    type Error = OverflowError<usize, { u32::MAX as u64 }>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        Self::new(s)
    }
}

#[derive(Debug, Default)]
pub struct QueryParameters<'a> {
    query: Option<Split<'a, char>>,
}
impl<'a> QueryParameters<'a> {
    pub fn new(
        query: Option<&'a str>,
    ) -> Result<Self, OverflowError<usize, { usize::MAX as u64 }>> {
        let Some(query) = query else { return Ok(Self { query: None }) };
        if query.len() > u32::MAX as usize {
            return Err(OverflowError(query.len()));
        }
        Ok(Self {
            query: Some(query.split('&')),
        })
    }
}

impl<'a> Iterator for QueryParameters<'a> {
    type Item = QueryParameter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.query
            .as_mut()
            .and_then(Iterator::next)
            .filter(|qp| !qp.is_empty())
            .map(QueryParameter::new)
            .map(Result::unwrap)
    }
}
