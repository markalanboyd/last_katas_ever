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

    pub fn dequeue(&mut self) -> Option<T> {
        match self.queue.get(0) {
            Some(_) => Some(self.queue.remove(0)),
            None => None,
        }
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

    #[test]
    fn test_new_queue_is_empty() {
        let queue: Queue<isize> = Queue::new();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_enqueue_and_peek() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.peek(), Some(&1));
    }

    #[test]
    fn test_enqueue_and_length() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.length(), 1);
    }

    #[test]
    fn test_enqueue_and_dequeue() {
        let mut queue: Queue<isize> = Queue::new();
        queue.enqueue(1);
        assert_eq!(queue.dequeue(), Some(1));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_dequeue_empty() {
        let mut queue: Queue<isize> = Queue::new();
        assert_eq!(queue.dequeue(), None);
    }
}
