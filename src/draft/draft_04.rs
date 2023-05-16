use crate::Uri;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

/// Returns the [Uri] of Schema Draft 04.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn schema_04_uri() -> &'static Uri {
    Lazy::get(&SCHEMA_04_URI).unwrap()
}

/// Returns `true` if the given [Uri] is the [Uri] of Schema Draft 04.
#[must_use]
pub fn is_schema_04_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == schema_04_uri()
}

/// Returns the [Uri] of Hyper Schema Draft 04.
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn hyper_schema_04_uri() -> &'static Uri {
    Lazy::get(&HYPER_SCHEMA_04_URI).unwrap()
}

/// Returns `true` if the given [Uri] is the [Uri] of Hyper Schema Draft 04.
#[must_use]
pub fn is_hyper_schema_04_uri(meta_schema_id: &Uri) -> bool {
    meta_schema_id == hyper_schema_04_uri()
}
#[must_use]
#[allow(clippy::missing_panics_doc)]
/// Returns a slice of the JSON Schema Draft 04 meta-schemas.
pub fn schema_04() -> &'static [Value] {
    Lazy::get(&SCHEMA_04).unwrap()
}
/// Returns a slice of the JSON Hyper Schema Draft 04 meta-schemas.
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn hyper_schema_04() -> &'static Vec<Value> {
    Lazy::get(&HYPER_SCHEMA_04).unwrap()
}

/// [Uri] of Schema Draft 04.
pub static SCHEMA_04_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-04/schema#").unwrap());

/// [Uri] of Hyper Schema Draft 04.
pub static HYPER_SCHEMA_04_URI: Lazy<Uri> =
    Lazy::new(|| Uri::parse("http://json-schema.org/draft-04/hyper-schema#").unwrap());

/// JSON Schema Draft 04 meta-schemas.
pub static SCHEMA_04: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![json!({
        "id": "http://json-schema.org/draft-04/schema#",
        "$schema": "http://json-schema.org/draft-04/schema#",
        "description": "Core schema meta-schema",
        "definitions": {
            "schemaArray": {
                "type": "array",
                "minItems": 1,
                "items": { "$ref": "#" }
            },
            "positiveInteger": {
                "type": "integer",
                "minimum": 0
            },
            "positiveIntegerDefault0": {
                "allOf": [ { "$ref": "#/definitions/positiveInteger" }, { "default": 0 } ]
            },
            "simpleTypes": {
                "enum": [ "array", "boolean", "integer", "null", "number", "object", "string" ]
            },
            "stringArray": {
                "type": "array",
                "items": { "type": "string" },
                "minItems": 1,
                "uniqueItems": true
            }
        },
        "type": "object",
        "properties": {
            "id": {
                "type": "string"
            },
            "$schema": {
                "type": "string"
            },
            "title": {
                "type": "string"
            },
            "description": {
                "type": "string"
            },
            "default": {},
            "multipleOf": {
                "type": "number",
                "minimum": 0,
                "exclusiveMinimum": true
            },
            "maximum": {
                "type": "number"
            },
            "exclusiveMaximum": {
                "type": "boolean",
                "default": false
            },
            "minimum": {
                "type": "number"
            },
            "exclusiveMinimum": {
                "type": "boolean",
                "default": false
            },
            "maxLength": { "$ref": "#/definitions/positiveInteger" },
            "minLength": { "$ref": "#/definitions/positiveIntegerDefault0" },
            "pattern": {
                "type": "string",
                "format": "regex"
            },
            "additionalItems": {
                "anyOf": [
                    { "type": "boolean" },
                    { "$ref": "#" }
                ],
                "default": {}
            },
            "items": {
                "anyOf": [
                    { "$ref": "#" },
                    { "$ref": "#/definitions/schemaArray" }
                ],
                "default": {}
            },
            "maxItems": { "$ref": "#/definitions/positiveInteger" },
            "minItems": { "$ref": "#/definitions/positiveIntegerDefault0" },
            "uniqueItems": {
                "type": "boolean",
                "default": false
            },
            "maxProperties": { "$ref": "#/definitions/positiveInteger" },
            "minProperties": { "$ref": "#/definitions/positiveIntegerDefault0" },
            "required": { "$ref": "#/definitions/stringArray" },
            "additionalProperties": {
                "anyOf": [
                    { "type": "boolean" },
                    { "$ref": "#" }
                ],
                "default": {}
            },
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
            "enum": {
                "type": "array",
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
            "allOf": { "$ref": "#/definitions/schemaArray" },
            "anyOf": { "$ref": "#/definitions/schemaArray" },
            "oneOf": { "$ref": "#/definitions/schemaArray" },
            "not": { "$ref": "#" }
        },
        "dependencies": {
            "exclusiveMaximum": [ "maximum" ],
            "exclusiveMinimum": [ "minimum" ]
        },
        "default": {}
    }
    )]
});


/// JSON Hyper Schema Draft 04 meta-schemas.
pub static HYPER_SCHEMA_04: Lazy<Vec<Value>> = Lazy::new(|| {
    vec![
        json!({
            "$schema" : "http://json-schema.org/draft-04/hyper-schema#",
            "id" : "http://json-schema.org/draft-04/links#",
            "type" : "object",

            "properties" : {
                "rel" : {
                    "type" : "string"
                },

                "href" : {
                    "type" : "string"
                },

                "template" : {
                    "type" : "string"
                },

                "targetSchema" : {"$ref" : "http://json-schema.org/draft-04/hyper-schema#"},

                "method" : {
                    "type" : "string",
                    "default" : "GET"
                },

                "enctype" : {
                    "type" : "string"
                },

                "properties" : {
                    "type" : "object",
                    "additionalProperties" : {"$ref" : "http://json-schema.org/draft-04/hyper-schema#"}
                }
            },

            "required" : ["rel", "href"],

            "dependencies" : {
                "enctype" : "method"
            }
        }
        ),
        json!({
            "$schema": "http://json-schema.org/draft-04/hyper-schema#",
            "id": "http://json-schema.org/draft-04/hyper-schema#",
            "title": "JSON Hyper-Schema",
            "allOf": [
                {"$ref": "http://json-schema.org/draft-04/schema#"}
            ],
            "properties": {
                "additionalItems": {
                    "anyOf": [
                        {"type": "boolean"},
                        {"$ref": "#"}
                    ]
                },
                "additionalProperties": {
                    "anyOf": [
                        {"type": "boolean"},
                        {"$ref": "#"}
                    ]
                },
                "dependencies": {
                    "additionalProperties": {
                        "anyOf": [
                            {"$ref": "#"},
                            {"type": "array"}
                        ]
                    }
                },
                "items": {
                    "anyOf": [
                        {"$ref": "#"},
                        {"$ref": "#/definitions/schemaArray"}
                    ]
                },
                "definitions": {
                    "additionalProperties": {"$ref": "#"}
                },
                "patternProperties": {
                    "additionalProperties": {"$ref": "#"}
                },
                "properties": {
                    "additionalProperties": {"$ref": "#"}
                },
                "allOf": {"$ref": "#/definitions/schemaArray"},
                "anyOf": {"$ref": "#/definitions/schemaArray"},
                "oneOf": {"$ref": "#/definitions/schemaArray"},
                "not": { "$ref": "#" },

                "links": {
                    "type": "array",
                    "items": {"$ref": "#/definitions/linkDescription"}
                },
                "fragmentResolution": {
                    "type": "string"
                },
                "media": {
                    "type": "object",
                    "properties": {
                        "type": {
                            "description": "A media type, as described in RFC 2046",
                            "type": "string"
                        },
                        "binaryEncoding": {
                            "description": "A content encoding scheme, as described in RFC 2045",
                            "type": "string"
                        }
                    }
                },
                "pathStart": {
                    "description": "Instances' URIs must start with this value for this schema to apply to them",
                    "type": "string",
                    "format": "uri"
                }
            },
            "definitions": {
                "schemaArray": {
                    "type": "array",
                    "items": {"$ref": "#"}
                },
                "linkDescription": {
                    "title": "Link Description Object",
                    "type": "object",
                    "required": ["href", "rel"],
                    "properties": {
                        "href": {
                            "description": "a URI template, as defined by RFC 6570, with the addition of the $, ( and ) characters for pre-processing",
                            "type": "string"
                        },
                        "rel": {
                            "description": "relation to the target resource of the link",
                            "type": "string"
                        },
                        "title": {
                            "description": "a title for the link",
                            "type": "string"
                        },
                        "targetSchema": {
                            "description": "JSON Schema describing the link target",
                            "$ref": "#"
                        },
                        "mediaType": {
                            "description": "media type (as defined by RFC 2046) describing the link target",
                            "type": "string"
                        },
                        "method": {
                            "description": "method for requesting the target of the link (e.g. for HTTP this might be \"GET\" or \"DELETE\")",
                            "type": "string"
                        },
                        "encType": {
                            "description": "The media type in which to submit data along with the request",
                            "type": "string",
                            "default": "application/json"
                        },
                        "schema": {
                            "description": "Schema describing the data to submit along with the request",
                            "$ref": "#"
                        }
                    }
                },
                "readOnly": {
                    "description": "If true, indicates that the value of this property is controlled by the server.",
                    "type": "boolean",
                    "default": false
                }
            },
            "links": [
                {
                    "rel": "self",
                    "href": "{+id}"
                },
                {
                    "rel": "full",
                    "href": "{+($ref)}"
                }
            ]
        }
        ),
    ]
});
