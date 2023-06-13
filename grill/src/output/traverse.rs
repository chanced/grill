use std::collections::VecDeque;

use super::Node;

pub struct TraverseAnnotations<'n, 'v> {
    nodes: VecDeque<&'n Node<'v>>,
}

impl<'n, 'v> Iterator for TraverseAnnotations<'n, 'v> {
    type Item = &'n Node<'v>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: &'n Node<'v>;
        let mut annotations: &'n [Node<'v>];
        loop {
            next = self.nodes.pop_front()?;
            annotations = next.annotations();
            self.nodes.reserve(annotations.len());
            for node in annotations.iter().rev() {
                self.nodes.push_front(node);
            }
            if next.is_error() {
                return Some(next);
            }
        }
    }
}

pub struct TraverseErrors<'n, 'v> {
    nodes: VecDeque<&'n Node<'v>>,
}

impl<'n, 'v> Iterator for TraverseErrors<'n, 'v> {
    type Item = &'n Node<'v>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: &'n Node<'v>;
        let mut errors: &'n [Node<'v>];
        loop {
            next = self.nodes.pop_front()?;
            errors = next.errors();
            self.nodes.reserve(errors.len());
            for node in errors.iter().rev() {
                self.nodes.push_front(node);
            }
            if next.is_error() {
                return Some(next);
            }
        }
    }
}
