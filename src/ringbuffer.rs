use std::collections::VecDeque;

#[derive(Debug)]
pub struct RingBuffer<T> {
    capacity: usize,
    buffer: VecDeque<T>,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, item: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}
