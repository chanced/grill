use core::slice::Iter as SliceIter;
use jsonptr::{resolve::ResolveError, PointerBuf, Resolve, Token};
use serde_json::{map::Iter as MapIter, Map, Value};
use std::iter::{Enumerate, Peekable};

/// An iterator that walks a JSON value from a specified path, emitting the
/// value's path represented a JSON Pointer and the value itself.
pub struct WalkFrom<'v> {
    steps: Vec<Step<'v>>,
}
impl<'v> WalkFrom<'v> {
    pub fn new(from: PointerBuf, value: &'v Value) -> Result<Self, ResolveError> {
        let value = value.resolve(&from)?;
        Ok(Self {
            steps: vec![Step::Root((from, value))],
        })
    }
}

enum Iter<'v> {
    Object(MapIter<'v>),
    Array(Enumerate<SliceIter<'v, Value>>),
}
impl<'v> Iter<'v> {
    fn object(map: &'v Map<String, Value>) -> Self {
        map.iter().into()
    }

    fn array(slice: &'v [Value]) -> Iter<'v> {
        slice.iter().into()
    }
}
impl<'v> From<MapIter<'v>> for Iter<'v> {
    fn from(iter: MapIter<'v>) -> Self {
        Self::Object(iter)
    }
}
impl<'v> From<SliceIter<'v, Value>> for Iter<'v> {
    fn from(iter: SliceIter<'v, Value>) -> Self {
        Self::Array(iter.enumerate())
    }
}

impl<'v> Iterator for Iter<'v> {
    type Item = (Token<'v>, &'v Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Iter::Object(iter) => iter.next().map(|(k, v)| (Token::from(k), v)),
            Iter::Array(iter) => iter.next().map(|(i, v)| (Token::from(i), v)),
        }
    }
}

struct Node<'v> {
    from: PointerBuf,
    iter: Peekable<Iter<'v>>,
}
impl<'v> Node<'v> {
    fn new(from: PointerBuf, value: &'v Value) -> Self {
        match value {
            Value::Object(map) => Self {
                from,
                iter: Iter::object(map).peekable(),
            },
            Value::Array(slice) => Self {
                from,
                iter: Iter::array(slice).peekable(),
            },
            _ => unreachable!(),
        }
    }
}

enum Step<'v> {
    Node(Node<'v>),
    Root((PointerBuf, &'v Value)),
}

impl<'v> Iterator for WalkFrom<'v> {
    type Item = (PointerBuf, &'v Value);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.steps.is_empty() {
            match self.steps.pop().unwrap() {
                Step::Node(mut node) => {
                    let Some((token, value)) = node.iter.next() else {
                        continue;
                    };
                    let path = node.from.with_trailing_token(token);
                    if node.iter.peek().is_some() {
                        self.steps.push(Step::Node(node));
                    }
                    if value.is_array() || value.is_object() {
                        self.steps.push(Step::Node(Node::new(path.clone(), value)));
                    }
                    return Some((path, value));
                }
                Step::Root((path, value)) => {
                    if value.is_array() || value.is_object() {
                        self.steps.push(Step::Node(Node::new(path.clone(), value)));
                    }
                    return Some((path, value));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::WalkFrom;
    use jsonptr::PointerBuf;
    use serde_json::json;

    #[test]
    fn valid() {
        let value = json!({
            "foo": {
                "bar": [
                    {
                        "baz": {
                            "qux": 34
                        }
                    },
                    {
                        "baz": {
                            "qux": 21
                        }
                    }
                ]
            }
        });
        let foo = value.get("foo").unwrap();
        let foo_bar = foo.get("bar").unwrap();
        let foo_bar_0 = foo_bar.get(0).unwrap();
        let foo_bar_0_baz = foo_bar_0.get("baz").unwrap();
        let foo_bar_0_baz_qux = foo_bar_0_baz.get("qux").unwrap();
        let foo_bar_1 = foo_bar.get(1).unwrap();
        let foo_bar_1_baz = foo_bar_1.get("baz").unwrap();
        let foo_bar_1_baz_qux = foo_bar_1_baz.get("qux").unwrap();

        let from_root = WalkFrom::new(PointerBuf::default(), &value).unwrap();

        assert_eq!(
            from_root.collect::<Vec<_>>(),
            vec![
                ("".try_into().unwrap(), &value),
                ("/foo".try_into().unwrap(), foo),
                ("/foo/bar".try_into().unwrap(), foo_bar),
                ("/foo/bar/0".try_into().unwrap(), foo_bar_0),
                ("/foo/bar/0/baz".try_into().unwrap(), foo_bar_0_baz),
                ("/foo/bar/0/baz/qux".try_into().unwrap(), foo_bar_0_baz_qux),
                ("/foo/bar/1".try_into().unwrap(), foo_bar_1),
                ("/foo/bar/1/baz".try_into().unwrap(), foo_bar_1_baz),
                ("/foo/bar/1/baz/qux".try_into().unwrap(), foo_bar_1_baz_qux),
            ]
        );
    }
}
