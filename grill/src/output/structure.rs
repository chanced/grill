use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Structure {
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
    Flag,
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
    Basic,
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
    Detailed,
    Verbose,
}

impl Structure {
    /// Returns `true` if the structure is [`Flag`].
    ///
    /// [`Flag`]: Structure::Flag
    #[must_use]
    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag)
    }

    /// Returns `true` if the structure is [`Basic`].
    ///
    /// [`Basic`]: Structure::Basic
    #[must_use]
    pub fn is_basic(&self) -> bool {
        matches!(self, Self::Basic)
    }

    /// Returns `true` if the structure is [`Detailed`].
    ///
    /// [`Detailed`]: Structure::Detailed
    #[must_use]
    pub fn is_detailed(&self) -> bool {
        matches!(self, Self::Detailed)
    }

    /// Returns `true` if the structure is [`Verbose`].
    ///
    /// [`Verbose`]: Structure::Verbose
    #[must_use]
    pub fn is_verbose(&self) -> bool {
        matches!(self, Self::Verbose)
    }
}
