use super::{Detail, ERROR_MSG, SUCCESS_MSG};
use crate::Uri;
use jsonptr::Pointer;
use serde::{
    de::{self, Unexpected},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;
use std::{
    borrow::Cow,
    collections::{BTreeMap, VecDeque},
    fmt::{self},
    sync::Arc,
};
const EXPECTED_FMT: &str = "a string equal to \"flag\", \"basic\", \"detailed\", or \"verbose\"";
const INSTANCE_LOCATION: &str = "instanceLocation";
const ABSOLUTE_KEYWORD_LOCATION: &str = "absoluteKeywordLocation";
const KEYWORD_LOCATION: &str = "keywordLocation";
const ANNOTATIONS: &str = "annotations";
const ANNOTATION: &str = "annotation";
const ERROR: &str = "error";
const ERRORS: &str = "errors";
const VALID: &str = "valid";
const FMT: &str = "fmt";
const FLAG: &str = "flag";
const BASIC: &str = "basic";
const DETAILED: &str = "detailed";
const VERBOSE: &str = "verbose";

const KEYS: [&str; 7] = [
    ABSOLUTE_KEYWORD_LOCATION,
    ANNOTATIONS,
    ERROR,
    ERRORS,
    INSTANCE_LOCATION,
    KEYWORD_LOCATION,
    VALID,
];

#[derive(Debug, Clone)]
pub enum Output<'v> {
    Flag(Flag<'v>),
    Basic(Basic<'v>),
    Detailed(Detailed<'v>),
    Verbose(Verbose<'v>),
}

impl<'de> Deserialize<'de> for Output<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;
        const EXPECTED: &str = "a JSON Schema Output object";
        let obj = match v {
            Value::Object(obj) => Ok(obj),
            Value::Null => Err(de::Error::invalid_value(Unexpected::Option, &EXPECTED)),
            Value::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(b), &EXPECTED)),
            Value::Number(_) => Err(de::Error::invalid_type(
                Unexpected::Other("number"),
                &EXPECTED,
            )),
            Value::String(s) => Err(de::Error::invalid_type(Unexpected::Str(&s), &EXPECTED)),
            Value::Array(_) => Err(de::Error::invalid_type(Unexpected::Seq, &EXPECTED)),
        }?;
        let fmt = determine_fmt(&v)?;

        match fmt {
            FLAG => Ok(Output::Flag(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            BASIC => Ok(Output::Basic(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            DETAILED => Ok(Output::Detailed(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            VERBOSE => Ok(Output::Verbose(
                serde_json::from_value(Value::Object(obj)).map_err(de::Error::custom)?,
            )),
            _ => unreachable!(),
        }
    }
}

fn deserialize_detailed<'de, D>(
    obj: serde_json::Map<String, Value>,
) -> Result<Output<'static>, <D as Deserializer<'de>>::Error>
where
    D: serde::Deserializer<'de>,
{
    serde_json::from_value(Value::Object(obj))
        .map(Output::Detailed)
        .map_err(de::Error::custom)
}

fn deserialize_flag<'de, D>(
    obj: serde_json::Map<String, Value>,
) -> Result<Output<'static>, <D as Deserializer<'de>>::Error>
where
    D: serde::Deserializer<'de>,
{
    serde_json::from_value(Value::Object(obj))
        .map(Output::Flag)
        .map_err(de::Error::custom)
}

fn deserialize_verbose<'de, D>(
    obj: serde_json::Map<String, Value>,
) -> Result<Output<'static>, <D as Deserializer<'de>>::Error>
where
    D: serde::Deserializer<'de>,
{
    serde_json::from_value(Value::Object(obj))
        .map(Output::Verbose)
        .map_err(de::Error::custom)
}

fn determine_fmt<E: de::Error>(obj: &Value) -> Result<&'_ str, E> {
    let fmt = obj.get(FMT);
    if let Some(fmt) = fmt {
        return fmt_from_str(fmt);
    }
    if is_hierarchical(obj) {
        return hierarchical_fmt(obj);
    }
    if has_nodes(obj) {
        return Ok(BASIC);
    }
    return Ok(FLAG);
}

fn hierarchical_fmt<E: de::Error>(obj: &Value) -> Result<&'_ str, E> {
    if contains_mixed(obj) {
        return Ok(VERBOSE);
    }
    Ok(DETAILED)
}

fn get_fmt(s: &str) -> Result<&'_ str, &'_ str> {
    match s {
        FLAG => Ok(FLAG),
        BASIC => Ok(BASIC),
        DETAILED => Ok(DETAILED),
        VERBOSE => Ok(VERBOSE),
        _ => Err(s),
    }
}

fn fmt_from_str<E>(v: &Value) -> Result<&'_ str, E>
where
    E: de::Error,
{
    let fmt = fmt_as_str(v)?;
    match fmt {
        FLAG | BASIC | DETAILED | VERBOSE => return Ok(fmt),
        _ => {
            return Err(de::Error::invalid_value(
                Unexpected::Str(fmt),
                &EXPECTED_FMT,
            ))
        }
    }
}

fn fmt_as_str<E>(v: &Value) -> Result<&'_ str, E>
where
    E: de::Error,
{
    match v {
        Value::String(s) => Ok(s),
        Value::Null => Err(de::Error::invalid_value(Unexpected::Option, &EXPECTED_FMT)),
        Value::Bool(b) => Err(de::Error::invalid_type(Unexpected::Bool(*b), &EXPECTED_FMT)),
        Value::Number(n) => Err(de::Error::invalid_type(
            Unexpected::Other(&format!("number {}", n)),
            &EXPECTED_FMT,
        )),
        Value::Array(_) => Err(de::Error::invalid_type(Unexpected::Seq, &EXPECTED_FMT)),
        Value::Object(_) => Err(de::Error::invalid_type(Unexpected::Map, &EXPECTED_FMT)),
    }
}

fn has_nodes(obj: &Value) -> bool {
    get_nodes(obj).is_some()
}
fn get_nodes(obj: &Value) -> Option<&Vec<Value>> {
    obj.get(ERRORS)
        .or_else(|| obj.get(ANNOTATIONS))
        .and_then(Value::as_array)
}

fn is_hierarchical(obj: &Value) -> bool {
    let Some(nodes) = get_nodes(obj) else {
        return false;
    };
    nodes
        .iter()
        .any(|v| v.get(ERRORS).or_else(|| v.get(ANNOTATIONS)).is_some())
}

fn contains_mixed(obj: &Value) -> bool {
    let mut has_errors = false;
    let mut has_annotations = false;
    let mut queue = VecDeque::new();
    let Some(obj) = obj.as_object() else {
        return false;
    };
    queue.push_back(obj);
    while !queue.is_empty() {
        let obj = queue.pop_front().unwrap();
        if obj.contains_key(ERROR) || obj.contains_key(ERRORS) {
            has_errors = true;
        }
        if obj.contains_key(ANNOTATION) || obj.contains_key(ANNOTATIONS) {
            has_annotations = true;
        }
        if has_annotations && has_errors {
            return true;
        }
        if let Some(errors) = obj.get(ERRORS).and_then(Value::as_array) {
            queue.extend(
                errors
                    .iter()
                    .map(|v| v.as_object())
                    .filter(Option::is_some)
                    .map(Option::unwrap),
            );
            continue;
        }
        if let Some(annotations) = obj.get(ANNOTATIONS).and_then(Value::as_array) {
            queue.extend(
                annotations
                    .iter()
                    .map(|v| v.as_object())
                    .filter(Option::is_some)
                    .map(Option::unwrap),
            );
            continue;
        }
    }
    false
}

impl Serialize for Output<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // if len is > 0, it has no impact on serialization
        let mut s = serializer.serialize_struct("Output", 3)?;
        match self {
            Output::Flag(f) => f.serialize(serializer),
            Output::Basic(b) => b.serialize(serializer),
            Output::Detailed(d) => d.serialize(serializer),
            Output::Verbose(v) => v.serialize(serializer),
        }?;
        s.end()
    }
}

impl<'v> Output<'v> {
    pub fn error(&self) -> Option<&dyn Detail<'v>> {
        match self {
            Output::Flag(o) => o.error(),
            Output::Basic(o) => o.error(),
            Output::Detailed(o) => o.error(),
            Output::Verbose(o) => o.error(),
        }
    }
}
impl fmt::Display for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Flag(v) => v.fmt(f),
            Output::Basic(v) => v.fmt(f),
            Output::Detailed(v) => v.fmt(f),
            Output::Verbose(v) => v.fmt(f),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicNode<'v> {
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub absolute_keyword_location: Uri,
    pub error: Option<Box<dyn Detail<'v>>>,
}

#[derive(Clone, Debug, Deserialize)]

pub struct Basic<'v> {
    pub valid: bool,
    #[serde(alias = "errors", alias = "annotations")]
    pub nodes: Vec<BasicNode<'v>>,
    #[serde(flatten)]
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,

    #[serde(skip)]
    transient: bool,
}

impl Serialize for Basic<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Basic", 3)?;
        s.serialize_field(FMT, BASIC);
        s.serialize_field(VALID, &self.valid)?;
        serialize_nodes(&mut s, &self.nodes, self.valid)?;
        serialize_additional_props(&mut s, self.additional_props.iter())?;
        s.end()
    }
}

impl<'v> Basic<'v> {
    pub fn error(&self) -> Option<&dyn Detail> {
        if self.valid {
            Some(&ERROR_MSG)
        } else {
            None
        }
    }
}

impl fmt::Display for Basic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.valid {
            write!(f, "{SUCCESS_MSG}")
        } else {
            write!(f, "{ERROR_MSG}")
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Detailed<'v> {
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub absolute_keyword_location: Option<Uri>,
    #[serde(rename = "valid")]
    pub is_valid: bool,
    pub error: Option<Arc<dyn Detail<'v>>>,
    #[serde(alias = "errors", alias = "annotations")]
    pub nodes: Vec<Detailed<'v>>,
    #[serde(flatten)]
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
    /// Indicates that this node is not part of the final output and is only
    /// used to store intermediate results.
    ///
    /// This is primarily for `if` / `then` / `else` branches but may be relevant
    /// for future or external keywords.
    #[serde(skip)]
    pub is_transient: bool,
}
impl<'v> Detailed<'v> {
    pub fn error(&self) -> Option<&dyn Detail> {
        self.error.as_deref()
    }

    pub fn append(&mut self, other: Detailed<'v>) {
        if other.is_valid {}
        if other.is_transient {
            self.nodes.extend(other.nodes);
            return;
        }
        self.nodes.push(other);
    }
}

impl Serialize for Detailed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // if len is > 0, it has no impact on serialization
        let mut s = serializer.serialize_struct("Detailed", 3)?;
        s.serialize_field(VALID, &self.is_valid)?;
        s.serialize_field(FMT, DETAILED);
        s.serialize_field(INSTANCE_LOCATION, &self.instance_location)?;
        s.serialize_field(KEYWORD_LOCATION, &self.keyword_location)?;
        serialize_option(
            &mut s,
            ABSOLUTE_KEYWORD_LOCATION,
            self.absolute_keyword_location.as_ref(),
        )?;
        serialize_nodes(&mut s, &self.nodes, self.is_valid)?;
        serialize_additional_props(&mut s, self.additional_props.iter())?;

        s.end()
    }
}
impl fmt::Display for Detailed<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_valid {
            write!(f, "{SUCCESS_MSG}")
        } else {
            write!(f, "{ERROR_MSG}")
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verbose<'v> {
    pub instance_location: Pointer,
    pub keyword_location: Pointer,
    pub absolute_keyword_location: Option<Uri>,
    pub error: Option<Box<dyn Detail<'v>>>,
    #[serde(default, alias = "errors", alias = "annotations")]
    pub nodes: Vec<Verbose<'v>>,
    pub valid: bool,
    #[serde(flatten)]
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
}
impl<'v> Verbose<'v> {
    pub fn error(&self) -> Option<&dyn Detail> {
        self.error.as_deref()
    }
}
impl<'v> fmt::Display for Verbose<'v> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(err) = self.error {
            write!(f, "{err}")
        } else {
            write!(f, "validation passed for {}", self.instance_location)
        }
    }
}

impl Serialize for Verbose<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Verbose", 3)?;
        s.serialize_field(FMT, VERBOSE)?;
        s.serialize_field(VALID, &self.valid)?;
        s.serialize_field(INSTANCE_LOCATION, &self.instance_location)?;
        s.serialize_field(KEYWORD_LOCATION, &self.keyword_location)?;
        serialize_option(
            &mut s,
            ABSOLUTE_KEYWORD_LOCATION,
            self.absolute_keyword_location.as_ref(),
        )?;
        serialize_nodes(&mut s, &self.nodes, self.valid)?;
        serialize_additional_props(&mut s, self.additional_props.iter())?;
        s.end()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flag<'v> {
    pub valid: bool,
    #[serde(flatten)]
    pub additional_props: BTreeMap<String, Cow<'v, Value>>,
}

impl Flag<'_> {
    pub fn error(&self) -> Option<&dyn Detail> {
        if self.valid {
            None
        } else {
            Some(&ERROR_MSG)
        }
    }
}

impl Serialize for Flag<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Flag", 3)?;
        s.serialize_field(FMT, FLAG);
        s.serialize_field(VALID, &self.valid)?;
        serialize_additional_props(&mut s, self.additional_props.iter())?;
        s.end()
    }
}

impl fmt::Display for Flag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.valid {
            write!(f, "validation passed")
        } else {
            write!(f, "{ERROR_MSG}")
        }
    }
}

fn serialize_additional_props<'a, S: SerializeStruct>(
    s: &mut S,
    additional_props: impl Iterator<Item = (&'a String, &'a Cow<'a, Value>)>,
) -> Result<(), S::Error> {
    let mut additional_props = additional_props
        .map(|(key, value)| (key.as_str(), value))
        .filter(|(key, _)| !KEYS.binary_search(key).is_ok());

    for (key, value) in additional_props {
        s.serialize_field(key, value)?;
    }

    Ok(())
}

fn serialize_option<S: SerializeStruct>(
    s: &mut S,
    key: &str,
    value: Option<&impl Serialize>,
) -> Result<(), S::Error> {
    if let Some(value) = value {
        s.serialize_field(key, value)?;
    }
    Ok(())
}
fn serialize_nodes<S: SerializeStruct>(
    s: &mut S,
    value: &[impl Serialize],
    valid: bool,
) -> Result<(), S::Error> {
    if value.is_empty() {
        return Ok(());
    }
    let key = if valid { "annotations" } else { "errors" };
    s.serialize_field(key, value)
}
