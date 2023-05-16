use serde::{Deserialize, Serialize};
use uniresid::Uri;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Location {
    pub keyword_location: jsonptr::Pointer,
    pub absolute_location: Uri,
    pub instance_location: jsonptr::Pointer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_keyword_location: Option<Uri>,
}

impl Location {
    #[must_use]
    pub fn new(
        keyword_location: jsonptr::Pointer,
        absolute_location: Uri,
        instance_location: jsonptr::Pointer,
    ) -> Self {
        Self {
            keyword_location,
            absolute_location,
            instance_location,
            absolute_keyword_location: None,
        }
    }
}
