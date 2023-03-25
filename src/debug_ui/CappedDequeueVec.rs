use std::collections::vec_deque::Iter;
use std::collections::VecDeque;

pub struct CappedVecDeque<T> {
    container: VecDeque<T>,
    max_size: usize,
}

impl<T> CappedVecDeque<T> {
    pub fn new(max_size: usize) -> Self {
        Self {
            container: VecDeque::new(),
            max_size,
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.container.push_back(value);
        if self.container.len() > self.max_size {
            self.container.pop_front();
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.container.iter()
    }
}
