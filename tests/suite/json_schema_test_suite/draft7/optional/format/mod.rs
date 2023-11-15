use grill::Interrogator;
async fn interrogator() {
    use super::Draft7;
    use once_cell::sync::Lazy;
    static INTERROGATOR: Lazy<Result<Interrogator, grill::error::BuildError>> =
        Lazy::new(|| super::build(Draft7::interrogator(&crate::Harness)).await);
}
async fn interrogator() {
    todo!()
}
mod date;
mod date_time;
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
