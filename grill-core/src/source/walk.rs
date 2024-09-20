use either::Either;
use jsonptr::{PointerBuf, Token};

use core::slice;
use serde_json::Value;
use std::{
    iter::{Enumerate, Map},
    ops::Deref,
};

type MapIterFn = for<'v> fn((&'v String, &'v Value)) -> (Token<'v>, &'v Value);
type MapIter<'v> = Map<serde_json::map::Iter<'v>, MapIterFn>;

type SliceIterFn = for<'v> fn((usize, &'v Value)) -> (Token<'v>, &'v Value);
type SliceIter<'v> = Map<Enumerate<slice::Iter<'v, Value>>, SliceIterFn>;

pub trait Walk {
    fn walk(&self, path: PointerBuf) -> WalkValue;
}

impl<V> Walk for V
where
    V: Deref<Target = Value>,
{
    fn walk(&self, path: PointerBuf) -> WalkValue {
        WalkValue::new(path, self)
    }
}

struct Node<'v> {
    base: PointerBuf,
    iter: Either<MapIter<'v>, SliceIter<'v>>,
}
impl<'v> Node<'v> {
    fn new(base: PointerBuf, value: &'v Value) -> Self {
        match value {
            Value::Object(map) => Self {
                base,
                iter: Either::Left(map.iter().map(object_entry_to_step)),
            },
            Value::Array(slice) => Self {
                base,
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

/// An iterator that walks a JSON value, emitting a JSON Pointer and the value
/// at that pointer.
pub struct WalkValue<'v> {
    steps: Vec<Step<'v>>,
}
impl<'v> WalkValue<'v> {
    pub fn new(path: PointerBuf, value: &'v Value) -> Self {
        Self {
            steps: vec![Step::Root((path, value))],
        }
    }
}
impl<'v> Iterator for WalkValue<'v> {
    type Item = (PointerBuf, &'v Value);

    fn next(&mut self) -> Option<Self::Item> {
        match self.steps.pop()? {
            Step::Node(mut node) => {
                let (token, value) = node.iter.next()?;
                let mut path = node.base.clone();
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
