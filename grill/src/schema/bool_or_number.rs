use num_rational::BigRational;
use serde_json::Number;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum BoolOrNumber {
    Bool(bool),
    Number(Number),
}

impl BoolOrNumber {
    /// Returns `true` if the bool or number is [`Bool`].
    ///
    /// [`Bool`]: BoolOrNumber::Bool
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    pub fn as_bool(&self) -> Option<&bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns `true` if the bool or number is [`Number`].
    ///
    /// [`Number`]: BoolOrNumber::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    pub fn as_number(&self) -> Option<&Number> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompiledBoolOrNumber {
    Bool(bool),
    Number(BigRational),
}

impl CompiledBoolOrNumber {
    /// Returns `true` if the compiled bool or number is [`Bool`].
    ///
    /// [`Bool`]: CompiledBoolOrNumber::Bool
    #[must_use]
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    /// Returns `true` if the compiled bool or number is [`Number`].
    ///
    /// [`Number`]: CompiledBoolOrNumber::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    pub fn as_number(&self) -> Option<&BigRational> {
        if let Self::Number(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<&bool> {
        if let Self::Bool(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
