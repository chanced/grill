use jsonptr::Pointer;

use crate::{evaluation::Field, Evaluation, Output};

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
    ) -> Evaluation<Self> {
        Evaluation::new(self, instance_location, keyword_location, output)
    }
}
