use std::collections::VecDeque;

use grill_uri::AbsoluteUri;
use slotmap::Key;

use crate::schema::Reference;

#[derive(Debug)]
pub(super) struct Pending<K> {
    pub(super) key: Option<K>,
    pub(super) uri: AbsoluteUri,
    pub(super) parent: Option<K>,
    /// Whether to diregard some errors
    pub(super) continue_on_err: bool,
    pub(super) reference: Option<Reference<K>>,
    // index in the output queue
    pub(super) index: Option<usize>,
}

pub(super) struct Compiled<K> {
    pub(super) key: K,
    pub(super) index: Option<usize>,
}

#[derive(Debug)]
pub(super) struct Queue<K> {
    items: VecDeque<Pending<K>>,
}
impl<K> IntoIterator for Queue<K> {
    type Item = Pending<K>;
    type IntoIter = std::collections::vec_deque::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<K> Queue<K>
where
    K: Key,
{
    pub(super) fn new(uris: Vec<AbsoluteUri>) -> Self {
        Self {
            items: uris
                .into_iter()
                .enumerate()
                .map(|(index, uri)| Pending {
                    key: None,
                    uri,
                    parent: None,
                    continue_on_err: false,
                    reference: None,
                    index: Some(index),
                })
                .collect(),
        }
    }
    pub(super) fn len(&self) -> usize {
        self.items.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub(super) fn push_back(&mut self, item: Pending<K>) {
        self.items.push_back(item);
    }
    pub(super) fn push_front(&mut self, item: Pending<K>) {
        self.items.push_front(item);
    }
    pub(super) fn pop(&mut self) -> Option<Pending<K>> {
        self.items.pop_front()
    }
    pub(super) fn contains_key(&self, key: K) -> bool {
        self.items.iter().any(|item| item.key == Some(key))
    }
}
