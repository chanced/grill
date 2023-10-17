pub use super::common::id;
pub use super::common::schema;

use super::metaschema;
use grill_core::schema::Dialect;
use serde_json::Value;

#[must_use]
pub fn dialect() -> Dialect {
    todo!()
}

#[must_use]
/// [Alias of [`json_schema_07_value`](`json_schema_07_value`)] - Returns the
/// [`Value`] of the JSON Schema 07 metaschema.
pub fn schema() -> &'static Value {
    json_schema_07_value()
}

metaschema!(
    [JSON Schema 07]("http://json-schema.org/draft-07/schema#")
    {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "$id": "http://json-schema.org/draft-07/schema#",
        "title": "Core schema meta-schema",
        "definitions": {
            "schemaArray": {
                "type": "array",
                "minItems": 1,
                "items": { "$ref": "#" }
            },
            "nonNegativeInteger": {
                "type": "integer",
                "minimum": 0
            },
            "nonNegativeIntegerDefault0": {
                "allOf": [
                    { "$ref": "#/definitions/nonNegativeInteger" },
                    { "default": 0 }
                ]
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
        },
        "type": ["object", "boolean"],
        "properties": {
            "$id": {
                "type": "string",
                "format": "uri-reference"
            },
            "$schema": {
                "type": "string",
                "format": "uri"
            },
            "$ref": {
                "type": "string",
                "format": "uri-reference"
            },
            "$comment": {
                "type": "string"
            },
            "title": {
                "type": "string"
            },
            "description": {
                "type": "string"
            },
            "default": true,
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
            "maxLength": { "$ref": "#/definitions/nonNegativeInteger" },
            "minLength": { "$ref": "#/definitions/nonNegativeIntegerDefault0" },
            "pattern": {
                "type": "string",
                "format": "regex"
            },
            "additionalItems": { "$ref": "#" },
            "items": {
                "anyOf": [
                    { "$ref": "#" },
                    { "$ref": "#/definitions/schemaArray" }
                ],
                "default": true
            },
            "maxItems": { "$ref": "#/definitions/nonNegativeInteger" },
            "minItems": { "$ref": "#/definitions/nonNegativeIntegerDefault0" },
            "uniqueItems": {
                "type": "boolean",
                "default": false
            },
            "contains": { "$ref": "#" },
            "maxProperties": { "$ref": "#/definitions/nonNegativeInteger" },
            "minProperties": { "$ref": "#/definitions/nonNegativeIntegerDefault0" },
            "required": { "$ref": "#/definitions/stringArray" },
            "additionalProperties": { "$ref": "#" },
            "definitions": {
                "type": "object",
                "additionalProperties": { "$ref": "#" },
                "default": {}
            },
            "properties": {
                "type": "object",
                "additionalProperties": { "$ref": "#" },
                "default": {}
            },
            "patternProperties": {
                "type": "object",
                "additionalProperties": { "$ref": "#" },
                "propertyNames": { "format": "regex" },
                "default": {}
            },
            "dependencies": {
                "type": "object",
                "additionalProperties": {
                    "anyOf": [
                        { "$ref": "#" },
                        { "$ref": "#/definitions/stringArray" }
                    ]
                }
            },
            "propertyNames": { "$ref": "#" },
            "const": true,
            "enum": {
                "type": "array",
                "items": true,
                "minItems": 1,
                "uniqueItems": true
            },
            "type": {
                "anyOf": [
                    { "$ref": "#/definitions/simpleTypes" },
                    {
                        "type": "array",
                        "items": { "$ref": "#/definitions/simpleTypes" },
                        "minItems": 1,
                        "uniqueItems": true
                    }
                ]
            },
            "format": { "type": "string" },
            "contentMediaType": { "type": "string" },
            "contentEncoding": { "type": "string" },
            "if": { "$ref": "#" },
            "then": { "$ref": "#" },
            "else": { "$ref": "#" },
            "allOf": { "$ref": "#/definitions/schemaArray" },
            "anyOf": { "$ref": "#/definitions/schemaArray" },
            "oneOf": { "$ref": "#/definitions/schemaArray" },
            "not": { "$ref": "#" }
        },
        "default": true
    }
);

metaschema!(
    [JSON Hyper Schema 07]("http://json-schema.org/draft-07/hyper-schema#")
    {
        "$schema": "http://json-schema.org/draft-07/hyper-schema#",
        "$id": "http://json-schema.org/draft-07/hyper-schema#",
        "title": "JSON Hyper-Schema",
        "definitions": {
            "schemaArray": {
                "allOf": [
                    { "$ref": "http://json-schema.org/draft-07/schema#/definitions/schemaArray" },
                    {
                        "items": { "$ref": "#" }
                    }
                ]
            }
        },
        "allOf": [ { "$ref": "http://json-schema.org/draft-07/schema#" } ],
        "properties": {
            "additionalItems": { "$ref": "#" },
            "additionalProperties": { "$ref": "#"},
            "dependencies": {
                "additionalProperties": {
                    "anyOf": [
                        { "$ref": "#" },
                        { "type": "array" }
                    ]
                }
            },
            "items": {
                "anyOf": [
                    { "$ref": "#" },
                    { "$ref": "#/definitions/schemaArray" }
                ]
            },
            "definitions": {
                "additionalProperties": { "$ref": "#" }
            },
            "patternProperties": {
                "additionalProperties": { "$ref": "#" }
            },
            "properties": {
                "additionalProperties": { "$ref": "#" }
            },
            "if": { "$ref": "#" },
            "then": { "$ref": "#" },
            "else": { "$ref": "#" },
            "allOf": { "$ref": "#/definitions/schemaArray" },
            "anyOf": { "$ref": "#/definitions/schemaArray" },
            "oneOf": { "$ref": "#/definitions/schemaArray" },
            "not": { "$ref": "#" },
            "contains": { "$ref": "#" },
            "propertyNames": { "$ref": "#" },
            "base": {
                "type": "string",
                "format": "uri-template"
            },
            "links": {
                "type": "array",
                "items": {
                    "$ref": "http://json-schema.org/draft-07/links#"
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
    [JSON Hyper Schema Links 07]("http://json-schema.org/draft-07/links#")
    {
        "$schema": "http://json-schema.org/draft-07/hyper-schema#",
        "$id": "http://json-schema.org/draft-07/links#",
        "title": "Link Description Object",
        "allOf": [
            { "required": [ "rel", "href" ] },
            { "$ref": "#/definitions/noRequiredFields" }
        ],
        "definitions": {
            "noRequiredFields": {
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
                        "type": "string"
                    },
                    "href": {
                        "type": "string",
                        "format": "uri-template"
                    },
                    "hrefSchema": {
                        "$ref": "http://json-schema.org/draft-07/hyper-schema#"
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
                        "$ref": "http://json-schema.org/draft-07/hyper-schema#"
                    },
                    "targetMediaType": {
                        "type": "string"
                    },
                    "targetHints": { },
                    "headerSchema": {
                        "$ref": "http://json-schema.org/draft-07/hyper-schema#"
                    },
                    "submissionMediaType": {
                        "type": "string",
                        "default": "application/json"
                    },
                    "submissionSchema": {
                        "$ref": "http://json-schema.org/draft-07/hyper-schema#"
                    },
                    "$comment": {
                        "type": "string"
                    }
                }
            }
        }
    }
);
