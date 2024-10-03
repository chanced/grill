use core::slice;
use either::Either;
use jsonptr::{PointerBuf, Token};
use serde_json::Value;
use std::iter::{Enumerate, Map};

type MapIterFn = for<'v> fn((&'v String, &'v Value)) -> (Token<'v>, &'v Value);
type MapIter<'v> = Map<serde_json::map::Iter<'v>, MapIterFn>;
type SliceIterFn = for<'v> fn((usize, &'v Value)) -> (Token<'v>, &'v Value);
type SliceIter<'v> = Map<Enumerate<slice::Iter<'v, Value>>, SliceIterFn>;

/// An iterator that walks a JSON value from a specified path, emitting the
/// value's path represented a JSON Pointer and the value itself.
pub struct WalkFrom<'v> {
    steps: Vec<Step<'v>>,
}
struct Node<'v> {
    from: PointerBuf,
    iter: Either<MapIter<'v>, SliceIter<'v>>,
}
impl<'v> Node<'v> {
    fn new(from: PointerBuf, value: &'v Value) -> Self {
        match value {
            Value::Object(map) => Self {
                from,
                iter: Either::Left(map.iter().map(object_entry_to_step)),
            },
            Value::Array(slice) => Self {
                from,
                iter: Either::Right(slice.iter().enumerate().map(array_entry_to_step)),
            },
            _ => unreachable!(),
        }
    }
}

enum Step<'v> {
    Node(Node<'v>),
    Root((PointerBuf, &'v Value)),
}

fn array_entry_to_step((index, value): (usize, &Value)) -> (Token, &Value) {
    (Token::new(index.to_string()), value)
}
fn object_entry_to_step<'v>((key, value): (&'v String, &'v Value)) -> (Token<'v>, &'v Value) {
    (Token::new(key), value)
}

impl<'v> Iterator for WalkFrom<'v> {
    type Item = (PointerBuf, &'v Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self.steps.pop()? {
            Step::Node(mut node) => {
                let (token, value) = node.iter.next()?;
                let mut path = node.from.clone();
                path.push_back(token);
                if value.is_array() || value.is_object() {
                    self.steps.push(Step::Node(Node::new(path.clone(), value)));
                }
                Some((path, value))
            }
            Step::Root((path, value)) => {
                if value.is_array() || value.is_object() {
                    self.steps.push(Step::Node(Node::new(path.clone(), value)));
                }
                Some((path, value))
            }
        }
    }
}
