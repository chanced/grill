use once_cell::sync::Lazy;
use serde_json::{json, Value};
pub(super) fn sources() -> impl Iterator<Item = (&'static str, &'static Value)> {
    static VALUES: Lazy<[(&'static str, Value); 73]> = Lazy::new(|| {
        [
            source_base_uri_change_folder_integer_json(),
            source_base_uri_change_folder_folder_integer_json(),
            source_base_uri_change_folder_in_subschema_folder_integer_json(),
            source_different_id_ref_string_json(),
            source_draft_next_base_uri_change_folder_integer_json(),
            source_draft_next_base_uri_change_folder_folder_integer_json(),
            source_draft_next_base_uri_change_folder_in_subschema_folder_integer_json(),
            source_draft_next_extendible_dynamic_ref_json(),
            source_draft_next_format_assertion_false_json(),
            source_draft_next_format_assertion_true_json(),
            source_draft_next_integer_json(),
            source_draft_next_location_independent_identifier_json(),
            source_draft_next_metaschema_no_validation_json(),
            source_draft_next_metaschema_optional_vocabulary_json(),
            source_draft_next_name_defs_json(),
            source_draft_next_nested_foo_ref_string_json(),
            source_draft_next_nested_string_json(),
            source_draft_next_ref_and_defs_json(),
            source_draft_next_sub_schemas_defs_json(),
            source_draft_next_sub_schemas_json(),
            source_draft_next_tree_json(),
            source_draft2019_09_base_uri_change_folder_integer_json(),
            source_draft2019_09_base_uri_change_folder_folder_integer_json(),
            source_draft2019_09_base_uri_change_folder_in_subschema_folder_integer_json(),
            source_draft2019_09_dependent_required_json(),
            source_draft2019_09_extendible_dynamic_ref_json(),
            source_draft2019_09_ignore_prefix_items_json(),
            source_draft2019_09_integer_json(),
            source_draft2019_09_location_independent_identifier_json(),
            source_draft2019_09_metaschema_no_validation_json(),
            source_draft2019_09_metaschema_optional_vocabulary_json(),
            source_draft2019_09_name_defs_json(),
            source_draft2019_09_nested_foo_ref_string_json(),
            source_draft2019_09_nested_string_json(),
            source_draft2019_09_ref_and_defs_json(),
            source_draft2019_09_sub_schemas_defs_json(),
            source_draft2019_09_sub_schemas_json(),
            source_draft2019_09_tree_json(),
            source_draft2020_12_base_uri_change_folder_integer_json(),
            source_draft2020_12_base_uri_change_folder_folder_integer_json(),
            source_draft2020_12_base_uri_change_folder_in_subschema_folder_integer_json(),
            source_draft2020_12_extendible_dynamic_ref_json(),
            source_draft2020_12_format_assertion_false_json(),
            source_draft2020_12_format_assertion_true_json(),
            source_draft2020_12_integer_json(),
            source_draft2020_12_location_independent_identifier_json(),
            source_draft2020_12_metaschema_no_validation_json(),
            source_draft2020_12_metaschema_optional_vocabulary_json(),
            source_draft2020_12_name_defs_json(),
            source_draft2020_12_nested_foo_ref_string_json(),
            source_draft2020_12_nested_string_json(),
            source_draft2020_12_prefix_items_json(),
            source_draft2020_12_ref_and_defs_json(),
            source_draft2020_12_sub_schemas_defs_json(),
            source_draft2020_12_sub_schemas_json(),
            source_draft2020_12_tree_json(),
            source_draft7_ignore_dependent_required_json(),
            source_extendible_dynamic_ref_json(),
            source_integer_json(),
            source_location_independent_identifier_json(),
            source_location_independent_identifier_draft4_json(),
            source_location_independent_identifier_pre2019_json(),
            source_name_defs_json(),
            source_name_json(),
            source_nested_foo_ref_string_json(),
            source_nested_string_json(),
            source_nested_absolute_ref_to_string_json(),
            source_ref_and_definitions_json(),
            source_ref_and_defs_json(),
            source_sub_schemas_defs_json(),
            source_sub_schemas_json(),
            source_tree_json(),
            source_urn_ref_string_json(),
        ]
    });
    VALUES.iter().map(|(uri, schema)| (*uri, schema))
}
fn source_base_uri_change_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/baseUriChange/folderInteger.json",
        json ! ({ "type" : "integer" }),
    )
}
fn source_base_uri_change_folder_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/baseUriChangeFolder/folderInteger.json",
        json ! ({ "type" : "integer" }),
    )
}
fn source_base_uri_change_folder_in_subschema_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/baseUriChangeFolderInSubschema/folderInteger.json",
        json ! ({ "type" : "integer" }),
    )
}
fn source_different_id_ref_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/different-id-ref-string.json",
        json ! ({ "$id" : "http://localhost:1234/real-id-ref-string.json" , "$defs" : { "bar" : { "type" : "string" } } , "$ref" : "#/$defs/bar" }),
    )
}
fn source_draft_next_base_uri_change_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/baseUriChange/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
    )
}
fn source_draft_next_base_uri_change_folder_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/baseUriChangeFolder/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
    )
}
fn source_draft_next_base_uri_change_folder_in_subschema_folder_integer_json(
) -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/baseUriChangeFolderInSubschema/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
    )
}
fn source_draft_next_extendible_dynamic_ref_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/extendible-dynamic-ref.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "description" : "extendible array" , "$id" : "http://localhost:1234/draft-next/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
    )
}
fn source_draft_next_format_assertion_false_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/format-assertion-false.json",
        json ! ({ "$id" : "http://localhost:1234/draft-next/format-assertion-false.json" , "$schema" : "https://json-schema.org/draft/next/schema" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/core" : true , "https://json-schema.org/draft/next/vocab/format-assertion" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/core" } , { "$ref" : "https://json-schema.org/draft/next/meta/format-assertion" }] }),
    )
}
fn source_draft_next_format_assertion_true_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/format-assertion-true.json",
        json ! ({ "$id" : "http://localhost:1234/draft-next/format-assertion-true.json" , "$schema" : "https://json-schema.org/draft/next/schema" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/core" : true , "https://json-schema.org/draft/next/vocab/format-assertion" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/core" } , { "$ref" : "https://json-schema.org/draft/next/meta/format-assertion" }] }),
    )
}
fn source_draft_next_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/integer.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "integer" }),
    )
}
fn source_draft_next_location_independent_identifier_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/locationIndependentIdentifier.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
    )
}
fn source_draft_next_metaschema_no_validation_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/metaschema-no-validation.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$id" : "http://localhost:1234/draft-next/metaschema-no-validation.json" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/applicator" : true , "https://json-schema.org/draft/next/vocab/core" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/applicator" } , { "$ref" : "https://json-schema.org/draft/next/meta/core" }] }),
    )
}
fn source_draft_next_metaschema_optional_vocabulary_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/metaschema-optional-vocabulary.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$id" : "http://localhost:1234/draft-next/metaschema-optional-vocabulary.json" , "$vocabulary" : { "https://json-schema.org/draft/next/vocab/validation" : true , "https://json-schema.org/draft/next/vocab/core" : true , "http://localhost:1234/draft/next/vocab/custom" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/next/meta/validation" } , { "$ref" : "https://json-schema.org/draft/next/meta/core" }] }),
    )
}
fn source_draft_next_name_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/name-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
    )
}
fn source_draft_next_nested_foo_ref_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/nested/foo-ref-string.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
    )
}
fn source_draft_next_nested_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/nested/string.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "type" : "string" }),
    )
}
fn source_draft_next_ref_and_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/ref-and-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$id" : "http://localhost:1234/draft-next/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
    )
}
fn source_draft_next_sub_schemas_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/subSchemas-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
    )
}
fn source_draft_next_sub_schemas_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/subSchemas.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
    )
}
fn source_draft_next_tree_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft-next/tree.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/next/schema" , "description" : "tree schema, extensible" , "$id" : "http://localhost:1234/draft-next/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
    )
}
fn source_draft2019_09_base_uri_change_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/baseUriChange/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
    )
}
fn source_draft2019_09_base_uri_change_folder_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/baseUriChangeFolder/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
    )
}
fn source_draft2019_09_base_uri_change_folder_in_subschema_folder_integer_json(
) -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/baseUriChangeFolderInSubschema/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
    )
}
fn source_draft2019_09_dependent_required_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/dependentRequired.json",
        json ! ({ "$id" : "http://localhost:1234/draft2019-09/dependentRequired.json" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "dependentRequired" : { "foo" : ["bar"] } }),
    )
}
fn source_draft2019_09_extendible_dynamic_ref_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/extendible-dynamic-ref.json",
        json ! ({ "description" : "extendible array" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
    )
}
fn source_draft2019_09_ignore_prefix_items_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/ignore-prefixItems.json",
        json ! ({ "$id" : "http://localhost:1234/draft2019-09/ignore-prefixItems.json" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "prefixItems" : [{ "type" : "string" }] }),
    )
}
fn source_draft2019_09_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/integer.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "integer" }),
    )
}
fn source_draft2019_09_location_independent_identifier_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/locationIndependentIdentifier.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
    )
}
fn source_draft2019_09_metaschema_no_validation_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/metaschema-no-validation.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/metaschema-no-validation.json" , "$vocabulary" : { "https://json-schema.org/draft/2019-09/vocab/applicator" : true , "https://json-schema.org/draft/2019-09/vocab/core" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2019-09/meta/applicator" } , { "$ref" : "https://json-schema.org/draft/2019-09/meta/core" }] }),
    )
}
fn source_draft2019_09_metaschema_optional_vocabulary_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/metaschema-optional-vocabulary.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/metaschema-optional-vocabulary.json" , "$vocabulary" : { "https://json-schema.org/draft/2019-09/vocab/validation" : true , "https://json-schema.org/draft/2019-09/vocab/core" : true , "http://localhost:1234/draft/2019-09/vocab/custom" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2019-09/meta/validation" } , { "$ref" : "https://json-schema.org/draft/2019-09/meta/core" }] }),
    )
}
fn source_draft2019_09_name_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/name-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
    )
}
fn source_draft2019_09_nested_foo_ref_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/nested/foo-ref-string.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
    )
}
fn source_draft2019_09_nested_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/nested/string.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "type" : "string" }),
    )
}
fn source_draft2019_09_ref_and_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/ref-and-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
    )
}
fn source_draft2019_09_sub_schemas_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/subSchemas-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
    )
}
fn source_draft2019_09_sub_schemas_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/subSchemas.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2019-09/schema" , "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
    )
}
fn source_draft2019_09_tree_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2019-09/tree.json",
        json ! ({ "description" : "tree schema, extensible" , "$schema" : "https://json-schema.org/draft/2019-09/schema" , "$id" : "http://localhost:1234/draft2019-09/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
    )
}
fn source_draft2020_12_base_uri_change_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/baseUriChange/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
    )
}
fn source_draft2020_12_base_uri_change_folder_folder_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/baseUriChangeFolder/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
    )
}
fn source_draft2020_12_base_uri_change_folder_in_subschema_folder_integer_json(
) -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/baseUriChangeFolderInSubschema/folderInteger.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
    )
}
fn source_draft2020_12_extendible_dynamic_ref_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/extendible-dynamic-ref.json",
        json ! ({ "description" : "extendible array" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
    )
}
fn source_draft2020_12_format_assertion_false_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/format-assertion-false.json",
        json ! ({ "$id" : "http://localhost:1234/draft2020-12/format-assertion-false.json" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/core" : true , "https://json-schema.org/draft/2020-12/vocab/format-assertion" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/core" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/format-assertion" }] }),
    )
}
fn source_draft2020_12_format_assertion_true_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/format-assertion-true.json",
        json ! ({ "$id" : "http://localhost:1234/draft2020-12/format-assertion-true.json" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/core" : true , "https://json-schema.org/draft/2020-12/vocab/format-assertion" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/core" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/format-assertion" }] }),
    )
}
fn source_draft2020_12_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/integer.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "integer" }),
    )
}
fn source_draft2020_12_location_independent_identifier_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/locationIndependentIdentifier.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
    )
}
fn source_draft2020_12_metaschema_no_validation_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/metaschema-no-validation.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/metaschema-no-validation.json" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/applicator" : true , "https://json-schema.org/draft/2020-12/vocab/core" : true } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/applicator" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/core" }] }),
    )
}
fn source_draft2020_12_metaschema_optional_vocabulary_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/metaschema-optional-vocabulary.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/metaschema-optional-vocabulary.json" , "$vocabulary" : { "https://json-schema.org/draft/2020-12/vocab/validation" : true , "https://json-schema.org/draft/2020-12/vocab/core" : true , "http://localhost:1234/draft/2020-12/vocab/custom" : false } , "allOf" : [{ "$ref" : "https://json-schema.org/draft/2020-12/meta/validation" } , { "$ref" : "https://json-schema.org/draft/2020-12/meta/core" }] }),
    )
}
fn source_draft2020_12_name_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/name-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
    )
}
fn source_draft2020_12_nested_foo_ref_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/nested/foo-ref-string.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
    )
}
fn source_draft2020_12_nested_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/nested/string.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "type" : "string" }),
    )
}
fn source_draft2020_12_prefix_items_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/prefixItems.json",
        json ! ({ "$id" : "http://localhost:1234/draft2020-12/prefixItems.json" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "prefixItems" : [{ "type" : "string" }] }),
    )
}
fn source_draft2020_12_ref_and_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/ref-and-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
    )
}
fn source_draft2020_12_sub_schemas_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/subSchemas-defs.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
    )
}
fn source_draft2020_12_sub_schemas_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/subSchemas.json",
        json ! ({ "$schema" : "https://json-schema.org/draft/2020-12/schema" , "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
    )
}
fn source_draft2020_12_tree_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft2020-12/tree.json",
        json ! ({ "description" : "tree schema, extensible" , "$schema" : "https://json-schema.org/draft/2020-12/schema" , "$id" : "http://localhost:1234/draft2020-12/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
    )
}
fn source_draft7_ignore_dependent_required_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/draft7/ignore-dependentRequired.json",
        json ! ({ "$id" : "http://localhost:1234/draft7/integer.json" , "$schema" : "http://json-schema.org/draft-07/schema#" , "dependentRequired" : { "foo" : ["bar"] } }),
    )
}
fn source_extendible_dynamic_ref_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/extendible-dynamic-ref.json",
        json ! ({ "description" : "extendible array" , "$id" : "http://localhost:1234/extendible-dynamic-ref.json" , "type" : "object" , "properties" : { "elements" : { "type" : "array" , "items" : { "$dynamicRef" : "#elements" } } } , "required" : ["elements"] , "additionalProperties" : false , "$defs" : { "elements" : { "$dynamicAnchor" : "elements" } } }),
    )
}
fn source_integer_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/integer.json",
        json ! ({ "type" : "integer" }),
    )
}
fn source_location_independent_identifier_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/locationIndependentIdentifier.json",
        json ! ({ "$defs" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$anchor" : "foo" , "type" : "integer" } } }),
    )
}
fn source_location_independent_identifier_draft4_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/locationIndependentIdentifierDraft4.json",
        json ! ({ "definitions" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "id" : "#foo" , "type" : "integer" } } }),
    )
}
fn source_location_independent_identifier_pre2019_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/locationIndependentIdentifierPre2019.json",
        json ! ({ "definitions" : { "refToInteger" : { "$ref" : "#foo" } , "A" : { "$id" : "#foo" , "type" : "integer" } } }),
    )
}
fn source_name_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/name-defs.json",
        json ! ({ "$defs" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
    )
}
fn source_name_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/name.json",
        json ! ({ "definitions" : { "orNull" : { "anyOf" : [{ "type" : "null" } , { "$ref" : "#" }] } } , "type" : "string" }),
    )
}
fn source_nested_foo_ref_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/nested/foo-ref-string.json",
        json ! ({ "type" : "object" , "properties" : { "foo" : { "$ref" : "string.json" } } }),
    )
}
fn source_nested_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/nested/string.json",
        json ! ({ "type" : "string" }),
    )
}
fn source_nested_absolute_ref_to_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/nested-absolute-ref-to-string.json",
        json ! ({ "$defs" : { "bar" : { "$id" : "http://localhost:1234/the-nested-id.json" , "type" : "string" } } , "$ref" : "http://localhost:1234/the-nested-id.json" }),
    )
}
fn source_ref_and_definitions_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/ref-and-definitions.json",
        json ! ({ "$id" : "http://localhost:1234/ref-and-definitions.json" , "definitions" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "allOf" : [{ "$ref" : "#/definitions/inner" }] }),
    )
}
fn source_ref_and_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/ref-and-defs.json",
        json ! ({ "$id" : "http://localhost:1234/ref-and-defs.json" , "$defs" : { "inner" : { "properties" : { "bar" : { "type" : "string" } } } } , "$ref" : "#/$defs/inner" }),
    )
}
fn source_sub_schemas_defs_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/subSchemas-defs.json",
        json ! ({ "$defs" : { "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/$defs/integer" } } }),
    )
}
fn source_sub_schemas_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/subSchemas.json",
        json ! ({ "integer" : { "type" : "integer" } , "refToInteger" : { "$ref" : "#/integer" } }),
    )
}
fn source_tree_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/tree.json",
        json ! ({ "description" : "tree schema, extensible" , "$id" : "http://localhost:1234/tree.json" , "$dynamicAnchor" : "node" , "type" : "object" , "properties" : { "data" : true , "children" : { "type" : "array" , "items" : { "$dynamicRef" : "#node" } } } }),
    )
}
fn source_urn_ref_string_json() -> (&'static str, Value) {
    (
        "http://localhost:1234/urn-ref-string.json",
        json ! ({ "$id" : "urn:uuid:feebdaed-ffff-0000-ffff-0000deadbeef" , "$defs" : { "bar" : { "type" : "string" } } , "$ref" : "#/$defs/bar" }),
    )
}
