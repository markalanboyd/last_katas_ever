pub struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO Break out tests and add random tests.

    #[test]
    fn test_queue() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.length(), 1);
        assert_eq!(queue.peek(), Some(&1));
        let item = queue.dequeue();
        assert_eq!(item, 1);
        assert!(queue.is_empty())
    }
}
