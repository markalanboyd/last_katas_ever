pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn length(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack: Stack<isize> = Stack::new();
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
    }

    #[test]
    fn test_push_and_peek() {
        let mut stack: Stack<isize> = Stack::new();
        stack.push(1);
        assert_eq!(stack.peek(), Some(&1));
    }

    #[test]
    fn test_push_and_length() {
        let mut stack: Stack<isize> = Stack::new();
        stack.push(1);
        assert_eq!(stack.length(), 1);
    }

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<isize> = Stack::new();
        assert!(stack.is_empty());
    }
}
