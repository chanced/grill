use std::collections::VecDeque;

use super::Evaluation;

pub struct Iter<'a> {
    eval: &'a Evaluation,
    queue: VecDeque<&'a Evaluation>,
}

impl<'a> Iter<'a> {
    pub fn new(eval: &'a Evaluation) -> Self {
        let mut queue = VecDeque::new();
        queue.push_front(eval);
        Self { eval, queue }
    }
    pub fn source(&self) -> &'a Evaluation {
        self.eval
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Evaluation;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(next) => {
                for n in next.nested.iter().rev() {
                    self.queue.push_front(n)
                }
                Some(next)
            }
            None => None,
        }
    }
}
