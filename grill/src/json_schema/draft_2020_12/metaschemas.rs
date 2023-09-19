use lazy_static::lazy_static;
use serde_json::{json, Value};

lazy_static! {
    pub static ref JSON_SCHEMA_2020_12_VALUE: Value =
        json!(
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
    /// [`Value`] of [JSON Schema 2020-12 Core](https://json-schema.org/draft/2020-12/json-schema-core.html#section-8)
    pub static ref JSON_SCHEMA_2020_12_CORE_VALUE: Value = json!(
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
    /// [`Value`] of [JSON Schema 2020-12 Applicator](https://json-schema.org/draft/2020-12/vocab/applicator)
    pub static ref JSON_SCHEMA_2020_12_APPLICATOR_VALUE: Value = json!(
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
    pub static ref JSON_SCHEMA_2020_12_CONTENT_VALUE: Value = json!(
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

    pub static ref JSON_SCHEMA_2020_12_FORMAT_ANNOTATION_VALUE:Value = json!(
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
}
