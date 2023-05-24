use serde_json::Value;

pub trait Integration {
    type Schema;
    type Subschema;
    type CompiledSchema;
    type CompiledSubschema;
    type Output;
    type Annotation;
    fn identify(schema: &Value, path: &jsonptr::Pointer) -> Option<String>;
    fn detect_dialect(source: &Value, path: &jsonptr::Pointer) -> Option<String>;
}
