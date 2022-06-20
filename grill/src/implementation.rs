use jsonptr::Pointer;

use crate::{annotation::Field, Annotation, Output};

pub trait Implementation: Send + Sync + Clone {
    /// The name of the field which represents the location of the keyword field.
    ///
    /// - In JSON Schema, this is `"keywordLocation"`
    /// - In JSON Typedef, this is `"schemaPath"`
    fn keyword_location_field() -> &'static str;
    /// The name of the field which represents the location of the instance field
    ///
    /// - In JSON Schema, this is `"instanceLocation"`
    /// - In JSON Typedef, this is `"instancePath"`
    fn instance_location_field() -> &'static str;

    /// The name of the field which represents the error message.
    ///
    /// - In JSON Schema, this is `"error"`
    ///
    fn error_field() -> &'static str;

    /// Returns a new `Evaluation`
    fn evaluation(
        &self,
        instance_location: Pointer,
        keyword_location: Pointer,
        output: Output,
    ) -> Annotation<Self> {
        Annotation::new(self, instance_location, keyword_location, output)
    }
}
