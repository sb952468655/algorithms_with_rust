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
        match self.size {
            0 => true,
            _ => false,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}