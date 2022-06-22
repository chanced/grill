use std::collections::VecDeque;

use super::Annotation;

pub struct Iter<'a> {
    eval: &'a Annotation,
    queue: VecDeque<&'a Annotation>,
}

impl<'a> Iter<'a> {
    pub fn new(eval: &'a Annotation) -> Self {
        let queue = VecDeque::new();
        queue.push_front(eval);
        Self { eval, queue }
    }
    pub fn source(&self) -> &'a Annotation {
        self.eval
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Annotation;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(next) => {
                for n in next.nested.iter() {
                    self.queue.push_front(n)
                }
                Some(next)
            }
            None => None,
        }
    }
}
