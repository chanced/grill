use jsonptr::Pointer;

use crate::Location;
#[derive(Default)]
/// Contains state and location information for a given keyword pertaining
/// to an evaluation.
pub struct Scope {
    location: Location,
}

impl Scope {}
