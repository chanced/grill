use jsonptr::Pointer;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Anchor<'v> {
    Static {
        /// Value of the anchor.
        name: &'v str,
        /// JSON Pointer to the containing value
        pointer: Pointer,
        /// The containing `Value`
        container: &'v Value,
    },
    Dynamic {
        /// Value of the anchor.  
        name: &'v str,
        /// JSON Pointer to the containing value
        pointer: Pointer,
        /// The containing `Value`
        container: &'v Value,
    },
    Recursive {
        /// JSON Pointer to the containing value
        pointer: Pointer,
        /// The containing `Value`
        container: &'v Value,
    },
}

impl<'v> Anchor<'v> {
    /// Returns `true` if the anchor is [`Static`].
    ///
    /// [`Static`]: Anchor::Static
    #[must_use]
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static { .. })
    }

    /// Returns `true` if the anchor is [`Dynamic`].
    ///
    /// [`Dynamic`]: Anchor::Dynamic
    #[must_use]
    pub fn is_dynamic(&self) -> bool {
        matches!(self, Self::Dynamic { .. })
    }

    /// Returns `true` if the anchor is [`Recursive`].
    ///
    /// [`Recursive`]: Anchor::Recursive
    #[must_use]
    pub fn is_recursive(&self) -> bool {
        matches!(self, Self::Recursive { .. })
    }
}
