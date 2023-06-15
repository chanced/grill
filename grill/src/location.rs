//! Location information for annotations and errors.

use core::fmt;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use jsonptr::Pointer;
use serde::{Deserialize, Serialize};

use crate::{AbsoluteUri, Uri};

impl<'de> Deserialize<'de> for AbsoluteKeywordLocation<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uri = Uri::deserialize(deserializer)?;
        let pointer = uri
            .fragment()
            .unwrap_or_default()
            .parse()
            .map_err(serde::de::Error::custom)?;
        Ok(AbsoluteKeywordLocation {
            uri: Cow::Owned(uri),
            pointer: Cow::Owned(pointer),
        })
    }
}
impl<'a> Serialize for AbsoluteKeywordLocation<'a> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str()));
        uri.serialize(serializer)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    /// The relative location of the validating keyword that follows the validation
    /// path.
    ///
    /// The value MUST be expressed as a JSON Pointer, and it MUST include any
    /// by-reference applicators such as "$ref" or "$dynamicRef".
    /// ```plaintext
    /// properties/width/$ref/minimum
    /// ```
    ///
    /// Note that this pointer may not be resolvable by the normal JSON Pointer
    /// process due to the inclusion of these by-reference applicator keywords.
    ///
    /// The JSON key for this information is "keywordLocation".
    ///
    /// - [JSON Schema Core 2020-12 #12.3.1. Keyword Relative Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-relative-location)
    #[serde(rename = "keywordLocation")]
    keyword_location: Pointer,

    /// The absolute, dereferenced location of the validating keyword.
    ///
    /// The value MUST be expressed as a full URI using the canonical URI of the
    /// relevant schema resource with a JSON Pointer fragment, and it MUST NOT
    /// include by-reference applicators such as `"$ref"` or `"$dynamicRef"` as
    /// non-terminal path components. It MAY end in such keywords if the error
    /// or annotation is for that keyword, such as an unresolvable reference.
    /// Note that "absolute" here is in the sense of "absolute filesystem path"
    /// (meaning the complete location) rather than the `"absolute-URI"
    /// terminology from RFC 3986 (meaning with scheme but without fragment).
    /// Keyword absolute locations will have a fragment in order to identify the
    /// keyword.
    ///
    /// # Example
    /// ```plaintext
    /// https://example.com/schemas/common#/$defs/count/minimum
    /// ```
    /// - [JSON Schema Core 2020-12 # 12.3.2. Keyword Absolute
    ///   Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-absolute-location)
    #[serde(rename = "absoluteKeywordLocation")]
    absolute_keyword_location: (Uri, Pointer),

    /// The location of the JSON value within the instance being validated. The
    /// value MUST be expressed as a JSON Pointer.
    ///
    /// - [JSON Schema Core 2020-12 # 12.3.3. Instance Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-instance-location)
    #[serde(rename = "instanceLocation")]
    instance_location: Pointer,
}

impl Location {
    /// The relative location of the validating keyword that follows the validation
    /// path. The value MUST be expressed as a JSON Pointer, and it MUST include any
    /// by-reference applicators such as "$ref" or "$dynamicRef".
    /// ```plaintext
    /// properties/width/$ref/minimum
    /// ```
    ///
    /// Note that this pointer may not be resolvable by the normal JSON Pointer
    /// process due to the inclusion of these by-reference applicator keywords.
    ///
    /// The JSON key for this information is "keywordLocation".
    ///
    /// - [JSON Schema Core 2020-12 #12.3.1. Keyword Relative Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-relative-location)
    #[must_use]
    pub fn keyword_location(&self) -> &Pointer {
        &self.keyword_location
    }
    #[must_use]
    pub fn absolute_keyword_location(&self) -> AbsoluteKeywordLocation<'_> {
        AbsoluteKeywordLocation {
            uri: Cow::Borrowed(&self.absolute_keyword_location.0),
            pointer: Cow::Borrowed(&self.absolute_keyword_location.1),
        }
    }
    #[must_use]
    pub fn instance_locaiton(&self) -> &Pointer {
        &self.instance_location
    }

    pub fn push_keyword_location(&mut self, keyword: &str) {
        let tok: jsonptr::Token = keyword.into();
        self.keyword_location.push_back(tok.clone());
        self.absolute_keyword_location.1.push_back(tok);
    }

    pub fn push_instance_location(&mut self, instance: &str) {
        let tok: jsonptr::Token = instance.into();
        self.instance_location.push_back(tok);
    }
    #[must_use]
    pub fn instance_location(&self) -> &Pointer {
        &self.instance_location
    }
}

pub trait Locate {
    fn location(&self) -> &Location;
    fn absolute_keyword_location(&self) -> AbsoluteKeywordLocation<'_> {
        self.location().absolute_keyword_location()
    }
    fn keyword_location(&self) -> &Pointer {
        self.location().keyword_location()
    }
    fn instance_location(&self) -> &Pointer {
        self.location().instance_location()
    }
}

/// The absolute, dereferenced location of the validating keyword.
///
/// The value MUST be expressed as a full URI using the canonical URI of the
/// relevant schema resource with a JSON Pointer fragment, and it MUST NOT
/// include by-reference applicators such as `"$ref"` or `"$dynamicRef"` as
/// non-terminal path components. It MAY end in such keywords if the error or
/// annotation is for that keyword, such as an unresolvable reference. Note that
/// "absolute" here is in the sense of "absolute filesystem path" (meaning the
/// complete location) rather than the `"absolute-URI"` terminology from RFC
/// 3986 (meaning with scheme but without fragment). Keyword absolute locations
/// will have a fragment in order to identify the keyword.
///
/// - [JSON Schema Core 2020-12 # 12.3.2. Keyword Absolute
///   Location](https://json-schema.org/draft/2020-12/json-schema-core.html#name-keyword-absolute-location)
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Hash)]
pub struct AbsoluteKeywordLocation<'a> {
    uri: Cow<'a, Uri>,
    pointer: Cow<'a, Pointer>,
}

impl Display for AbsoluteKeywordLocation<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str()));
        write!(f, "{uri}")
    }
}

impl PartialEq<Uri> for AbsoluteKeywordLocation<'_> {
    fn eq(&self, other: &Uri) -> bool {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str()));
        uri == *other
    }
}

impl PartialEq<AbsoluteUri> for AbsoluteKeywordLocation<'_> {
    fn eq(&self, other: &AbsoluteUri) -> bool {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str())).unwrap();
        let Ok(uri):Result<AbsoluteUri,_> = uri.try_into() else { return false};
        &uri == other
    }
}

impl PartialEq<String> for AbsoluteKeywordLocation<'_> {
    fn eq(&self, other: &String) -> bool {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str())).unwrap();
        uri.to_string() == *other
    }
}
impl PartialEq<str> for AbsoluteKeywordLocation<'_> {
    fn eq(&self, other: &str) -> bool {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str())).unwrap();
        uri.as_str() == other
    }
}

impl<'a> AbsoluteKeywordLocation<'a> {
    #[must_use]
    pub fn new(mut uri: Uri) -> Self {
        let pointer = uri
            .fragment()
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();

        uri.set_fragment(None);
        AbsoluteKeywordLocation {
            uri: Cow::Owned(uri),
            pointer: Cow::Owned(pointer),
        }
    }
    /// The base URI of the schema resource, excluding the fragment
    #[must_use]
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    /// Returns the fragment as a JSON Pointer
    #[must_use]
    pub fn pointer(&self) -> &Pointer {
        &self.pointer
    }
    #[must_use]
    pub fn borrow(&self) -> AbsoluteKeywordLocation<'_> {
        AbsoluteKeywordLocation {
            uri: Cow::Borrowed(&self.uri),
            pointer: Cow::Borrowed(&self.pointer),
        }
    }
    #[must_use]
    pub fn into_owned(self) -> AbsoluteKeywordLocation<'static> {
        AbsoluteKeywordLocation {
            uri: Cow::Owned(self.uri.into_owned()),
            pointer: Cow::Owned(self.pointer.into_owned()),
        }
    }
    #[must_use]
    pub fn into_uri(self) -> Uri {
        let mut uri = self.uri.into_owned();
        uri.set_fragment(Some(self.pointer.as_str()));
        uri
    }
    #[must_use]
    pub fn to_uri(&self) -> Uri {
        let mut uri = self.uri.clone().into_owned();
        uri.set_fragment(Some(self.pointer.as_str()));
        uri
    }
}
