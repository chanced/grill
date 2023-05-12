use std::{collections::HashSet, fmt::Display, str::FromStr};

use heck::ToLowerCamelCase;
use serde::{Deserialize, Serialize};

const ARRAY: &str = "array";
const BOOLEAN: &str = "boolean";
const INTEGER: &str = "integer";
const NULL: &str = "null";
const OBJECT: &str = "object";
const STRING: &str = "string";

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(from = "&str", into = "String")]
pub enum Type {
    Array,
    Boolean,
    Integer,
    Null,
    Object,
    String,
    Other(String),
}

impl From<Type> for String {
    fn from(t: Type) -> Self {
        t.to_string()
    }
}

impl AsRef<str> for Type {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq<str> for Type {
    fn eq(&self, other: &str) -> bool {
        match self {
            Type::Array => other == ARRAY,
            Type::Boolean => other == BOOLEAN,
            Type::Integer => other == INTEGER,
            Type::Null => other == NULL,
            Type::Object => other == OBJECT,
            Type::String => other == STRING,
            Type::Other(s) => other
                .chars()
                .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
                .flat_map(|c| c.to_lowercase())
                .zip(
                    s.chars()
                        .filter(|c| !c.is_whitespace() && *c != '-' && *c != '_')
                        .flat_map(|c| c.to_lowercase()),
                )
                .all(|(v1, v2)| v1 == v2),
        }
    }
}
impl PartialEq<String> for Type {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Type {
    pub fn as_str(&self) -> &str {
        match self {
            Type::Array => ARRAY,
            Type::Boolean => BOOLEAN,
            Type::Integer => INTEGER,
            Type::Null => NULL,
            Type::Object => OBJECT,
            Type::String => STRING,
            Type::Other(s) => s.as_str(),
        }
    }

    /// Returns `true` if the type is [`Array`].
    ///
    /// [`Array`]: Type::Array
    #[must_use]
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array)
    }

    /// Returns `true` if the type is [`Boolean`].
    ///
    /// [`Boolean`]: Type::Boolean
    #[must_use]
    pub fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean)
    }

    /// Returns `true` if the type is [`Integer`].
    ///
    /// [`Integer`]: Type::Integer
    #[must_use]
    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer)
    }

    /// Returns `true` if the type is [`Null`].
    ///
    /// [`Null`]: Type::Null
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if the type is [`Object`].
    ///
    /// [`Object`]: Type::Object
    #[must_use]
    pub fn is_object(&self) -> bool {
        matches!(self, Self::Object)
    }

    /// Returns `true` if the type is [`String`].
    ///
    /// [`String`]: Type::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String)
    }

    /// Returns `true` if the type is [`Other`].
    ///
    /// [`Other`]: Type::Other
    #[must_use]
    pub fn is_other(&self) -> bool {
        matches!(self, Self::Other(..))
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Array => write!(f, "{ARRAY}"),
            Type::Boolean => write!(f, "{BOOLEAN}"),
            Type::Integer => write!(f, "{INTEGER}"),
            Type::Null => write!(f, "{NULL}"),
            Type::Object => write!(f, "{OBJECT}"),
            Type::String => write!(f, "{STRING}"),
            Type::Other(other) => write!(f, "{other}"),
        }
    }
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

impl From<&String> for Type {
    fn from(value: &String) -> Self {
        Self::from(value.as_str())
    }
}
impl From<&str> for Type {
    fn from(s: &str) -> Self {
        match s {
            ARRAY => Type::Array,
            BOOLEAN => Type::Boolean,
            INTEGER => Type::Integer,
            NULL => Type::Null,
            OBJECT => Type::Object,
            STRING => Type::String,
            _ => {
                let t = s.to_lower_camel_case();
                if t == s {
                    Type::Other(t)
                } else {
                    match t.as_str() {
                        ARRAY => Type::Array,
                        BOOLEAN => Type::Boolean,
                        INTEGER => Type::Integer,
                        NULL => Type::Null,
                        OBJECT => Type::Object,
                        STRING => Type::String,
                        _ => Type::Other(t),
                    }
                }
            }
        }
    }
}

impl FromStr for Type {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}
impl From<Type> for Types {
    fn from(t: Type) -> Self {
        Types::Single(t)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Types {
    Single(Type),
    Multiple(HashSet<Type>),
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(s), Self::Single(o)) => s == o,
            (Self::Multiple(s), Self::Multiple(o)) => s.iter().all(|i| o.contains(i)),
            _ => false,
        }
    }
}
impl Eq for Types {}

impl Types {
    /// Returns `true` if the types is [`Single`].
    ///
    /// [`Single`]: Types::Single
    #[must_use]
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(..))
    }

    /// Returns `true` if the types is [`Multiple`].
    ///
    /// [`Multiple`]: Types::Multiple
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        matches!(self, Self::Multiple(..))
    }
    pub fn contains(&self, value: &Type) -> bool {
        match self {
            Types::Single(s) => s == value,
            Types::Multiple(s) => s.contains(value),
        }
    }
    pub fn insert(&mut self, value: Type) {
        match self {
            Types::Single(s) => {
                if s != &value {
                    *self = Types::Multiple(HashSet::from([s.clone(), value]));
                }
            }
            Types::Multiple(s) => {
                if !s.contains(&value) {
                    s.insert(value);
                }
            }
        }
    }

    pub fn iter(&self) -> Box<dyn '_ + Iterator<Item = &'_ Type>> {
        match self {
            Types::Single(s) => Box::new(std::iter::once(s)),
            Types::Multiple(m) => Box::new(m.iter()),
        }
    }

    pub fn as_single(&self) -> Option<&Type> {
        match self {
            Self::Single(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_multiple(&self) -> Option<&HashSet<Type>> {
        match self {
            Self::Multiple(s) => Some(s),
            _ => None,
        }
    }
    pub fn try_into_single(self) -> Result<Type, Self> {
        if let Self::Single(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl TryFrom<Vec<Type>> for Types {
    type Error = Vec<Type>;
    fn try_from(mut ts: Vec<Type>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            return Err(ts);
        }
        if ts.len() == 1 {
            return Ok(Types::Single(ts.remove(0)));
        }
        Ok(Types::Multiple(HashSet::from_iter(ts.into_iter())))
    }
}

impl TryFrom<HashSet<Type>> for Types {
    type Error = HashSet<Type>;
    fn try_from(ts: HashSet<Type>) -> Result<Self, Self::Error> {
        if ts.is_empty() {
            return Err(ts);
        }
        if ts.len() == 1 {
            return Ok(Types::Single(ts.into_iter().next().unwrap()));
        }
        Ok(Types::Multiple(ts))
    }
}

impl<'a> TryFrom<&'a [Type]> for Types {
    type Error = &'a [Type];
    fn try_from(ts: &'a [Type]) -> Result<Self, Self::Error> {
        let mut hs = HashSet::from_iter(ts.iter().cloned());
        match hs.len() {
            0 => Err(ts),
            1 => Ok(Types::Single(hs.drain().next().unwrap())),
            _ => Ok(Types::Multiple(hs)),
        }
    }
}

impl<'a, 'b> TryFrom<&'a [&'b str]> for Types {
    type Error = &'a [&'b str];
    fn try_from(ts: &'a [&'b str]) -> Result<Self, Self::Error> {
        let mut hs = HashSet::from_iter(ts.iter().map(|s| Type::from(*s)));
        match hs.len() {
            0 => Err(ts),
            1 => Ok(Types::Single(hs.drain().next().unwrap())),
            _ => Ok(Types::Multiple(hs)),
        }
    }
}
impl<'a> TryFrom<Vec<&'a str>> for Types {
    type Error = Vec<&'a str>;
    fn try_from(ts: Vec<&'a str>) -> Result<Self, Self::Error> {
        match ts.len() {
            0 => Err(ts),
            1 => Ok(Types::Single(Type::from(ts[0]))),
            _ => Ok(Types::Multiple(ts.iter().map(|s| Type::from(*s)).collect())),
        }
    }
}
impl TryFrom<Vec<String>> for Types {
    type Error = Vec<String>;
    fn try_from(ts: Vec<String>) -> Result<Self, Self::Error> {
        match ts.len() {
            0 => Err(ts),
            1 => Ok(Types::Single(Type::from(ts[0].as_str()))),
            _ => Ok(Types::Multiple(ts.iter().map(Type::from).collect())),
        }
    }
}

impl<'a> TryFrom<&'a [String]> for Types {
    type Error = &'a [String];
    fn try_from(ts: &'a [String]) -> Result<Self, Self::Error> {
        match ts.len() {
            0 => Err(ts),
            1 => Ok(Types::Single(Type::from(ts[0].as_str()))),
            _ => Ok(Types::Multiple(ts.iter().map(Type::from).collect())),
        }
    }
}

impl From<Types> for HashSet<Type> {
    fn from(value: Types) -> Self {
        match value {
            Types::Single(t) => HashSet::from_iter(std::iter::once(t)),
            Types::Multiple(ts) => ts,
        }
    }
}

impl From<Types> for Vec<Type> {
    fn from(t: Types) -> Self {
        match t {
            Types::Single(t) => vec![t],
            Types::Multiple(ts) => ts.into_iter().collect(),
        }
    }
}
impl<'a> IntoIterator for &'a Types {
    type Item = &'a Type;
    type IntoIter = Box<dyn 'a + Iterator<Item = Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for Types {
    type Item = Type;
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        match self {
            Types::Single(t) => Box::new(std::iter::once(t)),
            Types::Multiple(ts) => Box::new(ts.into_iter()),
        }
    }
}
