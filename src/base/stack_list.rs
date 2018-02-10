use std::fmt::Debug;
use std::clone::Clone;
#[derive(Debug)]
pub struct Node<T> {
    pub item: T,
    pub next: Option<*mut Node<T>>,
}

#[derive(Debug)]
pub struct Stack<T> {
    top: Option<*mut Node<T>>,
    size: usize,
}

impl<T> Stack<T>
    where T: Debug + Clone {
    pub fn new() ->Stack<T> {
        Stack {
            top: None,
            size: 0,
        }
    }

    pub fn push(&mut self, node: *mut Node<T>) {
        match self.top {
            Some(ref mut p) => {
                unsafe {
                    (*node).next = Some(*p);
                }
                *p = node;
            },
            None => self.top = Some(node),
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> T {
        if self.top == None {
            panic!("stack is empty");
        }
        else {
            unsafe {
                let item = (*self.top.unwrap()).item.clone();
                self.top = (*self.top.unwrap()).next;
                self.size -= 1;
                println!("pop {:?}",item);
                return item;
            }
        } 
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_stack_list() {
        let mut stack1 = Stack::new();
        let mut node1 = Node { item: "aaa",next: None };
        let mut node2 = Node { item: "bbb",next: None };
        let mut node3 = Node { item: "ccc",next: None };
        stack1.push(&mut node1);
        stack1.push(&mut node2);
        stack1.push(&mut node3);
        assert_eq!("ccc", stack1.pop());
        assert_eq!("bbb", stack1.pop());
        assert_eq!("aaa", stack1.pop());

        assert_eq!(0, stack1.size());
        assert_eq!(true, stack1.is_empty());
    }
}