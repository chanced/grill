use std::{borrow::Cow, fmt::Write};

use crate::error::OverflowError;

use super::to_u32;

#[allow(clippy::unnecessary_unwrap, clippy::unnecessary_wraps)]
pub(super) fn username<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    username: Option<T>,
) -> Result<Option<u32>, OverflowError> {
    let Some(authority) = username else { return Ok(None) };
    let authority = authority.into();
    if !buf.ends_with("//") && !authority.starts_with("//") {
        buf.push_str("//");
    }
    buf.push_str(&authority.to_lowercase());
    Ok(Some(2))
}

pub(super) fn password<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    password: Option<T>,
) -> Result<Option<u32>, OverflowError> {
    let Some(password) = password else { return Ok(None) };
    let password = password.into();
    _ = buf.trim_end_matches(':');
    if password.is_empty() {
        return Ok(None);
    }
    let buf_len = to_u32(buf.len())?;
    _ = buf.write_char(':');
    _ = buf.write_str(&password);
    Ok(Some(buf_len))
}

pub(super) fn host<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    host: Option<T>,
) -> Result<Option<u32>, OverflowError> {
    let Some(host) = host else { return Ok(None) };
    let host = host.into();
    _ = buf.trim_end_matches('@');
    if buf.is_empty() {
        buf.write_str("//").unwrap();
    }
    let buf_len = to_u32(buf.len())?;

    if buf.len() > 2 && !host.starts_with('@') {
        buf.write_char('@').unwrap();
    }

    buf.write_str(&host).unwrap();
    Ok(Some(buf_len))
}

pub(super) fn port<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    port: Option<T>,
) -> Result<Option<u32>, OverflowError> {
    let Some(port) = port else { return Ok(None) };
    let port = port.into();
    _ = buf.trim_end_matches(':');
    if port.is_empty() {
        return Ok(None);
    }
    let buf_len = to_u32(buf.len())?;
    buf.write_char(':').unwrap();
    buf.write_str(&port).unwrap();

    Ok(Some(buf_len))
}

pub(super) fn path<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    path: T,
) -> Result<u32, OverflowError> {
    let path = path.into();
    if path.is_empty() {
        if buf.is_empty() {
            return Ok(0);
        }
        return to_u32(buf.len());
    }
    let buf_len = buf.len();
    if buf_len > 0 && !path.starts_with('/') {
        buf.push('/');
    }
    buf.push_str(&path);
    to_u32(buf_len)
}

pub(super) fn query<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    query: Option<T>,
    has_authority: bool,
    has_path: bool,
) -> Result<Option<u32>, OverflowError> {
    let Some(query) = query else { return Ok(None) };
    let query = query.into();
    if has_authority && !has_path && !buf.is_empty() && !buf.ends_with('/') {
        buf.push('/');
    }
    let buf_len = buf.len();
    if !query.starts_with('?') {
        buf.push('?');
    }
    buf.push_str(&query);
    Ok(Some(to_u32(buf_len)?))
}

pub(super) fn fragment<'a, T: Into<Cow<'a, str>>>(
    buf: &mut String,
    fragment: Option<T>,
) -> Result<Option<u32>, OverflowError> {
    let Some(fragment) = fragment else { return Ok(None) };
    let fragment = fragment.into();
    _ = buf.trim_end_matches('#');
    let buf_len = buf.len();
    if !fragment.starts_with('#') {
        buf.push('#');
    }
    buf.write_str(&fragment).unwrap();
    Ok(Some(to_u32(buf_len)?))
}
