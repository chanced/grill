use itertools::Itertools;
use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};

use crate::error::OverflowError;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const PATH: &AsciiSet = &FRAGMENT.add(b'#').add(b'?').add(b'{').add(b'}');
const PATH_SEGMENT: &AsciiSet = &PATH.add(b'#').add(b'?').add(b'{').add(b'}').add(b'\\');
const USERINFO: &AsciiSet = &PATH
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');

const QUERY: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'\'');

#[inline]
pub(super) fn path_segment(s: &str) -> String {
    utf8_percent_encode(s, PATH_SEGMENT).to_string()
}

#[inline]
pub(super) fn path(path: &str) -> String {
    let mut buf = String::with_capacity(path.len());
    path.split('/').map(path_segment).join("/")
}

#[inline]
pub(super) fn query(query: &str) -> String {
    utf8_percent_encode(query, &QUERY).to_string()
}

#[inline]
pub(super) fn fragment(fragment: &str) -> String {
    utf8_percent_encode(fragment, &FRAGMENT).to_string()
}
