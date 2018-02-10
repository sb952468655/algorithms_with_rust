#[derive(Debug)]
pub struct Stack<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            items: Vec::new(),
            size: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
    }

    pub fn pop(&mut self) -> T {
        self.size -= 1;
        match self.items.pop() {
            Some(v) => v,
            None => panic!("stack is empty!"),
        } 
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stack() {
        let mut stack1 = Stack::new();
        assert_eq!(true, stack1.is_empty());
        stack1.push(1);
        stack1.push(2);
        stack1.push(3);
        assert_eq!(3, stack1.size());
        assert_eq!(3, stack1.pop());
        println!("stack1 = {:?}",stack1);
    }
}