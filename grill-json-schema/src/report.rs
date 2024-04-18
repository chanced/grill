use grill_core::{criterion, uri::AbsoluteUri};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

// /// Set of keywords to check which disable short-circuiting
// pub const DISABLING_KEYWORDS: [&'static str; 2] = [UNEVALUATED_PROPERTIES, UNEVALUATED_ITEMS];

// if Self::ENABLING_STRUCTURES.contains(ctx.structure().into()) {
//     ctx.enable_short_circuiting();
// }
// pub const ENABLING_STRUCTURES: Structures = Structures::FLAG;
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, strum::Display,
)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum Output {
    /// A concise [`Output`] [`Structure`] which only contains a single
    /// `"valid"` `bool` field.
    ///
    /// This `Structure` may have a positive impact on
    /// performance as [`Keyword`]s are expected to short circuit and return errors as
    /// soon as possible.
    ///
    /// # Example
    /// ```json
    /// { "valid": false }
    /// ```
    ///
    /// - [JSON Schema Core 2020-12 # 12.4.1
    ///   `Flag`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-flag)
    Flag = 1,
    /// The `Basic` structure is a flat list of output units.
    /// # Example
    /// ```json
    /// {
    ///   "valid": false,
    ///   "errors": [
    ///     {
    ///       "keywordLocation": "",
    ///       "instanceLocation": "",
    ///       "error": "A subschema had errors."
    ///     },
    ///     {
    ///       "keywordLocation": "/items/$ref",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point",
    ///       "instanceLocation": "/1",
    ///       "error": "A subschema had errors."
    ///     },
    ///     {
    ///       "keywordLocation": "/items/$ref/required",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point/required",
    ///       "instanceLocation": "/1",
    ///       "error": "Required property 'y' not found."
    ///     },
    ///     {
    ///       "keywordLocation": "/items/$ref/additionalProperties",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point/additionalProperties",
    ///       "instanceLocation": "/1/z",
    ///       "error": "Additional property 'z' found but was invalid."
    ///     },
    ///     {
    ///       "keywordLocation": "/minItems",
    ///       "instanceLocation": "",
    ///       "error": "Expected at least 3 items but found 2"
    ///     }
    ///   ]
    /// }
    /// ```
    Basic = 2,
    /// The `Detailed` structure is based on the schema and can be more readable
    /// for both humans and machines. Having the structure organized this way
    /// makes associations between the errors more apparent. For example, the
    /// fact that the missing "y" property and the extra "z" property both stem
    /// from the same location in the instance is not immediately obvious in the
    /// "Basic" structure. In a hierarchy, the correlation is more easily
    /// identified.
    ///
    /// The following rules govern the construction of the results object:
    ///
    /// - All applicator keywords (`"*Of"`, `"$ref"`, `"if"`/`"then"`/`"else"`,
    ///   etc.) require a node.
    /// - Nodes that have no children are removed.
    /// - Nodes that have a single child are replaced by the child.
    /// - Branch nodes do not require an error message or an annotation.
    ///
    /// # Example
    ///
    /// ## Schema:
    /// ```json
    /// {
    ///   "$id": "https://example.com/polygon",
    ///   "$schema": "https://json-schema.org/draft/2020-12/schema",
    ///   "$defs": {
    ///     "point": {
    ///       "type": "object",
    ///       "properties": {
    ///         "x": { "type": "number" },
    ///         "y": { "type": "number" }
    ///       },
    ///       "additionalProperties": false,
    ///       "required": [ "x", "y" ]
    ///     }
    ///   },
    ///   "type": "array",
    ///   "items": { "$ref": "#/$defs/point" },
    ///   "minItems": 3
    /// }
    /// ```
    /// ## Instance:
    /// ```json
    /// [ { "x": 2.5, "y": 1.3 }, { "x": 1, "z": 6.7 } ]
    /// ```
    /// ## Output:
    ///
    /// ```json
    /// {
    ///   "valid": false,
    ///   "keywordLocation": "",
    ///   "instanceLocation": "",
    ///   "errors": [
    ///     {
    ///       "valid": false,
    ///       "keywordLocation": "/items/$ref",
    ///       "absoluteKeywordLocation":
    ///         "https://example.com/polygon#/$defs/point",
    ///       "instanceLocation": "/1",
    ///       "errors": [
    ///         {
    ///           "valid": false,
    ///           "keywordLocation": "/items/$ref/required",
    ///           "absoluteKeywordLocation":
    ///             "https://example.com/polygon#/$defs/point/required",
    ///           "instanceLocation": "/1",
    ///           "error": "Required property 'y' not found."
    ///         },
    ///         {
    ///           "valid": false,
    ///           "keywordLocation": "/items/$ref/additionalProperties",
    ///           "absoluteKeywordLocation":
    ///             "https://example.com/polygon#/$defs/point/additionalProperties",
    ///           "instanceLocation": "/1/z",
    ///           "error": "Additional property 'z' found but was invalid."
    ///         }
    ///       ]
    ///     },
    ///     {
    ///       "valid": false,
    ///       "keywordLocation": "/minItems",
    ///       "instanceLocation": "",
    ///       "error": "Expected at least 3 items but found 2"
    ///     }
    ///   ]
    /// }
    ///
    Detailed = 4,
    Verbose = 8,
}

impl criterion::Output for Output {
    fn verbose() -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub enum Report<'v> {
    Flag(Flag<'v>),
    Basic(Basic<'v>),
    Detailed(Detailed<'v>),
    Verbose(Verbose<'v>),
}

impl<'v> criterion::Report<'v> for Report<'v> {
    type Output = Output;
    type Owned = Report<'static>;

    fn is_valid(&self) -> bool {
        todo!()
    }

    fn append(&mut self, _nodes: impl Iterator<Item = Self>) {
        todo!()
    }

    fn push(&mut self, _output: Self) {
        todo!()
    }

    fn into_owned(self) -> Self::Owned {
        todo!()
    }

    fn new(
        _output: Self::Output,
        _absolute_keyword_location: &AbsoluteUri,
        _keyword_location: jsonptr::Pointer,
        _instance_location: jsonptr::Pointer,
    ) -> Self {
        todo!()
    }
}
impl<'de> Deserialize<'de> for Report<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Report<'static>, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

impl Serialize for Report<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct Flag<'v> {
    marker: std::marker::PhantomData<&'v ()>,
}

#[derive(Clone, Debug)]
pub struct Basic<'v> {
    marker: std::marker::PhantomData<&'v ()>,
}

#[derive(Clone, Debug)]
pub struct Detailed<'v> {
    marker: std::marker::PhantomData<&'v ()>,
}

#[derive(Clone, Debug)]
pub struct Verbose<'v> {
    marker: std::marker::PhantomData<&'v ()>,
}

#[derive(Clone, Debug)]
pub enum Annotation<'v> {
    Schema(crate::keyword::schema::Annotation<'v>),
}
