use serde::{Deserialize, Serialize};
use slotmap::new_key_type;

new_key_type! {
    pub(crate) struct InternalKey;
}
impl From<InternalKey> for Key {
    fn from(value: InternalKey) -> Self {
        Self(value)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Key(pub(crate) InternalKey);
