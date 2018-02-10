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
        self.size == 0
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bag() {
        let mut bag1 = Bag::new();
        assert_eq!(true, bag1.is_empty());
        bag1.add(3);
        bag1.add(7);
        bag1.add(9);
        assert_eq!(3, bag1.size());
    }
}