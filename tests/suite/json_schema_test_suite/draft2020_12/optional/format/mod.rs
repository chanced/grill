use super::*;
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    let mut interrogator = super::interrogator();
    if let Ok(interrogator) = interrogator.as_mut() {
        crate::Harness.setup_optional_format(interrogator)
    }
    interrogator
}
use super::*;
use grill::{error::BuildError, Interrogator};
mod date;
mod date_time;
mod duration;
mod email;
mod hostname;
mod idn_email;
mod idn_hostname;
mod ipv4;
mod ipv6;
mod iri;
mod iri_reference;
mod json_pointer;
mod regex;
mod relative_json_pointer;
mod time;
mod unknown;
mod uri;
mod uri_reference;
mod uri_template;
mod uuid;
