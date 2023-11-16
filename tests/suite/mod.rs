use grill::{Interrogator, JsonSchema};

mod json_schema_test_suite;

#[derive(Clone, Copy)]
pub struct Harness;
impl json_schema_test_suite::Harness for Harness {
    type Draft202012 = Harness;

    fn draft2020_12(&self) -> Self::Draft202012 {
        todo!()
    }
}

impl json_schema_test_suite::Draft202012 for Harness {
    fn build(&self) -> grill::Build {
        Interrogator::build().json_schema_2020_12()
    }
}
