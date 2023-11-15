use grill::Interrogator;

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
    fn interrogator(&self) -> grill::Finish {
        Interrogator::build().finish()
    }
}
