use std::borrow::Cow;

use serde_json::Value;

use grill_core::schema::Dialect;

use crate::keyword::{self, ID, SCHEMA};

use super::{keyword::REF, metaschema};

/// Constructs a new [`Dialect`] for Draft 2020-12
#[must_use]
pub fn dialect() -> Dialect {
    Dialect::build(json_schema_2020_12_uri().clone())
        .add_metaschema(
            json_schema_2020_12_uri().clone(),
            Cow::Borrowed(json_schema_2020_12_value()),
        )
        .add_keyword(keyword::schema::Schema::new(SCHEMA, false))
        .add_keyword(keyword::id::Id::new(ID, false))
        .add_keyword(keyword::boolean::Boolean::default())
        .add_keyword(keyword::comment::Comment::default())
        .add_keyword(keyword::const_::Const::new(None))
        .add_keyword(keyword::defs::Defs)
        .add_keyword(keyword::enum_::Enum::new(None))
        .add_keyword(keyword::if_then_else::IfThenElse::default())
        .add_keyword(keyword::not::Not::default())
        .add_keyword(keyword::pattern_properties::PatternProperties::default())
        .add_keyword(keyword::properties::Properties::default())
        .add_keyword(keyword::read_only::ReadOnly::default())
        .add_keyword(keyword::write_only::WriteOnly::default())
        .add_keyword(keyword::ref_::Ref::new(REF, true))
        .add_keyword(keyword::type_::Type::new(None))
        .finish()
        .unwrap()
}

/// Returns the static `Value` of the primary schema for Draft 2020-12
///
/// This is an alias to [`json_schema_2020_12_value`]
#[must_use]
pub fn schema() -> &'static Value {
    json_schema_2020_12_value()
}

metaschema!(
    [JSON Schema 2020_12]("https://json-schema.org/draft/2020-12/schema")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/schema",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/core": true,
            "https://json-schema.org/draft/2020-12/vocab/applicator": true,
            "https://json-schema.org/draft/2020-12/vocab/unevaluated": true,
            "https://json-schema.org/draft/2020-12/vocab/validation": true,
            "https://json-schema.org/draft/2020-12/vocab/meta-data": true,
            "https://json-schema.org/draft/2020-12/vocab/format-annotation": true,
            "https://json-schema.org/draft/2020-12/vocab/content": true
        },
        "$dynamicAnchor": "meta",

        "title": "Core and Validation specifications meta-schema",
        "allOf": [
            {"$ref": "meta/core"},
            {"$ref": "meta/applicator"},
            {"$ref": "meta/unevaluated"},
            {"$ref": "meta/validation"},
            {"$ref": "meta/meta-data"},
            {"$ref": "meta/format-annotation"},
            {"$ref": "meta/content"}
        ],
        "type": ["object", "boolean"],
        "$comment": "This meta-schema also defines keywords that have appeared in previous drafts in order to prevent incompatible extensions as they remain in common use.",
        "properties": {
            "definitions": {
                "$comment": "\"definitions\" has been replaced by \"$defs\".",
                "type": "object",
                "additionalProperties": { "$dynamicRef": "#meta" },
                "deprecated": true,
                "default": {}
            },
            "dependencies": {
                "$comment": "\"dependencies\" has been split and replaced by \"dependentSchemas\" and \"dependentRequired\" in order to serve their differing semantics.",
                "type": "object",
                "additionalProperties": {
                    "anyOf": [
                        { "$dynamicRef": "#meta" },
                        { "$ref": "meta/validation#/$defs/stringArray" }
                    ]
                },
                "deprecated": true,
                "default": {}
            },
            "$recursiveAnchor": {
                "$comment": "\"$recursiveAnchor\" has been replaced by \"$dynamicAnchor\".",
                "$ref": "meta/core#/$defs/anchorString",
                "deprecated": true
            },
            "$recursiveRef": {
                "$comment": "\"$recursiveRef\" has been replaced by \"$dynamicRef\".",
                "$ref": "meta/core#/$defs/uriReferenceString",
                "deprecated": true
            }
        }
    }
);

metaschema!(
    [JSON Schema 2020_12 Core]("https://json-schema.org/draft/2020-12/meta/core")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/core",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/core": true
        },
        "$dynamicAnchor": "meta",

        "title": "Core vocabulary meta-schema",
        "type": ["object", "boolean"],
        "properties": {
            "$id": {
                "$ref": "#/$defs/uriReferenceString",
                "$comment": "Non-empty fragments not allowed.",
                "pattern": "^[^#]*#?$"
            },
            "$schema": { "$ref": "#/$defs/uriString" },
            "$ref": { "$ref": "#/$defs/uriReferenceString" },
            "$anchor": { "$ref": "#/$defs/anchorString" },
            "$dynamicRef": { "$ref": "#/$defs/uriReferenceString" },
            "$dynamicAnchor": { "$ref": "#/$defs/anchorString" },
            "$vocabulary": {
                "type": "object",
                "propertyNames": { "$ref": "#/$defs/uriString" },
                "additionalProperties": {
                    "type": "boolean"
                }
            },
            "$comment": {
                "type": "string"
            },
            "$defs": {
                "type": "object",
                "additionalProperties": { "$dynamicRef": "#meta" }
            }
        },
        "$defs": {
            "anchorString": {
                "type": "string",
                "pattern": "^[A-Za-z_][-A-Za-z0-9._]*$"
            },
            "uriString": {
                "type": "string",
                "format": "uri"
            },
            "uriReferenceString": {
                "type": "string",
                "format": "uri-reference"
            }
        }
    }
);

metaschema!(
    [JSON Schema 2020_12 Content]("https://json-schema.org/draft/2020-12/meta/content")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/content",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/content": true
        },
        "$dynamicAnchor": "meta",

        "title": "Content vocabulary meta-schema",

        "type": ["object", "boolean"],
        "properties": {
            "contentEncoding": { "type": "string" },
            "contentMediaType": { "type": "string" },
            "contentSchema": { "$dynamicRef": "#meta" }
        }
    }
);

metaschema!(
    [JSON Hyper Schema 2020_12 Links]("https://json-schema.org/draft/2020-12/schema")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/links",
        "title": "Link Description Object",

        "type": "object",
        "properties": {
            "anchor": {
                "type": "string",
                "format": "uri-template"
            },
            "anchorPointer": {
                "type": "string",
                "anyOf": [
                    { "format": "json-pointer" },
                    { "format": "relative-json-pointer" }
                ]
            },
            "rel": {
                "anyOf": [
                    { "type": "string" },
                    {
                        "type": "array",
                        "items": { "type": "string" },
                        "minItems": 1
                    }
                ]
            },
            "href": {
                "type": "string",
                "format": "uri-template"
            },
            "hrefSchema": {
                "$dynamicRef": "https://json-schema.org/draft/2020-12/hyper-schema#meta",
                "default": false
            },
            "templatePointers": {
                "type": "object",
                "additionalProperties": {
                    "type": "string",
                    "anyOf": [
                        { "format": "json-pointer" },
                        { "format": "relative-json-pointer" }
                    ]
                }
            },
            "templateRequired": {
                "type": "array",
                "items": {
                    "type": "string"
                },
                "uniqueItems": true
            },
            "title": {
                "type": "string"
            },
            "description": {
                "type": "string"
            },
            "targetSchema": {
                "$dynamicRef": "https://json-schema.org/draft/2020-12/hyper-schema#meta",
                "default": true
            },
            "targetMediaType": {
                "type": "string"
            },
            "targetHints": {},
            "headerSchema": {
                "$dynamicRef": "https://json-schema.org/draft/2020-12/hyper-schema#meta",
                "default": true
            },
            "submissionMediaType": {
                "type": "string",
                "default": "application/json"
            },
            "submissionSchema": {
                "$dynamicRef": "https://json-schema.org/draft/2020-12/hyper-schema#meta",
                "default": true
            },
            "$comment": {
                "type": "string"
            }
        },
        "required": [ "rel", "href" ]
    }
);

metaschema!(
    [JSON Schema 2020_12 Output]("https://json-schema.org/draft/2020-12/output/schema")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/output/schema",
        "description": "A schema that validates the minimum requirements for validation output",

        "anyOf": [
          { "$ref": "#/$defs/flag" },
          { "$ref": "#/$defs/basic" },
          { "$ref": "#/$defs/detailed" },
          { "$ref": "#/$defs/verbose" }
        ],
        "$defs": {
          "outputUnit":{
            "properties": {
              "valid": { "type": "boolean" },
              "keywordLocation": {
                "type": "string",
                "format": "json-pointer"
              },
              "absoluteKeywordLocation": {
                "type": "string",
                "format": "uri"
              },
              "instanceLocation": {
                "type": "string",
                "format": "json-pointer"
              },
              "error": {
                "type": "string"
              },
              "errors": {
                "$ref": "#/$defs/outputUnitArray"
              },
              "annotations": {
                "$ref": "#/$defs/outputUnitArray"
              }
            },
            "required": [ "valid", "keywordLocation", "instanceLocation" ],
            "allOf": [
              {
                "if": {
                  "properties": {
                    "valid": { "const": false }
                  }
                },
                "then": {
                  "anyOf": [
                    {
                      "required": [ "error" ]
                    },
                    {
                      "required": [ "errors" ]
                    }
                  ]
                }
              },
              {
                "if": {
                  "anyOf": [
                    {
                      "properties": {
                        "keywordLocation": {
                          "pattern": "/\\$ref/"
                        }
                      }
                    },
                    {
                      "properties": {
                        "keywordLocation": {
                          "pattern": "/\\$dynamicRef/"
                        }
                      }
                    }
                  ]
                },
                "then": {
                  "required": [ "absoluteKeywordLocation" ]
                }
              }
            ]
          },
          "outputUnitArray": {
            "type": "array",
            "items": { "$ref": "#/$defs/outputUnit" }
          },
          "flag": {
            "properties": {
              "valid": { "type": "boolean" }
            },
            "required": [ "valid" ]
          },
          "basic": { "$ref": "#/$defs/outputUnit" },
          "detailed": { "$ref": "#/$defs/outputUnit" },
          "verbose": { "$ref": "#/$defs/outputUnit" }
        }
      }
);

metaschema!(
    [JSON Schema 2020_12 Applicator]("https://json-schema.org/draft/2020-12/meta/applicator")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/applicator",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/applicator": true
        },
        "$dynamicAnchor": "meta",

        "title": "Applicator vocabulary meta-schema",
        "type": ["object", "boolean"],
        "properties": {
            "prefixItems": { "$ref": "#/$defs/schemaArray" },
            "items": { "$dynamicRef": "#meta" },
            "contains": { "$dynamicRef": "#meta" },
            "additionalProperties": { "$dynamicRef": "#meta" },
            "properties": {
                "type": "object",
                "additionalProperties": { "$dynamicRef": "#meta" },
                "default": {}
            },
            "patternProperties": {
                "type": "object",
                "additionalProperties": { "$dynamicRef": "#meta" },
                "propertyNames": { "format": "regex" },
                "default": {}
            },
            "dependentSchemas": {
                "type": "object",
                "additionalProperties": { "$dynamicRef": "#meta" },
                "default": {}
            },
            "propertyNames": { "$dynamicRef": "#meta" },
            "if": { "$dynamicRef": "#meta" },
            "then": { "$dynamicRef": "#meta" },
            "else": { "$dynamicRef": "#meta" },
            "allOf": { "$ref": "#/$defs/schemaArray" },
            "anyOf": { "$ref": "#/$defs/schemaArray" },
            "oneOf": { "$ref": "#/$defs/schemaArray" },
            "not": { "$dynamicRef": "#meta" }
        },
        "$defs": {
            "schemaArray": {
                "type": "array",
                "minItems": 1,
                "items": { "$dynamicRef": "#meta" }
            }
        }
    }
);

metaschema!(
    [JSON Schema 2020_12 Format Assertion]("https://json-schema.org/draft/2020-12/meta/format-assertion")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/format-assertion",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/format-assertion": true
        },
        "$dynamicAnchor": "meta",

        "title": "Format vocabulary meta-schema for assertion results",
        "type": ["object", "boolean"],
        "properties": {
            "format": { "type": "string" }
        }
    }
);

metaschema!(
    [JSON Schema 2020_12 Format Annotation]("https://json-schema.org/draft/2020-12/meta/format-annotation")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/format-annotation",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/format-annotation": true
        },
        "$dynamicAnchor": "meta",

        "title": "Format vocabulary meta-schema for annotation results",
        "type": ["object", "boolean"],
        "properties": {
            "format": { "type": "string" }
        }
    }

);

metaschema!(
    [JSON Hyper Schema 2020_12]("https://json-schema.org/draft/2020-12/meta/hyper-schema")
    {
        "$schema": "https://json-schema.org/draft/2020-12/hyper-schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/hyper-schema",
        "$vocabulary": {
            "https://json-schema.org/draft/2019-09/vocab/hyper-schema": true
        },
        "$dynamicAnchor": "meta",

        "title": "JSON Hyper-Schema Vocabulary Schema",
        "type": ["object", "boolean"],
        "properties": {
            "base": {
                "type": "string",
                "format": "uri-template"
            },
            "links": {
                "type": "array",
                "items": {
                    "$ref": "https://json-schema.org/draft/2020-12/links"
                }
            }
        },
        "links": [
            {
                "rel": "self",
                "href": "{+%24id}"
            }
        ]
    }
);

metaschema!(
    [JSON Schema 2020_12 Metadata]("https://json-schema.org/draft/2020-12/meta/meta-data")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/meta-data",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/meta-data": true
        },
        "$dynamicAnchor": "meta",

        "title": "Meta-data vocabulary meta-schema",

        "type": ["object", "boolean"],
        "properties": {
            "title": {
                "type": "string"
            },
            "description": {
                "type": "string"
            },
            "default": true,
            "deprecated": {
                "type": "boolean",
                "default": false
            },
            "readOnly": {
                "type": "boolean",
                "default": false
            },
            "writeOnly": {
                "type": "boolean",
                "default": false
            },
            "examples": {
                "type": "array",
                "items": true
            }
        }
    }
);

metaschema!(
    [JSON Schema 2020_12 Unevaluated]("https://json-schema.org/draft/2020-12/meta/unevaluated")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/unevaluated",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/unevaluated": true
        },
        "$dynamicAnchor": "meta",

        "title": "Unevaluated applicator vocabulary meta-schema",
        "type": ["object", "boolean"],
        "properties": {
            "unevaluatedItems": { "$dynamicRef": "#meta" },
            "unevaluatedProperties": { "$dynamicRef": "#meta" }
        }
    }
);

metaschema!(
    [JSON Schema 2020_12 Validation]("https://json-schema.org/draft/2020-12/meta/validation")
    {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://json-schema.org/draft/2020-12/meta/validation",
        "$vocabulary": {
            "https://json-schema.org/draft/2020-12/vocab/validation": true
        },
        "$dynamicAnchor": "meta",

        "title": "Validation vocabulary meta-schema",
        "type": ["object", "boolean"],
        "properties": {
            "type": {
                "anyOf": [
                    { "$ref": "#/$defs/simpleTypes" },
                    {
                        "type": "array",
                        "items": { "$ref": "#/$defs/simpleTypes" },
                        "minItems": 1,
                        "uniqueItems": true
                    }
                ]
            },
            "const": true,
            "enum": {
                "type": "array",
                "items": true
            },
            "multipleOf": {
                "type": "number",
                "exclusiveMinimum": 0
            },
            "maximum": {
                "type": "number"
            },
            "exclusiveMaximum": {
                "type": "number"
            },
            "minimum": {
                "type": "number"
            },
            "exclusiveMinimum": {
                "type": "number"
            },
            "maxLength": { "$ref": "#/$defs/nonNegativeInteger" },
            "minLength": { "$ref": "#/$defs/nonNegativeIntegerDefault0" },
            "pattern": {
                "type": "string",
                "format": "regex"
            },
            "maxItems": { "$ref": "#/$defs/nonNegativeInteger" },
            "minItems": { "$ref": "#/$defs/nonNegativeIntegerDefault0" },
            "uniqueItems": {
                "type": "boolean",
                "default": false
            },
            "maxContains": { "$ref": "#/$defs/nonNegativeInteger" },
            "minContains": {
                "$ref": "#/$defs/nonNegativeInteger",
                "default": 1
            },
            "maxProperties": { "$ref": "#/$defs/nonNegativeInteger" },
            "minProperties": { "$ref": "#/$defs/nonNegativeIntegerDefault0" },
            "required": { "$ref": "#/$defs/stringArray" },
            "dependentRequired": {
                "type": "object",
                "additionalProperties": {
                    "$ref": "#/$defs/stringArray"
                }
            }
        },
        "$defs": {
            "nonNegativeInteger": {
                "type": "integer",
                "minimum": 0
            },
            "nonNegativeIntegerDefault0": {
                "$ref": "#/$defs/nonNegativeInteger",
                "default": 0
            },
            "simpleTypes": {
                "enum": [
                    "array",
                    "boolean",
                    "integer",
                    "null",
                    "number",
                    "object",
                    "string"
                ]
            },
            "stringArray": {
                "type": "array",
                "items": { "type": "string" },
                "uniqueItems": true,
                "default": []
            }
        }
    }

);
