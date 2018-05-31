use std::fmt::Debug;
use std::collections::VecDeque;

/// Ring buffer data structure tracks a fixed size of function calls.
/// Behaves like a FIFO queue.
#[derive(Debug)]
pub struct RingBuffer<T> {
    data: VecDeque<T>,
    capacity: usize,
}

impl<T: Debug> RingBuffer<T> {
    /// Initialize a new ring buffer with a given capacity.
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            data: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Appends an element to the end of the buffer.
    /// If the buffer is filled to capacity, the oldest element is removed.
    pub fn enqueue(&mut self, item: T) {
        if self.len() > self.capacity {
            unreachable!();
        }
        if self.len() == self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(item);
    }

    /// Dequeues an item from the front of the buffer.
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    /// Returns the number of items in the buffer.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns an iterator over the buffer contents.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns a slice of the buffer contents.
    pub fn as_slice(&self) -> &[T] {
        let (front, back) = self.data.as_slices();
        assert!(back.is_empty(), "elements are only inserted at the back");
        front
    }
}

#[cfg(test)]
mod test_ring_buffer {
    use super::RingBuffer;

    #[test]
    fn initialize() {
        let mut buf: RingBuffer<usize> = RingBuffer::new(4);
        assert_eq!(buf.len(), 0);
        assert_eq!(buf.capacity, 4);
        assert_eq!(buf.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue_fifo() {
        let capacity = 10;
        let mut buf: RingBuffer<usize> = RingBuffer::new(capacity);
        for i in 0..capacity {
            buf.enqueue(i);
        }
        for i in 0..capacity {
            assert_eq!(buf.dequeue(), Some(i));
        }
        assert_eq!(buf.dequeue(), None);
    }

    #[test]
    fn enqueue_dequeue_overwrite() {
        let mut buf: RingBuffer<usize> = RingBuffer::new(10);
        for x in 0..15 {
            buf.enqueue(x);
        }
        assert_eq!(buf.len(), 10);
        let mut contents = Vec::new();
        while let Some(x) = buf.dequeue() {
            contents.push(x);
        }
        assert_eq!(contents, vec![5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
        assert_eq!(buf.len(), 0);
    }

    #[test]
    fn iter() {
        let mut buf: RingBuffer<usize> = RingBuffer::new(10);
        for x in 0..10 {
            buf.enqueue(x);
        }
        for (i, &x) in buf.iter().enumerate() {
            assert_eq!(x, i);
        }
    }

    #[test]
    fn as_slice() {
        let mut buf: RingBuffer<&str> = RingBuffer::new(5);
        let strings = ["apple", "banana", "carrot"];
        for s in strings.iter() {
            buf.enqueue(s);
        }
        let slice = buf.as_slice();
        for (&actual, &expected) in slice.iter().zip(strings.iter()) {
            assert_eq!(actual, expected);
        }
    }
}
