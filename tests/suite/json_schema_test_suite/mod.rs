mod draft2020_12;
mod sources;
use grill::{Finish, Interrogator, Key};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
pub trait Harness: Copy {
    type Draft202012: Draft202012;
    fn draft2020_12(&self) -> Self::Draft202012;
}
pub trait Draft202012 {
    fn build(&self) -> grill::Build;
    fn setup_additional_properties(&self, interrogator: &mut Interrogator) {}
    fn setup_all_of(&self, interrogator: &mut Interrogator) {}
    fn setup_anchor(&self, interrogator: &mut Interrogator) {}
    fn setup_any_of(&self, interrogator: &mut Interrogator) {}
    fn setup_boolean_schema(&self, interrogator: &mut Interrogator) {}
    fn setup_const(&self, interrogator: &mut Interrogator) {}
    fn setup_contains(&self, interrogator: &mut Interrogator) {}
    fn setup_content(&self, interrogator: &mut Interrogator) {}
    fn setup_default(&self, interrogator: &mut Interrogator) {}
    fn setup_defs(&self, interrogator: &mut Interrogator) {}
    fn setup_dependent_required(&self, interrogator: &mut Interrogator) {}
    fn setup_dependent_schemas(&self, interrogator: &mut Interrogator) {}
    fn setup_dynamic_ref(&self, interrogator: &mut Interrogator) {}
    fn setup_enum(&self, interrogator: &mut Interrogator) {}
    fn setup_exclusive_maximum(&self, interrogator: &mut Interrogator) {}
    fn setup_exclusive_minimum(&self, interrogator: &mut Interrogator) {}
    fn setup_format(&self, interrogator: &mut Interrogator) {}
    fn setup_id(&self, interrogator: &mut Interrogator) {}
    fn setup_if_then_else(&self, interrogator: &mut Interrogator) {}
    fn setup_infinite_loop_detection(&self, interrogator: &mut Interrogator) {}
    fn setup_items(&self, interrogator: &mut Interrogator) {}
    fn setup_max_contains(&self, interrogator: &mut Interrogator) {}
    fn setup_max_items(&self, interrogator: &mut Interrogator) {}
    fn setup_max_length(&self, interrogator: &mut Interrogator) {}
    fn setup_max_properties(&self, interrogator: &mut Interrogator) {}
    fn setup_maximum(&self, interrogator: &mut Interrogator) {}
    fn setup_min_contains(&self, interrogator: &mut Interrogator) {}
    fn setup_min_items(&self, interrogator: &mut Interrogator) {}
    fn setup_min_length(&self, interrogator: &mut Interrogator) {}
    fn setup_min_properties(&self, interrogator: &mut Interrogator) {}
    fn setup_minimum(&self, interrogator: &mut Interrogator) {}
    fn setup_multiple_of(&self, interrogator: &mut Interrogator) {}
    fn setup_not(&self, interrogator: &mut Interrogator) {}
    fn setup_one_of(&self, interrogator: &mut Interrogator) {}
    fn setup_pattern(&self, interrogator: &mut Interrogator) {}
    fn setup_pattern_properties(&self, interrogator: &mut Interrogator) {}
    fn setup_prefix_items(&self, interrogator: &mut Interrogator) {}
    fn setup_properties(&self, interrogator: &mut Interrogator) {}
    fn setup_property_names(&self, interrogator: &mut Interrogator) {}
    fn setup_ref(&self, interrogator: &mut Interrogator) {}
    fn setup_ref_remote(&self, interrogator: &mut Interrogator) {}
    fn setup_required(&self, interrogator: &mut Interrogator) {}
    fn setup_type(&self, interrogator: &mut Interrogator) {}
    fn setup_unevaluated_items(&self, interrogator: &mut Interrogator) {}
    fn setup_unevaluated_properties(&self, interrogator: &mut Interrogator) {}
    fn setup_unique_items(&self, interrogator: &mut Interrogator) {}
    fn setup_unknown_keyword(&self, interrogator: &mut Interrogator) {}
    fn setup_vocabulary(&self, interrogator: &mut Interrogator) {}
    fn setup_optional(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_date_time(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_date(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_duration(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_email(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_hostname(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_idn_email(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_idn_hostname(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_ipv4(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_ipv6(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_iri_reference(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_iri(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_json_pointer(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_regex(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_relative_json_pointer(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_time(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_unknown(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_uri_reference(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_uri_template(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_uri(&self, interrogator: &mut Interrogator) {}
    fn setup_optional_format_uuid(&self, interrogator: &mut Interrogator) {}
}
async fn build(build: grill::Build) -> Result<grill::Interrogator, grill::error::BuildError> {
    futures::executor::block_on(|| build.source_static_values(sources::sources()))
}
