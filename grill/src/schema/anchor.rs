use jsonptr::Pointer;

use super::Keyword;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor {
    /// Value of the anchor.  
    pub name: String,
    /// Path to the anchor
    pub path: Pointer,
    /// Path to the object which contains the anchor
    pub container_path: Pointer,
    /// The keyword of the anchor, e.g. `"$anchor"`, `"$dynamicAnchor"`, `"$recursiveAnchor"`
    pub keyword: Keyword<'static>,
}
