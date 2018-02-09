#[derive(Debug)]
pub struct Queue<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            items: Vec::new(),
            size: 0,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
        self.size += 1;
    }

    pub fn dequeue(&mut self) -> T {
        self.size -= 1;
        self.items.remove(0)
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