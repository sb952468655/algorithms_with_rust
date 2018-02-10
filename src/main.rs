extern crate algorithms_with;

use algorithms_with::base;

use base::bag;
use base::queue;
use base::stack;
use base::stack_list;

fn main() {
    let mut bag1 = bag::Bag::new();
    let mut queue1 = queue::Queue::new();
    let mut stack1 = stack::Stack::new();
    let mut stack_list1 = stack_list::Stack::new();
    bag1.add(2);
    queue1.enqueue(0);
    stack1.push(1);
    let mut node1 = stack_list::Node { item: 8,next: None };
    stack_list1.push(&mut node1);
}