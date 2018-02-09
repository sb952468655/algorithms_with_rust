#[derive(Debug)]
pub struct Bag<T> {
    items: Vec<T>,
    size: usize,
}

impl<T> Bag<T> {
    pub fn add(&mut self,item: T) {
        self.items.push(item);
        self.size += 1; 
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

    pub fn new() -> Bag<T> {
        Bag {
            items: Vec::new(),
            size: 0,
        }
    }
}