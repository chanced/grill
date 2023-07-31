use std::str::FromStr;

use super::{RelativeUri, Segments, Uri};
use crate::error::UriError;
use percent_encoding::utf8_percent_encode;
use url::Url;
use urn::Urn;

pub(super) fn uri(value: &str) -> Result<Uri, UriError> {
    let mut parse = Parse::default();
    let mut iter = value.chars().enumerate().peekable();
    while let Some((i, c)) = iter.next() {
        let next = iter.peek().map(|(_, c)| *c);
        if !parse.next(i, c, next) {
            return parse.finalize(value);
        }
    }
    parse.finalize(value)
}

#[derive(Default)]
struct Parse {
    path_idx: usize,
    path_end_idx: usize,
    has_authority: bool,
    authority_end_idx: Option<usize>,
    query_idx: Option<usize>,
    fragment_idx: Option<usize>,
    maybe_fully_qualified: bool,
    maybe_urn: bool,
    done: bool,
}

impl Parse {
    fn finalize(self, value: &str) -> Result<Uri, UriError> {
        if self.maybe_urn {
            return Ok(Urn::from_str(value)?.into());
        }
        if !self.done || self.maybe_fully_qualified {
            return Ok(Url::parse(value)?.into());
        }

        let mut buf = String::with_capacity(value.len());
        self.finalize_path(value, &mut buf);
        todo!()
    }

    fn next(&mut self, index: usize, current: char, next: Option<char>) -> bool {
        match current {
            '/' => self.slash(index, next),
            '?' => self.question(index, next),
            '#' => self.bang(index, next),
            ':' => self.colon(index, next),
            _ => self.other(index, current, next),
        }
    }

    fn slash(&mut self, index: usize, next: Option<char>) -> bool {
        todo!()
    }

    fn question(&mut self, index: usize, next: Option<char>) -> bool {
        if index == 0 {
            self.maybe_fully_qualified = false;
        }
        if self.query_idx.is_none() {
            self.query_idx = Some(index);
        }
        if self.has_authority && self.authority_end_idx.is_none() {
            self.path_idx = index;
            self.authority_end_idx = Some(index);
        }
        true
    }

    fn bang(&mut self, index: usize, next: Option<char>) -> bool {
        todo!()
    }

    fn colon(&mut self, index: usize, next: Option<char>) -> bool {
        todo!()
    }

    fn other(&self, index: usize, current: char, next: Option<char>) -> bool {
        todo!()
    }

    fn finalize_authority(&self, value: &str, buf: &mut String) {
        if let Some(authority_end_idx) = self.authority_end_idx {
            buf.push_str(&value[..authority_end_idx]);
        }
    }

    fn finalize_path(&self, value: &str, buf: &mut String) {
        for segment in Segments::from_path(&value[self.path_idx..self.path_end_idx]) {
            buf.push_str(&segment.encode())
        }
    }

    fn write_encoded_query(&self, value: &str, buf: &mut String) {
        if let Some(query_idx) = self.query_idx {
            let last;
            if let Some(fragment_idx) = self.fragment_idx {
                last = fragment_idx;
            } else {
                last = value.len();
            }
            utf8_percent_encode(value[self.path_end_idx..last].trim(), &super::QUERY);
        }
    }
}
