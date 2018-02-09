extern crate algorithms_with;

use algorithms_with::base;

use base::bag::*;
use base::queue::*;
use base::stack::*;
fn main() {
    /* test euclidean algorithm, binary search */
    assert_eq!(6, base::euclidean_algorithm(12, 30));  
    assert_eq!(3, base::binary_search(3,[1,3,5,7,9,35,54,99]));

    /* test struct Bag */
    let mut bag1 = Bag::new();
    assert_eq!(true, bag1.is_empty());
    bag1.add(3);
    bag1.add(7);
    bag1.add(9);
    assert_eq!(3, bag1.size());
    println!("bag1 = {:?}",bag1);

    /* test struct Queue */    
    let mut queue1 = Queue::new();
    assert_eq!(true, queue1.is_empty());
    queue1.enqueue(3);
    queue1.enqueue(4);
    queue1.enqueue(5);
    assert_eq!(3, queue1.size());
    assert_eq!(3, queue1.dequeue());
    println!("queue1 = {:?}",queue1);

    /* test struct Stack */    
    let mut stack1 = Stack::new();
    assert_eq!(true, stack1.is_empty());
    stack1.push(1);
    stack1.push(2);
    stack1.push(3);
    assert_eq!(3, stack1.size());
    assert_eq!(3, stack1.pop());
    println!("stack1 = {:?}",stack1);
}