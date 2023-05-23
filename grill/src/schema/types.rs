use std::{collections::HashSet, fmt::Display, str::FromStr};

use itertools::Itertools;
// use heck::ToLowerCamelCase;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const ARRAY: &str = "array";
const BOOLEAN: &str = "boolean";
const INTEGER: &str = "integer";
const NULL: &str = "null";
const OBJECT: &str = "object";
const STRING: &str = "string";
const NUMBER: &str = "number";

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(from = "String", into = "String")]
/// Possible values for the `"type"` keyword, represented as [`Types`].
///
/// <https://json-schema.org/understanding-json-schema/reference/type.html>
pub enum Type {
    /// Arrays are used for ordered elements. In JSON, each element in an array
    /// may be of a different type.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/array.html#array>
    Array,
    /// The boolean type matches only two special values: `true` and `false`. Note
    /// that values that evaluate to true or false, such as `1` and `0`, are not
    /// accepted by the schema.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/boolean.html#boolean>
    Boolean,
    /// The integer type is used for integral numbers. JSON does not have
    /// distinct types for integers and floating-point values. Therefore, the
    /// presence or absence of a decimal point is not enough to distinguish
    /// between integers and non-integers. For example, `1` and `1.0` are two ways
    /// to represent the same value in JSON. JSON Schema considers that value an
    /// integer no matter which representation was used.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/numeric.html#integer>
    Integer,
    /// When a schema specifies a type of null, it has only one acceptable value: `null`.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/null.html#null>
    Null,
    /// Number used for any numeric type, either integers or floating point
    /// numbers.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/numeric.html#number>
    Number,
    /// Objects are the mapping type in JSON. They map “keys” to “values”. In
    /// JSON, the “keys” must always be strings. Each of these pairs is
    /// conventionally referred to as a “property”.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/object.html#object>
    Object,
    /// The string type is used for strings of text. It may contain Unicode characters.
    ///
    /// - <https://json-schema.org/understanding-json-schema/reference/string.html#string>
    String,
    /// Non-core types
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
impl PartialEq<Type> for str {
    fn eq(&self, other: &Type) -> bool {
        other.as_str() == self
    }
}
impl PartialEq<str> for Type {
    fn eq(&self, other: &str) -> bool {
        match self {
            Type::Array => other == ARRAY,
            Type::Boolean => other == BOOLEAN,
            Type::Integer => other == INTEGER,
            Type::Null => other == NULL,
            Type::Number => other == NUMBER,
            Type::Object => other == OBJECT,
            Type::String => other == STRING,
            Type::Other(s) => other == s.as_str(),
        }
    }
}
impl PartialEq<String> for Type {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Type {
    #[must_use]
    /// Returns the [`&str`] representation of the `Type`.
    pub fn as_str(&self) -> &str {
        match self {
            Type::Array => ARRAY,
            Type::Boolean => BOOLEAN,
            Type::Integer => INTEGER,
            Type::Null => NULL,
            Type::Number => NUMBER,
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
            Type::Number => write!(f, "{NUMBER}"),
            Type::Object => write!(f, "{OBJECT}"),
            Type::String => write!(f, "{STRING}"),
            Type::Other(other) => write!(f, "{other}"),
        }
    }
}

impl PartialEq<Type> for &str {
    fn eq(&self, other: &Type) -> bool {
        *self == other.as_str()
    }
}
impl PartialEq<Type> for String {
    fn eq(&self, other: &Type) -> bool {
        self.as_str() == other.as_str()
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
            NUMBER => Type::Number,
            _ => {
                let t = s.to_lowercase();
                if t == s {
                    Type::Other(s.to_string())
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
/// The `"type"` keyword is fundamental to JSON Schema. It specifies the data
/// type for a schema.
///
/// The type keyword may either be a string or an array:
///
/// If it’s a string, it is the name of one of the basic types above.
///
/// If it is an array, it must be an array of strings, where each string is the
/// name of one of the basic types, and each element is unique. In this case, the
/// JSON snippet is valid if it matches any of the given types.
pub enum Types {
    /// A single [`Type`], represented as a string.
    Single(Type),
    /// A set of [`Type`]s, represented as an array of strings.
    Set(Vec<Type>),
}

impl Types {
    /// Returns the [`Types`](crate::Types) of a [`serde_json::Value`].
    #[must_use]
    pub fn of_value(value: &Value) -> Self {
        use serde_json::Value::*;
        match value {
            Null => Types::Single(Type::Null),
            Bool(_) => Types::Single(Type::Boolean),
            Number(n) => {
                // TODO: handle large numbers
                if n.is_i64() {
                    Types::Set(vec![Type::Number, Type::Integer])
                } else {
                    Types::Single(Type::Number)
                }
            }
            String(_) => Types::Single(Type::String),
            Array(_) => Types::Single(Type::Array),
            Object(_) => Types::Single(Type::Object),
        }
    }

    /// Returns `true` if the types is [`Single`].
    ///
    /// [`Single`]: Types::Single
    #[must_use]
    pub fn is_single(&self) -> bool {
        matches!(self, Self::Single(..))
    }
    /// Returns `true` if the [`Type`] `typ` is present
    #[must_use]
    pub fn contains(&self, typ: &Type) -> bool {
        match self {
            Types::Single(s) => s == typ,
            Types::Set(s) => s.contains(typ),
        }
    }

    #[must_use]
    pub fn contains_any(&self, types: &Types) -> bool {
        match self {
            Types::Single(s) => types.contains(s),
            Types::Set(s) => s.iter().any(|t| types.contains(t)),
        }
    }

    /// Inserts `value` into the [`Types`].
    ///
    /// If the [`Types`] is [`Single`](`Types::Single`), it will be converted to [`Set`](`Types::Set`).
    pub fn insert(&mut self, value: Type) {
        match self {
            Types::Single(s) => {
                if s != &value {
                    *self = Types::Set(vec![s.clone(), value]);
                }
            }
            Types::Set(s) => {
                if !s.contains(&value) {
                    s.push(value);
                }
            }
        }
    }
    /// Returns an [`Iterator`] of [`Type`] within the [`Types`].
    #[must_use]
    pub fn iter(&self) -> Box<dyn '_ + Iterator<Item = &'_ Type>> {
        match self {
            Types::Single(s) => Box::new(std::iter::once(s)),
            Types::Set(m) => Box::new(m.iter()),
        }
    }

    /// Returns `true` if the types is [`Set`].
    ///
    /// [`Set`]: Types::Set
    #[must_use]
    pub fn is_set(&self) -> bool {
        matches!(self, Self::Set(..))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Types::Single(_) => 1,
            Types::Set(s) => s.len(),
        }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Types::Single(_) => false,
            Types::Set(s) => s.is_empty(),
        }
    }
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Single(t) => Display::fmt(t, f),
            Types::Set(s) => match s.len() {
                0 => write!(f, "[]"),
                1 => write!(f, "[{}]", s.iter().next().unwrap()),
                _ => {
                    let result =
                        serde_json::to_string(&s).expect("Types HashSet to json should not fail");
                    write!(f, "{result}")
                }
            },
        }
    }
}
impl From<Type> for Types {
    fn from(t: Type) -> Self {
        Types::Single(t)
    }
}

impl PartialEq for Types {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Single(s), Self::Single(o)) => s == o,
            (Self::Set(s), Self::Set(o)) => s.iter().all(|i| o.contains(i)),
            _ => false,
        }
    }
}
impl Eq for Types {}

impl From<Vec<Type>> for Types {
    fn from(mut ts: Vec<Type>) -> Self {
        match ts.len() {
            1 => Types::Single(ts.remove(0)),
            _ => Types::Set(ts.into_iter().collect()),
        }
    }
}

impl From<HashSet<Type>> for Types {
    fn from(ts: HashSet<Type>) -> Self {
        match ts.len() {
            1 => Types::Single(ts.into_iter().next().unwrap()),
            _ => Types::Set(ts.into_iter().collect_vec()),
        }
    }
}

impl From<&str> for Types {
    fn from(value: &str) -> Self {
        Types::Single(Type::from(value))
    }
}
impl From<String> for Types {
    fn from(value: String) -> Self {
        Types::Single(Type::from(value))
    }
}
impl From<&[Type]> for Types {
    fn from(ts: &[Type]) -> Self {
        let mut hs = ts.iter().cloned().collect::<HashSet<_>>();
        match hs.len() {
            1 => Types::Single(hs.drain().next().unwrap()),
            _ => Types::Set(hs.into_iter().collect()),
        }
    }
}

impl From<&[&str]> for Types {
    fn from(ts: &[&str]) -> Self {
        let mut hs: HashSet<Type> = ts.iter().map(|s| Type::from(*s)).collect();
        match hs.len() {
            1 => Types::Single(hs.drain().next().unwrap()),
            _ => Types::Set(hs.into_iter().collect()),
        }
    }
}
impl From<Vec<&str>> for Types {
    fn from(ts: Vec<&str>) -> Self {
        Self::from(ts.as_slice())
    }
}
impl From<Vec<String>> for Types {
    fn from(ts: Vec<String>) -> Self {
        match ts.len() {
            1 => Types::Single(Type::from(ts[0].as_str())),
            _ => Types::Set(ts.iter().map(Type::from).collect()),
        }
    }
}

impl From<&[String]> for Types {
    fn from(ts: &[String]) -> Self {
        match ts.len() {
            1 => Types::Single(Type::from(ts[0].as_str())),
            _ => Types::Set(ts.iter().map(Type::from).collect()),
        }
    }
}

impl From<Types> for HashSet<Type> {
    fn from(value: Types) -> Self {
        match value {
            Types::Single(t) => {
                let mut hs: HashSet<Type> = HashSet::with_capacity(1);
                hs.insert(t);
                hs
            }
            Types::Set(ts) => ts.into_iter().collect(),
        }
    }
}

impl From<Types> for Vec<Type> {
    fn from(t: Types) -> Self {
        match t {
            Types::Single(t) => vec![t],
            Types::Set(ts) => ts.into_iter().collect(),
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
            Types::Set(ts) => Box::new(ts.into_iter()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use serde_json::json;

    use super::*;
    #[derive(Serialize, Deserialize, Debug)]
    struct Obj {
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub types: Option<Types>,
    }

    #[test]
    fn test_fmt() {
        let t = Types::Single(Type::from("test"));
        assert_eq!(t.to_string(), "test");
    }

    #[test]
    fn test_serde() {
        let t = Types::Single(Type::from("test"));
        let s = serde_json::to_string(&t).expect("Types to json should not fail");
        assert_eq!(s, "\"test\"");
        let t2: Types = serde_json::from_str(&s).expect("Types from json should not fail");
        assert_eq!(t, t2);

        let obj_json = json!(
            {
                "type": "object"
            }
        );
        let Obj { types } =
            serde_json::from_value(obj_json).expect("Obj from json should not fail");
        assert_eq!(types.unwrap(), Types::Single(Type::Object));
    }
}
