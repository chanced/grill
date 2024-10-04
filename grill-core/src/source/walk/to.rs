use std::{num::NonZeroUsize, path::Path};

use jsonptr::{Pointer, Resolve, Token};
use serde_json::Value;

#[derive(Debug)]
pub struct WalkTo<'p, 'v> {
    value: &'v Value,
    full_path: &'p Pointer,
    offset: Option<NonZeroUsize>,
}
impl<'p, 'v> WalkTo<'p, 'v> {
    pub fn new(value: &'v Value, to: &'p Pointer) -> Self {
        Self {
            value,
            full_path: to,
            offset: None,
        }
    }
}

// /some/example
//  ^

impl<'p, 'v> Iterator for WalkTo<'p, 'v> {
    type Item = (&'p Pointer, &'v Value);
    fn next(&mut self) -> Option<Self::Item> {
        // we need to get the offset to determine where we are in the pointer
        let offset = if let Some(offset) = self.offset {
            // the offset has previously been set, so we use that
            offset.get()
        } else {
            // An empty offset means we are at the beginning of the walk.

            // this is a special case where the target path is root
            // we need to handle this separately because we later account
            // for tokens, which an empty path has none of.
            if self.full_path.is_root() {
                // the target path is root, so we set the offset to 1 ensures we
                // do not send repeats due to the bounds check on offset below.
                self.offset = NonZeroUsize::new(1);
                return Some((Pointer::root(), self.value));
            }
            // if the offset was not previously set, we start at 0
            0
        };

        // checking to make sure we are not out of bounds
        if offset > self.full_path.len() {
            return None;
        }

        // split the path at the offset, where everything to the left
        // is the full path of the current value to be sent and everything
        // to the right is the remaining path to be resolved.
        let (path, remaining) = self
            .full_path
            .split_at(offset)
            .unwrap_or((self.full_path, Pointer::root()));

        if let Some(next) = remaining.first() {
            // if there is a next token, we set the offset to the next token's length
            // plus 1 to account for the slash.
            self.offset = NonZeroUsize::new(offset + next.encoded().len() + 1);
        } else {
            // otherwise we intentionally push the offset out of bounds
            self.offset = NonZeroUsize::new(offset + 1)
        }

        // we want the last token as a `&Pointer` so that we can use the resolve logic
        // the path is either splittable (contains a token) or is empty (root).
        //
        // If it is splittable, we use the token's length to determine where the token's
        // offset and split the path there.
        //
        // Otherwise, we use the root pointer.
        let resolvable = path
            .last()
            .map(|t| path.split_at(path.len() - t.encoded().len() - 1).unwrap().1)
            .unwrap_or(Pointer::root());
        // we attempt to resolve the value
        let value = self.value.resolve(resolvable).ok()?;
        self.value = value;
        Some((path, value))
    }
}

#[cfg(test)]
mod test {
    use jsonptr::Pointer;
    use serde_json::json;

    use super::WalkTo;

    #[test]
    fn walk() {
        dbg!("inside test");
        let value = json!({
            "foo": {
                "bar": [
                    {
                        "baz": {
                            "qux": 34
                        }
                    }
                ]
            }
        });
        let full_path = Pointer::from_static("/foo/bar/0/baz/qux");
        let walk_to = WalkTo::new(&value, full_path);
        let foo = value.get("foo").unwrap();
        let foo_bar = foo.get("bar").unwrap();
        let foo_bar_0 = foo_bar.get(0).unwrap();
        let foo_bar_0_baz = foo_bar_0.get("baz").unwrap();
        let foo_bar_0_baz_qux = foo_bar_0_baz.get("qux").unwrap();
        assert_eq!(
            walk_to.collect::<Vec<_>>(),
            vec![
                (Pointer::from_static(""), &value),
                (Pointer::from_static("/foo"), foo),
                (Pointer::from_static("/foo/bar"), foo_bar),
                (Pointer::from_static("/foo/bar/0"), foo_bar_0),
                (Pointer::from_static("/foo/bar/0/baz"), foo_bar_0_baz),
                (
                    Pointer::from_static("/foo/bar/0/baz/qux"),
                    foo_bar_0_baz_qux
                ),
            ]
        );
    }
}
