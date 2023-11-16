use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_optional(interrogator)
    }
    interrogator
}
use super::*;
use grill::{error::BuildError, Interrogator};
mod format;
