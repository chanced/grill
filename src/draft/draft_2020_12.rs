use crate::Uri;
use once_cell::sync::Lazy;
use serde_json::{json, Value};
pub fn schema_2020_12_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_2020_12_URI).unwrap()
}

/// Returns `true` if the given [Uri] is the [Uri] of Schema Draft 04.
pub fn is_schema_2020_12_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_2020_12_uri()
}

/// Returns the [Uri] of Hyper Schema Draft 04.
pub fn hyper_schema_2020_12_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_2020_12_URI).unwrap()
}

/// Returns `true` if the given [Uri] is the [Uri] of Hyper Schema Draft 04.
pub fn is_hyper_schema_2020_12_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_2020_12_uri()
}

pub fn schema_2020_12() -> &'static Vec<Value> {
    Lazy::get(&SCHEMA_2020_12).unwrap()
}

pub fn hyper_schema_2020_12() -> &'static Vec<Value> {
    Lazy::get(&HYPER_SCHEMA_2020_12).unwrap()
}

pub static SCHEMA_2020_12: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![
        json!({
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
        }),
        json!({
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
        ),
        json!({
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
        ),
        json!({
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
        ),
        json!({
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
        ),
        json!({
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
        ),
        json!({
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
        ),
        json!({
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
        ),
    ]
});

pub static HYPER_SCHEMA_2020_12: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![
        json!({
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
        ),
        json!({
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
            "required": ["rel", "href"]
        }
        ),
        json!({
            "$schema": "https://json-schema.org/draft/2020-12/hyper-schema",
            "$id": "https://json-schema.org/draft/2020-12/hyper-schema",
            "$vocabulary": {
                "https://json-schema.org/draft/2020-12/vocab/core": true,
                "https://json-schema.org/draft/2020-12/vocab/applicator": true,
                "https://json-schema.org/draft/2020-12/vocab/unevaluated": true,
                "https://json-schema.org/draft/2020-12/vocab/validation": true,
                "https://json-schema.org/draft/2020-12/vocab/meta-data": true,
                "https://json-schema.org/draft/2020-12/vocab/format-annotation": true,
                "https://json-schema.org/draft/2020-12/vocab/content": true,
                "https://json-schema.org/draft/2019-09/vocab/hyper-schema": true
            },
            "$dynamicAnchor": "meta",

            "title": "JSON Hyper-Schema",
            "allOf": [
                { "$ref": "https://json-schema.org/draft/2020-12/schema" },
                { "$ref": "https://json-schema.org/draft/2020-12/meta/hyper-schema" }
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

/// [Uri] of Schema Draft 2020-12.
pub static SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2020-12/schema").unwrap());

/// [Uri] of Hyper Schema Draft 2020-12.
pub static HYPER_SCHEMA_2020_12_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("https://json-schema.org/draft/2020-12/hyper-schema").unwrap());
