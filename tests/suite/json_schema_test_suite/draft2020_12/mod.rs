use super::*;
use grill::{error::BuildError, Interrogator};
fn interrogator() -> Result<Interrogator, &'static BuildError> {
    use crate::Harness;
    use std::sync::OnceLock;
    static INTERROGATOR: OnceLock<Result<Interrogator, BuildError>> = OnceLock::new();
    INTERROGATOR
        .get_or_init(|| block_on(build(Harness.draft2020_12().build())))
        .as_ref()
        .map(Clone::clone)
}
mod additional_properties;
mod all_of;
mod anchor;
mod any_of;
mod boolean_schema;
mod const_;
mod contains;
mod content;
mod default;
mod defs;
mod dependent_required;
mod dependent_schemas;
mod dynamic_ref;
mod enum_;
mod exclusive_maximum;
mod exclusive_minimum;
mod format;
mod id;
mod if_then_else;
mod infinite_loop_detection;
mod items;
mod max_contains;
mod max_items;
mod max_length;
mod max_properties;
mod maximum;
mod min_contains;
mod min_items;
mod min_length;
mod min_properties;
mod minimum;
mod multiple_of;
mod not;
mod one_of;
mod optional;
mod pattern;
mod pattern_properties;
mod prefix_items;
mod properties;
mod property_names;
mod ref_;
mod ref_remote;
mod required;
mod type_;
mod unevaluated_items;
mod unevaluated_properties;
mod unique_items;
mod unknown_keyword;
mod vocabulary;
