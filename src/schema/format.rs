/// The format keyword allows for basic semantic identification of certain kinds
/// of string values that are commonly used. For example, because JSON doesn’t
/// have a `DateTime` type, dates need to be encoded as strings. format allows
/// the schema author to indicate that the string value should be interpreted as
/// a date. By default, format is just an annotation and does not effect
/// validation.
///
/// | `type`      | `format`                  | Comments                                                                                                                                                                                                                                                          | Core |
/// | :---------- | :------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | :--: |
/// | `"string"`  | `"date-time"`             | Date and time together, for example, `2018-11-13T20:20:39+00:00`                                                                                                                                                                                                  |  ✔️   |
/// | `"string"`  | `"time"`                  | Time, for example, `20:20:39+00:00`                                                                                                                                                                                                                               |  ✔️   |
/// | `"string"`  | `"date"`                  | Date, for example, `2018-11-13`                                                                                                                                                                                                                                   |  ✔️   |
/// | `"string"`  | `"duration"`              | Duration as defined by [ISO 8601 ABNF for “duration”](https://datatracker.ietf.org/doc/html/rfc3339#appendix-A), for example, `P3D` expresses a duration of 3 days.                                                                                               |  ✔️   |
/// | `"string"`  | `"email"`                 | Internet email address, see [RFC 5321, section 4.1.2](https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.2)                                                                                                                                                |  ✔️   |
/// | `"string"`  | `"idn-email"`             | The internationalized form of an Internet email address, see [RFC 6531](https://datatracker.ietf.org/doc/html/rfc6531)                                                                                                                                            |  ✔️   |
/// | `"string"`  | `"hostname"`              | Internet host name, see [RFC 1123, section 2.1](https://datatracker.ietf.org/doc/html/rfc1123#section-2.1)                                                                                                                                                        |  ✔️   |
/// | `"string"`  | `"idn-hostname"`          | An internationalized Internet host name, see [RFC5890, section 2.3.2.3](https://tools.ietf.org/html/rfc5890#section-2.3.2.3)                                                                                                                                      |  ✔️   |
/// | `"string"`  | `"ipv4"`                  | IPv4 address, according to dotted-quad ABNF syntax as defined in [RFC 2673, section 3.2](http://tools.ietf.org/html/rfc2673#section-3.2)                                                                                                                          |  ✔️   |
/// | `"string"`  | `"ipv6"`                  | IPv6 address, as defined in [RFC 2373, section 2.2](http://tools.ietf.org/html/rfc2373#section-2.2)                                                                                                                                                               |  ✔️   |
/// | `"string"`  | `"uuid"`                  | A Universally Unique Identifier as defined by [RFC 4122](https://datatracker.ietf.org/doc/html/rfc4122)                                                                                                                                                           |  ✔️   |
/// | `"string"`  | `"uri"`                   | A universal resource identifier (URI), according to [RFC 3986](http://tools.ietf.org/html/rfc3986)                                                                                                                                                                |  ✔️   |
/// | `"string"`  | `"uri-reference"`         | A URI Reference (either a URI or a relative-reference), according to [RFC 3986, section 4.1](http://tools.ietf.org/html/rfc3986#section-4.1)                                                                                                                      |  ✔️   |
/// | `"string"`  | `"iri"`                   | The internationalized equivalent of a “uri”, according to [RFC 3987](https://tools.ietf.org/html/rfc3987)                                                                                                                                                         |  ✔️   |
/// | `"string"`  | `"iri-reference"`         | The internationalized equivalent of a “uri-reference”, according to [RFC 3987](https://tools.ietf.org/html/rfc3987)                                                                                                                                               |  ✔️   |
/// | `"string"`  | `"uri-template"`          | A URI Template (of any level) according to [RFC 6570](https://tools.ietf.org/html/rfc6570). If you don’t already know what a URI Template is, you probably don’t need this value                                                                                  |  ✔️   |
/// | `"string"`  | `"json-pointer"`          | A JSON Pointer, according to [RFC 6901](https://tools.ietf.org/html/rfc6901). Note that this should be used only when the entire string contains only JSON Pointer content, e.g. /foo/bar. JSON Pointer URI fragments, e.g. #/foo/bar/ should use "uri-reference" |  ✔️   |
/// | `"string"`  | `"relative-json-pointer"` | A [relative JSON pointer](https://datatracker.ietf.org/doc/html/draft-handrews-relative-json-pointer-01)                                                                                                                                                          |  ✔️   |
/// | `"string"`  | `"regex"`                 | A regular expression, which should be valid according to the [ECMA 262 dialect](http://www.ecma-international.org/publications/standards/Ecma-262.htm)                                                                                                            |  ✔️   |
/// | `"string"`  | `"password"`              | A hint to UIs to obscure input                                                                                                                                                                                                                                    |      |
/// | `"integer"` | `"int32"`                 | Signed 32-bit integer                                                                                                                                                                                                                                             |      |
/// | `"integer"` | `"int64"`                 | Signed 64-bit integer                                                                                                                                                                                                                                             |      |
/// | `"number"`  | `"float"`                 | 32-bit precision [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) floating point                                                                                                                                                                                |      |
/// | `"number"`  | `"double"`                | 64-bit precision [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754) floating point                                                                                                                                                                                |      |
///
/// - [Draft 2020-12 # 7. Vocabularies for Semantic Content With "format" ](https://json-schema.org/draft/2020-12/json-schema-validation.html#name-vocabularies-for-semantic-c)
/// - [Understanding Json Schema # string Built-in Formats](https://json-schema.org/understanding-json-schema/reference/string.html#id7)

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    /// Date and time together, for example, `2018-11-13T20:20:39+00:00`
    DateTime,
    /// Time, for example, `20:20:39+00:00`
    Time,
    /// Date, for example, `2018-11-13`
    Date,
    /// Duration as defined by [ISO 8601 ABNF for
    /// “duration”](https://datatracker.ietf.org/doc/html/rfc3339#appendix-A),
    /// for example, `P3D` expresses a duration of 3 days.
    Duration,
    /// Internet email address, see [RFC 5321, section
    /// 4.1.2](https://datatracker.ietf.org/doc/html/rfc5321#section-4.1.2)
    Email,
    /// The internationalized form of an Internet email address, see [RFC
    /// 6531](https://datatracker.ietf.org/doc/html/rfc6531)
    IdnEmail,
    /// Internet host name, see [RFC 1123, section
    /// 2.1](https://datatracker.ietf.org/doc/html/rfc1123#section-2.1)
    Hostname,
    /// An internationalized Internet host name, see [RFC5890, section
    /// 2.3.2.3](https://tools.ietf.org/html/rfc5890#section-2.3.2.3)
    IdnHostname,
    /// IPv4 address, according to dotted-quad ABNF syntax as defined in [RFC
    /// 2673, section 3.2](http://tools.ietf.org/html/rfc2673#section-3.2)
    Ipv4,
    /// IPv6 address, as defined in [RFC 2373, section
    /// 2.2](http://tools.ietf.org/html/rfc2373#section-2.2)
    Ipv6,
    /// A Universally Unique Identifier as defined by [RFC
    /// 4122](https://datatracker.ietf.org/doc/html/rfc4122)
    Uuid,
    /// A universal resource identifier (URI), according to [RFC
    /// 3986](http://tools.ietf.org/html/rfc3986)
    Uri,
    /// A URI Reference (either a URI or a relative-reference), according to
    /// [RFC 3986, section 4.1](http://tools.ietf.org/html/rfc3986#section-4.1)
    UriReference,
    /// The internationalized equivalent of a “uri”, according to [RFC
    /// 3987](https://tools.ietf.org/html/rfc3987)
    Iri,
    /// The internationalized equivalent of a “uri-reference”, according to [RFC
    /// 3987](https://tools.ietf.org/html/rfc3987)
    IriReference,
    /// A URI Template (of any level) according to [RFC
    /// 6570](https://tools.ietf.org/html/rfc6570). If you don’t already know
    /// what a URI Template is, you probably don’t need this value
    UriTemplate,
    /// A JSON Pointer, according to [RFC
    /// 6901](https://tools.ietf.org/html/rfc6901). Note that this should be
    /// used only when the entire string contains only JSON Pointer content,
    /// e.g. /foo/bar. JSON Pointer URI fragments, e.g. #/foo/bar/ should use
    /// "uri-reference"
    JsonPointer,
    /// A [relative JSON
    /// pointer](https://datatracker.ietf.org/doc/html/draft-handrews-relative-json-pointer-01)
    RelativeJsonPointer,
    /// A regular expression, which should be valid according to the [ECMA 262
    /// dialect](http://www.ecma-international.org/publications/standards/Ecma-262.htm)
    Regex,
    /// A hint to UIs to obscure input
    Password,
    /// Signed 32-bit integer
    Int32,
    /// Signed 64-bit integer
    Int64,
    /// 32-bit precision [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
    /// floating point
    Float,
    /// 64-bit precision [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)
    /// floating point
    Double,

	/// A custom format
	Other(String)
}
