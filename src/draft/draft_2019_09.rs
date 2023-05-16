use crate::Uri;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

#[must_use]
#[allow(clippy::missing_panics_doc)]
/// Returns the [Uri] of Schema Draft 2019-09.
pub fn schema_2019_09_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_2019_09_URI).unwrap()
}

/// Returns `true` if the given [Uri] is the [Uri] of Schema Draft 2019-09.
#[must_use]
pub fn is_schema_2019_09_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_2019_09_uri()
}

/// Returns the [Uri] of Hyper Schema Draft 2019-09.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn hyper_schema_2019_09_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_2019_09_URI).unwrap()
}

/// Returns `true` if the given [Uri] is the [Uri] of Hyper Schema Draft 2019-09.
#[must_use]
pub fn is_hyper_schema_2019_09_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_2019_09_uri()
}

/// Returns Meta Schemas for Draft 2019-09.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_2019_09() -> &'static Vec<Value> {
    Lazy::get(&SCHEMA_2019_09).unwrap()
}

/// Returns Meta Schemas for Hyper Schema Draft 2019-09.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn hyper_schema_2019_09() -> &'static Vec<Value> {
    Lazy::get(&HYPER_SCHEMA_2019_09).unwrap()
}

/// [Uri] of Schema Draft 2019-09.
pub static SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2019-09/schema").unwrap());

/// [Uri] of Hyper Schema Draft 2019-09.
pub static HYPER_SCHEMA_2019_09_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2019-09/hyper-schema").unwrap());

pub static SCHEMA_2019_09: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://json-schema.org/draft/2019-09/meta/applicator",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/applicator": true
            },
            "$recursiveAnchor": true,

            "title": "Applicator vocabulary meta-schema",
            "type": ["object", "boolean"],
            "properties": {
                "additionalItems": { "$recursiveRef": "#" },
                "unevaluatedItems": { "$recursiveRef": "#" },
                "items": {
                    "anyOf": [
                        { "$recursiveRef": "#" },
                        { "$ref": "#/$defs/schemaArray" }
                    ]
                },
                "contains": { "$recursiveRef": "#" },
                "additionalProperties": { "$recursiveRef": "#" },
                "unevaluatedProperties": { "$recursiveRef": "#" },
                "properties": {
                    "type": "object",
                    "additionalProperties": { "$recursiveRef": "#" },
                    "default": {}
                },
                "patternProperties": {
                    "type": "object",
                    "additionalProperties": { "$recursiveRef": "#" },
                    "propertyNames": { "format": "regex" },
                    "default": {}
                },
                "dependentSchemas": {
                    "type": "object",
                    "additionalProperties": {
                        "$recursiveRef": "#"
                    }
                },
                "propertyNames": { "$recursiveRef": "#" },
                "if": { "$recursiveRef": "#" },
                "then": { "$recursiveRef": "#" },
                "else": { "$recursiveRef": "#" },
                "allOf": { "$ref": "#/$defs/schemaArray" },
                "anyOf": { "$ref": "#/$defs/schemaArray" },
                "oneOf": { "$ref": "#/$defs/schemaArray" },
                "not": { "$recursiveRef": "#" }
            },
            "$defs": {
                "schemaArray": {
                    "type": "array",
                    "minItems": 1,
                    "items": { "$recursiveRef": "#" }
                }
            }
        }
        ),
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://json-schema.org/draft/2019-09/meta/content",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/content": true
            },
            "$recursiveAnchor": true,

            "title": "Content vocabulary meta-schema",

            "type": ["object", "boolean"],
            "properties": {
                "contentMediaType": { "type": "string" },
                "contentEncoding": { "type": "string" },
                "contentSchema": { "$recursiveRef": "#" }
            }
        }
        ),
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://json-schema.org/draft/2019-09/meta/core",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/core": true
            },
            "$recursiveAnchor": true,

            "title": "Core vocabulary meta-schema",
            "type": ["object", "boolean"],
            "properties": {
                "$id": {
                    "type": "string",
                    "format": "uri-reference",
                    "$comment": "Non-empty fragments not allowed.",
                    "pattern": "^[^#]*#?$"
                },
                "$schema": {
                    "type": "string",
                    "format": "uri"
                },
                "$anchor": {
                    "type": "string",
                    "pattern": "^[A-Za-z][-A-Za-z0-9.:_]*$"
                },
                "$ref": {
                    "type": "string",
                    "format": "uri-reference"
                },
                "$recursiveRef": {
                    "type": "string",
                    "format": "uri-reference"
                },
                "$recursiveAnchor": {
                    "type": "boolean",
                    "default": false
                },
                "$vocabulary": {
                    "type": "object",
                    "propertyNames": {
                        "type": "string",
                        "format": "uri"
                    },
                    "additionalProperties": {
                        "type": "boolean"
                    }
                },
                "$comment": {
                    "type": "string"
                },
                "$defs": {
                    "type": "object",
                    "additionalProperties": { "$recursiveRef": "#" },
                    "default": {}
                }
            }
        }
        ),
        json!(
            {
                "$schema": "https://json-schema.org/draft/2019-09/schema",
                "$id": "https://json-schema.org/draft/2019-09/meta/format",
                "$vocabulary": {
                    "https://json-schema.org/draft/2019-09/vocab/format": true
                },
                "$recursiveAnchor": true,

                "title": "Format vocabulary meta-schema",
                "type": ["object", "boolean"],
                "properties": {
                    "format": { "type": "string" }
                }
            }

        ),
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://json-schema.org/draft/2019-09/meta/meta-data",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/meta-data": true
            },
            "$recursiveAnchor": true,

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
        ),
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/schema",
            "$id": "https://json-schema.org/draft/2019-09/meta/validation",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/validation": true
            },
            "$recursiveAnchor": true,

            "title": "Validation vocabulary meta-schema",
            "type": ["object", "boolean"],
            "properties": {
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
                },
                "const": true,
                "enum": {
                    "type": "array",
                    "items": true
                },
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
        ),
    ]
});
pub static HYPER_SCHEMA_2019_09: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/hyper-schema",
            "$id": "https://json-schema.org/draft/2019-09/meta/hyper-schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/hyper-schema": true
            },
            "$recursiveAnchor": true,

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
                        "$ref": "https://json-schema.org/draft/2019-09/links"
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
        ),
        json!(
            {
                "$schema": "https://json-schema.org/draft/2019-09/schema",
                "$id": "https://json-schema.org/draft/2019-09/links",
                "title": "Link Description Object",
                "allOf": [
                    { "required": [ "rel", "href" ] },
                    { "$ref": "#/$defs/noRequiredFields" }
                ],
                "$defs": {
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
                                "$ref": "https://json-schema.org/draft/2019-09/hyper-schema",
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
                                "$ref": "https://json-schema.org/draft/2019-09/hyper-schema",
                                "default": true
                            },
                            "targetMediaType": {
                                "type": "string"
                            },
                            "targetHints": { },
                            "headerSchema": {
                                "$ref": "https://json-schema.org/draft/2019-09/hyper-schema",
                                "default": true
                            },
                            "submissionMediaType": {
                                "type": "string",
                                "default": "application/json"
                            },
                            "submissionSchema": {
                                "$ref": "https://json-schema.org/draft/2019-09/hyper-schema",
                                "default": true
                            },
                            "$comment": {
                                "type": "string"
                            }
                        }
                    }
                }
            }
        ),
        json!({
            "$schema": "https://json-schema.org/draft/2019-09/hyper-schema",
            "$id": "https://json-schema.org/draft/2019-09/hyper-schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2019-09/vocab/core": true,
                "https://json-schema.org/draft/2019-09/vocab/applicator": true,
                "https://json-schema.org/draft/2019-09/vocab/validation": true,
                "https://json-schema.org/draft/2019-09/vocab/meta-data": true,
                "https://json-schema.org/draft/2019-09/vocab/format": false,
                "https://json-schema.org/draft/2019-09/vocab/content": true,
                "https://json-schema.org/draft/2019-09/vocab/hyper-schema": true
            },
            "$recursiveAnchor": true,

            "title": "JSON Hyper-Schema",
            "allOf": [
                {"$ref": "https://json-schema.org/draft/2019-09/schema"},
                {"$ref": "https://json-schema.org/draft/2019-09/meta/hyper-schema"}
            ],
            "links": [
                {
                    "rel": "self",
                    "href": "{+%24id}"
                }
            ]
        }
        ),
    ]
});
