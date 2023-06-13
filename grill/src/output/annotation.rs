use std::{
    collections::{BTreeMap, VecDeque},
    default, mem,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Location;

use super::{Node, ValidationError};

pub struct Iter<'i, 'v> {
    queue: VecDeque<&'i Annotation<'v>>,
}
impl<'i, 'v> Iter<'i, 'v> {
    pub fn new(annotation: &'i Annotation<'v>) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(annotation);
        Self { queue }
    }
}
impl<'i, 'v> Iterator for Iter<'i, 'v> {
    type Item = &'i Annotation<'v>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.queue.pop_front()?;
        for annotation in next.annotations().iter().rev() {
            self.queue.push_front(annotation);
        }
        Some(next)
    }
}

#[derive(Debug, Default)]
pub struct Invalid<'i, 'v> {
    queue: VecDeque<&'i Annotation<'v>>,
}
impl<'i, 'v> Invalid<'i, 'v> {
    pub fn new(annotation: &'i Annotation<'v>) -> Self {
        if annotation.is_valid() {
            return Self::default();
        }
        let mut queue = VecDeque::new();
        queue.push_back(annotation);
        Self { queue }
    }
}

impl<'i, 'v> Iterator for Invalid<'i, 'v> {
    type Item = &'i Annotation<'v>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: &'i Annotation<'v>;
        loop {
            next = self.queue.pop_front()?;
            self.queue.reserve(next.node().invalid.len());
            for annotation in next.node().invalid.iter().rev() {
                self.queue.push_front(annotation);
            }
            if next.is_invalid() {
                return Some(next);
            }
        }
    }
}
pub struct Valid<'i, 'v> {
    queue: VecDeque<&'i Annotation<'v>>,
}

impl<'i, 'v> Valid<'i, 'v> {
    pub fn new(annotation: &'i Annotation<'v>) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(annotation);
        Self { queue }
    }
}

impl<'i, 'v> Iterator for Valid<'i, 'v> {
    type Item = &'i Annotation<'v>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: &'i Annotation<'v>;
        loop {
            next = self.queue.pop_front()?;
            self.queue.reserve(next.annotations().len());
            for annotation in next.annotations().iter().rev() {
                self.queue.push_front(annotation);
            }
            if next.is_valid() {
                return Some(next);
            }
        }
    }
}

/// Represents a valid or invalid (error) annotation
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Annotation<'v> {
    /// Valid annotation
    Valid(Node<'v>),
    /// Invalid annotaiton, meaning that the [`Detail`] either contains an [`ValidationError`] or
    /// has nested error annotations.
    Invalid(Node<'v>),
}

impl<'v> Annotation<'v> {
    #[must_use]
    pub fn new(location: Location, error: Option<Box<dyn ValidationError<'v>>>) -> Self {
        Self::Valid(Node {
            location,
            error,
            valid: Vec::default(),
            invalid: Vec::default(),
            additional_properties: BTreeMap::default(),
        })
    }
    pub fn node(&self) -> &Node<'v> {
        match self {
            Annotation::Valid(n) | Annotation::Invalid(n) => n,
        }
    }
    /// Sets the error of this annotation
    pub fn error(&mut self, error: impl 'v + ValidationError<'v>) {
        let error = Some(Box::new(error) as Box<dyn 'v + ValidationError<'v>>);
        match self {
            Annotation::Valid(n) => {
                n.error = error;
                *self = Annotation::Invalid(mem::take(n));
            }
            Annotation::Invalid(n) => {
                n.error = error;
            }
        }
    }

    /// Returns a slice of nested `Annotation`s.
    pub fn annotations(&self) -> &[Annotation<'v>] {
        match self {
            Annotation::Valid(n) | Annotation::Invalid(n) => &n.valid,
        }
    }

    /// Returns an [`Iter`] which **includes self** and nested `Annotation`s.
    ///
    /// For direct nested `Annotation`s, use [`annotations`].
    pub fn iter(&self) -> Iter<'_, 'v> {
        Iter::new(self)
    }

    /// Returns a depth-first [`Iterator`] [`Valid`] over valid `Annotation`s,
    /// **including `self`** if `self` is valid.
    pub fn valid(&self) -> Valid<'_, 'v> {
        Valid::new(self)
    }

    /// Returns a depth-first [`Iterator`] [`Invalid`] over invalid `Annotation`s,
    /// **including `self`** if `self` is invalid.
    pub fn invalid(&self) -> Invalid<'_, 'v> {
        Invalid::new(self)
    }

    pub fn field(&mut self, field: String, value: Value) {
        match self {
            Annotation::Invalid(n) | Annotation::Valid(n) => {
                n.additional_props.insert(field, value);
            }
        }
    }

    /// Pushes a new nested annotation
    pub fn add(&mut self, annotation: Annotation<'v>) {
        match self {
            Annotation::Valid(detail) => match annotation {
                Annotation::Valid(_) => {
                    detail.valid.push(annotation);
                }
                Annotation::Invalid(_) => {
                    detail.invalid.push(annotation);
                    let detail = mem::take(detail);
                    *self = Annotation::Invalid(detail);
                }
            },
            Annotation::Invalid(detail) => match annotation {
                Annotation::Valid(_) => {
                    detail.valid.push(annotation);
                }
                Annotation::Invalid(_) => {
                    detail.invalid.push(annotation);
                }
            },
        }
    }

    /// Returns `true` if the annotation is [`Valid`].
    ///
    /// [`Valid`]: Annotation::Valid
    #[must_use]
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Valid(..))
    }

    /// Returns `true` if the annotation is [`Invalid`].
    ///
    /// [`Invalid`]: Annotation::Invalid
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(..))
    }

    /// Returns the [`Detail`] of this annotation.
    #[must_use]
    pub fn detail(&self) -> &Node<'v> {
        match self {
            Annotation::Invalid(detail) | Annotation::Valid(detail) => detail,
        }
    }

    // pub(crate) fn is_empty(&self) -> bool {
    //     match self {
    //         Annotation::Valid(v) => v.annotations.is_empty(),
    //         Annotation::Invalid(i) => i.annotations.is_empty(),
    //     }
    // }

    #[must_use]
    pub fn as_valid(&self) -> Option<&Node<'v>> {
        if let Self::Valid(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// # Errors
    /// Returns `Err(self)` if the annotation is [`Invalid`].
    pub fn try_into_valid(self) -> Result<Node<'v>, Self> {
        if let Self::Valid(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }

    #[must_use]
    pub fn as_invalid(&self) -> Option<&Node<'v>> {
        if let Self::Invalid(v) = self {
            Some(v)
        } else {
            None
        }
    }
    /// # Errors
    /// Returns `Err(self)` if the annotation is [`Valid`].
    pub fn try_into_invalid(self) -> Result<Node<'v>, Self> {
        if let Self::Invalid(v) = self {
            Ok(v)
        } else {
            Err(self)
        }
    }
}

impl<'de> Deserialize<'de> for Annotation<'static> {
    fn deserialize<D>(deserializer: D) -> Result<Annotation<'static>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let d: Node = Deserialize::deserialize(deserializer)?;
        if d.is_error() {
            Ok(Annotation::Valid(d))
        } else {
            Ok(Annotation::Invalid(d))
        }
    }
}

impl<'v> Default for Annotation<'v> {
    fn default() -> Self {
        Self::Valid(Node::default())
    }
}
