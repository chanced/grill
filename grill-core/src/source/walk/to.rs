use std::path::Path;

use jsonptr::{Pointer, Resolve};
use serde_json::Value;

#[derive(Debug)]
pub struct WalkTo<'p, 'v> {
    value: &'v Value,
    full_path: &'p Pointer,
    next: Option<&'p Pointer>,
    remaining: &'p Pointer,
    offset: usize,
}
impl<'p, 'v> WalkTo<'p, 'v> {
    fn new(value: &'v Value, from: &'p Pointer) -> Self {
        Self {
            value,
            next: Some(Pointer::root()),
            full_path: from,
            remaining: from,
            offset: 0,
        }
    }
}

impl<'p, 'v> Iterator for WalkTo<'p, 'v> {
    type Item = (&'p Pointer, &'v Value);
    fn next(&mut self) -> Option<Self::Item> {
        let remaining_str = self.remaining.as_str();
        let next_str = self.next.map(|next| next.as_str());
        let _ = (remaining_str, next_str);

        let next = self.next.take()?;
        let value = self.value.resolve(next).ok()?;
        let (path, _) = self
            .full_path
            .split_at(self.offset)
            .unwrap_or((self.full_path, Pointer::root()));
        if !self.remaining.is_root() {
            if let Some(tok) = self.remaining.first() {
                let offset = tok.encoded().len() + 1;
                let (next_path, remaining) = if offset == self.remaining.len() {
                    self.remaining.split_at(0).unwrap()
                } else {
                    self.remaining.split_at(offset).unwrap()
                };
                dbg!(next_path, remaining);
                self.offset += offset;
                self.next = Some(next_path);
                self.remaining = remaining;
                self.value = value;
            } else {
                self.next = None;
            }
        }
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
                (full_path, foo_bar_0_baz_qux),
            ]
        );
    }
}
