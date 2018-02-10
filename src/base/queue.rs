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
    fn test_queue() {
        let mut queue1 = Queue::new();
        assert_eq!(true, queue1.is_empty());
        queue1.enqueue(3);
        queue1.enqueue(4);
        queue1.enqueue(5);
        assert_eq!(3, queue1.size());
        assert_eq!(3, queue1.dequeue());
        println!("queue1 = {:?}",queue1);
    }
}