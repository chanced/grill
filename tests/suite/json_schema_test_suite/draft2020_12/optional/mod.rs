use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_optional(&crate::Harness, &mut interrogator);
    todo!()
}
use grill::{error::BuildError, Interrogator};
mod format;
