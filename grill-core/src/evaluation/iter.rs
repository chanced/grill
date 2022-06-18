use std::collections::VecDeque;

use super::Evaluation;

pub struct Iter<'a, I> {
    eval: &'a Evaluation<I>,
    queue: VecDeque<Evaluation<I>>,
}

impl<'a, I> Iter<'a, I> {
    pub fn new(eval: &'a Evaluation<I>) -> Self {
        let queue = VecDeque::new();
        queue.push_front(eval);
        Self { eval, queue }
    }
    pub fn source(&self) -> &'a Evaluation<I> {
        self.eval
    }
}

impl<'a, I> Iterator for Iter<'a, I> {
    type Item = Evaluation<I>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.queue.pop_front() {
            Some(next) => {
                for n in next.nested {
                    self.queue.push_front(n)
                }
                Some(next)
            }
            None => None,
        }
    }
}
