use grill::Interrogator;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

pub(super) trait Draft202012: Copy {
    fn interrogator(self) -> Interrogator;
}

fn base_interrogator() -> Interrogator {
    static INTERROGATOR: Lazy<Interrogator> = Lazy::new(|| 
        super::draft_202012().interrogator()
    );
    INTERROGATOR.clone()
}

fn runner() -> impl Draft202012 {
    super::draft_202012()
}

fn sources() -> &'static [(&'static str, Value)] {
    static VALUES: Lazy<Vec<(&'static str, Value)>> = Lazy::new(|| {
        Vec::from(
            [
                (
                    "http://localhost:1234/baseUriChange/folderInteger.json",
                    json ! ({ "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/baseUriChangeFolder/folderInteger.json",
                    json ! ({ "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/baseUriChangeFolderInSubschema/folderInteger.json",
                    json ! ({ "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/different-id-ref-string.json",
                    json ! ({ "$id" : "http://localhost:1234/real-id-ref-string.json" , "$defs" : { "bar" : { "type" : "string" } } , "$ref" : "#/$defs/bar" }),
                ),
                (
                    "http://localhost:1234/draft-next/baseUriChange/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft-next/baseUriChangeFolder/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft-next/baseUriChangeFolderInSubschema/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft-next/extendible-dynamic-ref.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "description" : "extendible array" , "$id" : "http://localhost:1234/draft-next/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
                ),
                (
                    "http://localhost:1234/draft-next/format-assertion-false.json",
                    json ! ({ "$id" : "http://localhost:1234/draft-next/format-assertion-false.json" , "$schema" : "https://json-schema.org/draft/next/schema" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/core" : true , "https://json-schema.org/draft/next/vocab/format-assertion" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/core" } , { "$ref" : "https://json-schema.org/draft/next/meta/format-assertion" }] }),
                ),
                (
                    "http://localhost:1234/draft-next/format-assertion-true.json",
                    json ! ({ "$id" : "http://localhost:1234/draft-next/format-assertion-true.json" , "$schema" : "https://json-schema.org/draft/next/schema" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/core" : true , "https://json-schema.org/draft/next/vocab/format-assertion" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/core" } , { "$ref" : "https://json-schema.org/draft/next/meta/format-assertion" }] }),
                ),
                (
                    "http://localhost:1234/draft-next/integer.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft-next/locationIndependentIdentifier.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
                ),
                (
                    "http://localhost:1234/draft-next/metaschema-no-validation.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$id" : "http://localhost:1234/draft-next/metaschema-no-validation.json" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/applicator" : true , "https://json-schema.org/draft/next/vocab/core" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/applicator" } , { "$ref" : "https://json-schema.org/draft/next/meta/core" }] }),
                ),
                (
                    "http://localhost:1234/draft-next/metaschema-optional-vocabulary.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$id" : "http://localhost:1234/draft-next/metaschema-optional-vocabulary.json" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/validation" : true , "https://json-schema.org/draft/next/vocab/core" : true , "http://localhost:1234/draft/next/vocab/custom" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/validation" } , { "$ref" : "https://json-schema.org/draft/next/meta/core" }] }),
                ),
                (
                    "http://localhost:1234/draft-next/name-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/draft-next/nested/foo-ref-string.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
                ),
                (
                    "http://localhost:1234/draft-next/nested/string.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/draft-next/ref-and-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$id" : "http://localhost:1234/draft-next/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
                ),
                (
                    "http://localhost:1234/draft-next/subSchemas-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
                ),
                (
                    "http://localhost:1234/draft-next/subSchemas.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
                ),
                (
                    "http://localhost:1234/draft-next/tree.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "description" : "tree schema, extensible" , "$id" : "http://localhost:1234/draft-next/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/baseUriChange/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/baseUriChangeFolder/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/baseUriChangeFolderInSubschema/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/dependentRequired.json",
                    json ! ({ "$id" : "http://localhost:1234/draft2019-09/dependentRequired.json" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "dependentRequired" : { "foo" : ["bar"] } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/extendible-dynamic-ref.json",
                    json ! ({ "description" : "extendible array" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/ignore-prefixItems.json",
                    json ! ({ "$id" : "http://localhost:1234/draft2019-09/ignore-prefixItems.json" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "prefixItems" : [{ "type" : "string" }] }),
                ),
                (
                    "http://localhost:1234/draft2019-09/integer.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/locationIndependentIdentifier.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/metaschema-no-validation.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/metaschema-no-validation.json" , "$vocabulary" : { "https://json-schema.org/draft/2019-09/vocab/applicator" : true , "https://json-schema.org/draft/2019-09/vocab/core" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2019-09/meta/applicator" } , { "$ref" : "https://json-schema.org/draft/2019-09/meta/core" }] }),
                ),
                (
                    "http://localhost:1234/draft2019-09/metaschema-optional-vocabulary.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/metaschema-optional-vocabulary.json" , "$vocabulary" : { "https://json-schema.org/draft/2019-09/vocab/validation" : true , "https://json-schema.org/draft/2019-09/vocab/core" : true , "http://localhost:1234/draft/2019-09/vocab/custom" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2019-09/meta/validation" } , { "$ref" : "https://json-schema.org/draft/2019-09/meta/core" }] }),
                ),
                (
                    "http://localhost:1234/draft2019-09/name-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/nested/foo-ref-string.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/nested/string.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/ref-and-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
                ),
                (
                    "http://localhost:1234/draft2019-09/subSchemas-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/subSchemas.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
                ),
                (
                    "http://localhost:1234/draft2019-09/tree.json",
                    json ! ({ "description" : "tree schema, extensible" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
                ),
                (
                    "http://localhost:1234/draft2020-12/baseUriChange/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/baseUriChangeFolder/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/baseUriChangeFolderInSubschema/folderInteger.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/extendible-dynamic-ref.json",
                    json ! ({ "description" : "extendible array" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
                ),
                (
                    "http://localhost:1234/draft2020-12/format-assertion-false.json",
                    json ! ({ "$id" : "http://localhost:1234/draft2020-12/format-assertion-false.json" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/core" : true , "https://json-schema.org/draft/2020-12/vocab/format-assertion" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/core" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/format-assertion" }] }),
                ),
                (
                    "http://localhost:1234/draft2020-12/format-assertion-true.json",
                    json ! ({ "$id" : "http://localhost:1234/draft2020-12/format-assertion-true.json" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/core" : true , "https://json-schema.org/draft/2020-12/vocab/format-assertion" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/core" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/format-assertion" }] }),
                ),
                (
                    "http://localhost:1234/draft2020-12/integer.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/locationIndependentIdentifier.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
                ),
                (
                    "http://localhost:1234/draft2020-12/metaschema-no-validation.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/metaschema-no-validation.json" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/applicator" : true , "https://json-schema.org/draft/2020-12/vocab/core" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/applicator" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/core" }] }),
                ),
                (
                    "http://localhost:1234/draft2020-12/metaschema-optional-vocabulary.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/metaschema-optional-vocabulary.json" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/validation" : true , "https://json-schema.org/draft/2020-12/vocab/core" : true , "http://localhost:1234/draft/2020-12/vocab/custom" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/validation" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/core" }] }),
                ),
                (
                    "http://localhost:1234/draft2020-12/name-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/nested/foo-ref-string.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
                ),
                (
                    "http://localhost:1234/draft2020-12/nested/string.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/prefixItems.json",
                    json ! ({ "$id" : "http://localhost:1234/draft2020-12/prefixItems.json" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "prefixItems" : [{ "type" : "string" }] }),
                ),
                (
                    "http://localhost:1234/draft2020-12/ref-and-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
                ),
                (
                    "http://localhost:1234/draft2020-12/subSchemas-defs.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
                ),
                (
                    "http://localhost:1234/draft2020-12/subSchemas.json",
                    json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
                ),
                (
                    "http://localhost:1234/draft2020-12/tree.json",
                    json ! ({ "description" : "tree schema, extensible" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
                ),
                (
                    "http://localhost:1234/draft7/ignore-dependentRequired.json",
                    json ! ({ "$id" : "http://localhost:1234/draft7/integer.json" , "$schema" : "http://json-schema.org/draft-07/schema#" , "dependentRequired" : { "foo" : ["bar"] } }),
                ),
                (
                    "http://localhost:1234/extendible-dynamic-ref.json",
                    json ! ({ "description" : "extendible array" , "$id" : "http://localhost:1234/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
                ),
                (
                    "http://localhost:1234/integer.json",
                    json ! ({ "type" : "integer" }),
                ),
                (
                    "http://localhost:1234/locationIndependentIdentifier.json",
                    json ! ({ "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
                ),
                (
                    "http://localhost:1234/locationIndependentIdentifierDraft4.json",
                    json ! ({ "definitions" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "id" : "#foo" , "type" : "integer" } } }),
                ),
                (
                    "http://localhost:1234/locationIndependentIdentifierPre2019.json",
                    json ! ({ "definitions" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$id" : "#foo" , "type" : "integer" } } }),
                ),
                (
                    "http://localhost:1234/name-defs.json",
                    json ! ({ "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/name.json",
                    json ! ({ "definitions" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
                ),
                (
                    "http://localhost:1234/nested/foo-ref-string.json",
                    json ! ({ "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
                ),
                (
                    "http://localhost:1234/nested/string.json",
                    json ! ({ "type" : "string" }),
                ),
                (
                    "http://localhost:1234/nested-absolute-ref-to-string.json",
                    json ! ({ "$defs" : { "bar" : { "$id" : "http://localhost:1234/the-nested-id.json" , "type" : "string" } } , "$ref" : "http://localhost:1234/the-nested-id.json" }),
                ),
                (
                    "http://localhost:1234/ref-and-definitions.json",
                    json ! ({ "$id" : "http://localhost:1234/ref-and-definitions.json" , "definitions" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "allOf" : [{ "$ref" : "#/definitions/inner" }] }),
                ),
                (
                    "http://localhost:1234/ref-and-defs.json",
                    json ! ({ "$id" : "http://localhost:1234/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
                ),
                (
                    "http://localhost:1234/subSchemas-defs.json",
                    json ! ({ "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
                ),
                (
                    "http://localhost:1234/subSchemas.json",
                    json ! ({ "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
                ),
                (
                    "http://localhost:1234/tree.json",
                    json ! ({ "description" : "tree schema, extensible" , "$id" : "http://localhost:1234/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
                ),
                (
                    "http://localhost:1234/urn-ref-string.json",
                    json ! ({ "$id" : "urn:uuid:feebdaed-ffff-0000-ffff-0000deadbeef" , "$defs" : { "bar" : { "type" : "string" } } , "$ref" : "#/$defs/bar" }),
                ),
            ],
        )
    });
    &VALUES
}
