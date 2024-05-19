pub(crate) trait ControlFlowExt<B, L>: Sized {
    /// Converts the `ControlFlow` into an `Option` which is `Some` if the
    /// `ControlFlow` was `Break` and `None` otherwise.
    ///
    /// Named `break_val` to avoid conflict with the `break` keyword and the
    /// nightly `break_value` method of `ControlFlow`.

    fn break_val(self) -> Option<B>;
    fn continue_val(self) -> Option<L>;
    fn map_continue<U, F: FnOnce(L) -> Option<U>>(self, f: F) -> Option<U>;
    fn map_break<U, F: FnOnce(B) -> Option<U>>(self, f: F) -> Option<U>;
    fn unwrap_break(self) -> B {
        self.break_val().unwrap()
    }
    fn unwrap_continue(self) -> L {
        self.continue_val().unwrap()
    }
}

impl<B, L> ControlFlowExt<B, L> for ControlFlow<B, L> {
    fn break_val(self) -> Option<B> {
        match self {
            ControlFlow::Break(b) => Some(b),
            _ => None,
        }
    }

    fn continue_val(self) -> Option<L> {
        match self {
            ControlFlow::Continue(c) => Some(c),
            _ => None,
        }
    }

    fn map_continue<U, F: FnOnce(L) -> Option<U>>(self, _f: F) -> Option<U> {
        match self {
            ControlFlow::Continue(_) => todo!(),
            ControlFlow::Break(_) => todo!(),
        }
    }

    fn map_break<U, F: FnOnce(B) -> Option<U>>(self, _f: F) -> Option<U> {
        todo!()
    }
}
