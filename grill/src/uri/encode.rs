use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::borrow::Cow;

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
pub(super) fn username(username: Option<&str>) -> Option<Cow<'static, str>> {
    Some(
        utf8_percent_encode(username?.trim_end_matches(':'), USERINFO)
            .to_string()
            .into(),
    )
}

#[inline]
pub(super) fn password(password: Option<&str>) -> Option<Cow<'static, str>> {
    Some(
        utf8_percent_encode(password?.trim_end_matches('@'), USERINFO)
            .to_string()
            .into(),
    )
}

#[inline]
pub(super) fn host(host: Option<&str>) -> Option<Cow<'static, str>> {
    Some(host?.to_lowercase().into())
}

#[inline]
pub(super) fn path(path: &str) -> String {
    path.split('/').map(path_segment).join("/")
}

#[inline]
pub(super) fn query(query: Option<&str>) -> Option<Cow<'static, str>> {
    Some(
        format!(
            "?{}",
            utf8_percent_encode(query?.trim_start_matches('?'), QUERY)
        )
        .into(),
    )
}

#[inline]
pub(super) fn fragment(fragment: Option<&str>) -> Option<Cow<'static, str>> {
    Some(
        format!(
            "#{}",
            utf8_percent_encode(fragment?.trim_start_matches('#'), FRAGMENT)
        )
        .into(),
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_encode_authority() {
        let s = "user:pass@host:1234";
        let userinfo_idx = s.find('@');
    }
}
