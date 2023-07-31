use crate::error::OverflowError;

use super::{encode, to_u32};

pub(super) fn authority(authority: Option<&str>, buf: &mut String) -> bool {
    if let Some(authority) = authority {
        buf.reserve(authority.len() + 2);
        buf.push_str("//");
        buf.push_str(authority);
        return true;
    }
    false
}

pub(super) fn path(path: &str, buf: &mut String) -> Result<u32, OverflowError> {
    let buf_len = buf.len();
    if path.is_empty() {
        if buf.is_empty() {
            return Ok(0);
        }
        return to_u32(buf_len - 1);
    }
    if buf_len > 0 {
        buf.push('/');
    }
    buf.push_str(path);
    to_u32(buf_len)
}

pub(super) fn query(
    query: Option<&str>,
    buf: &mut String,
    has_authority: bool,
    has_path: bool,
) -> Result<Option<u32>, OverflowError> {
    let Some(query) = query else { return Ok(None) };
    buf.reserve(query.len() + 1);
    let query = encode::query(query);
    if has_authority && has_path && !buf.is_empty() && !buf.ends_with('/') {
        buf.push('/');
    }
    let buf_len = buf.len();
    if !query.starts_with('?') {
        buf.push('?');
    }
    buf.push_str(&query);
    Ok(Some(to_u32(buf_len)?))
}

pub(super) fn fragment(
    fragment: Option<&str>,
    buf: &mut String,
) -> Result<Option<u32>, OverflowError> {
    let Some(fragment) = fragment else { return Ok(None) };
    let fragment = fragment.trim_start_matches('#');
    buf.reserve(fragment.len() + 1);
    let fragment = encode::fragment(fragment);
    let buf_len = buf.len();
    buf.push('#');
    buf.push_str(&fragment);
    Ok(Some(to_u32(buf_len)?))
}
