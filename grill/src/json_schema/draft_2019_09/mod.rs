use serde_json::Value;

use crate::json_schema::metaschema;

#[must_use]
pub fn dialect() -> &'static crate::schema::Dialect {
    todo!()
}

#[must_use]
pub fn schema() -> &'static Value {
    json_schema_2019_09_value()
}

metaschema!(
    [JSON Schema 2019_09]("https://json-schema.org/draft/2019-09/schema")
    {
        "$schema": "https://json-schema.org/draft/2019-09/schema",
        "$id": "https://json-schema.org/draft/2019-09/schema",
        "$vocabulary": {
            "https://json-schema.org/draft/2019-09/vocab/core": true,
            "https://json-schema.org/draft/2019-09/vocab/applicator": true,
            "https://json-schema.org/draft/2019-09/vocab/validation": true,
            "https://json-schema.org/draft/2019-09/vocab/meta-data": true,
            "https://json-schema.org/draft/2019-09/vocab/format": false,
            "https://json-schema.org/draft/2019-09/vocab/content": true
        },
        "$recursiveAnchor": true,

        "title": "Core and Validation specifications meta-schema",
        "allOf": [
            {"$ref": "meta/core"},
            {"$ref": "meta/applicator"},
            {"$ref": "meta/validation"},
            {"$ref": "meta/meta-data"},
            {"$ref": "meta/format"},
            {"$ref": "meta/content"}
        ],
        "type": ["object", "boolean"],
        "properties": {
            "definitions": {
                "$comment": "While no longer an official keyword as it is replaced by $defs, this keyword is retained in the meta-schema to prevent incompatible extensions as it remains in common use.",
                "type": "object",
                "additionalProperties": { "$recursiveRef": "#" },
                "default": {}
            },
            "dependencies": {
                "$comment": "\"dependencies\" is no longer a keyword, but schema authors should avoid redefining it to facilitate a smooth transition to \"dependentSchemas\" and \"dependentRequired\"",
                "type": "object",
                "additionalProperties": {
                    "anyOf": [
                        { "$recursiveRef": "#" },
                        { "$ref": "meta/validation#/$defs/stringArray" }
                    ]
                }
            }
        }
    }
);

metaschema!(
    [JSON Hyper Schema 2019_09]("https://json-schema.org/draft/2019-09/hyper-schema")
    {
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
);

metaschema!(
    [JSON Hyper Schema Links 2019_09]("https://json-schema.org/draft/2019-09/links")
    {
        "$schema": "https://json-schema.org/draft/2019-09/schema",
        "$id": "https://json-schema.org/draft/2019-09/links",
        "title": "Link Description Object",
        "allOf": [
            { "required": ["rel", "href"] },
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
                    "targetHints": {},
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
);

metaschema!(
    [JSON Schema 2019_09 Content]("https://json-schema.org/draft/2019-09/meta/content")
    {
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

);

metaschema!(
    [JSON Schema 2019_09 Metadata]("https://json-schema.org/draft/2019-09/meta/meta-data")
    {
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
);

metaschema!(
    [JSON Schema 2019_09 Core]("https://json-schema.org/draft/2019-09/meta/core")
    {
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
);

metaschema!(
    [JSON Schema 2019_09 Format]("https://json-schema.org/draft/2019-09/meta/format")
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
);

metaschema!(
    [JSON Schema 2019_09 Validation]("https://json-schema.org/draft/2019-09/meta/validation")
    {
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

);

metaschema!(
    [JSON Schema 2019_09 Applicator]("https://json-schema.org/draft/2019-09/meta/applicator")
    {
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

);
