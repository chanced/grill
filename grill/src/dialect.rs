use std::collections::HashMap;

use crate::Vocabulary;
/// A composition of [`Vocabulary`].
pub struct Dialect {
    pub vocabularies: HashMap<String, DialectEntry>,
}
/// A [`Dialect`] entry which defines the [`Vocabulary`] if known and whether or
/// not the [`Vocabulary`] is required.
pub struct DialectEntry {
    pub vocabulary: Vocabulary,
    pub is_required: bool,
}
