use crate::json_schema_test_suite::Draft202012;
async fn interrogator() {
    let mut interrogator = super::interrogator().await;
    Draft202012::setup_format(&crate::Harness, &mut interrogator);
    todo!()
}
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
