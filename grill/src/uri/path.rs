//! Path utilities for URIs.

use std::{borrow::Cow, path::PathBuf};

use super::{PathSegment, PathSegments};

/// Normalizes a path by removing dot segments, i.e. `'.'`, `'..'`.
#[must_use]
pub fn normalize(path: &str) -> Cow<'_, str> {
    let mut normalized = false;
    let mut buf = PathBuf::new();
    for segment in PathSegments::new(path) {
        match segment {
            PathSegment::Parent => {
                normalized = true;
                buf.pop();
            }
            PathSegment::Current => normalized = true,
            PathSegment::Normal(seg) => buf.push(seg.as_ref()),
            PathSegment::Root => {}
        }
    }
    if normalized {
        // safety: path is already in utf8
        buf.to_str().unwrap().to_string().into()
    } else {
        path.into()
    }
}

/// Merges two paths. This is essentially the same as [`PathBuf::push`], but
/// operates UTF-8 strings.
///
/// Note: this does not normalize the paths. See [`resolve`] or [`normalize`] for dot removal.
///
/// # Example
/// ```
/// use grill::uri::path::merge;
/// assert_eq!(merge("/path/to", "file"), "/path/to/file");
/// ```
pub fn merge(base: &str, path: &str) -> String {
    let mut buf = PathBuf::from(base);
    buf.push(path);
    // safety: path is already in utf8
    buf.to_str().unwrap().to_string()
}

/// Normalizes and merges `base` with `path`.
/// # Example
/// ```
/// use grill::uri::path::resolve;
/// assert_eq!(resolve("/path/to/other", "../file"), "/path/to/file");
/// ```
pub fn resolve(base: &str, path: &str) -> String {
    let buf = merge(base, path);
    crate::uri::path::normalize(&buf).into_owned()
}
