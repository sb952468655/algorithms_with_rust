extern crate algorithms_with;

use algorithms_with::base;

use base::bag::*;

fn main() {

    assert_eq!(6, base::euclidean_algorithm(12, 30));  
    assert_eq!(3, base::binary_search(3,[1,3,5,7,9,35,54,99]));

    let mut bag1 = Bag::new();
    let mut bag2 = Bag::new();
    assert_eq!(true, bag1.is_empty());
    assert_eq!(true, bag2.is_empty());
    bag1.add(3);
    bag2.add("ddd");
    assert_eq!(1, bag1.size());
    assert_eq!(1, bag2.size());
    println!("{:?}",bag1);
    println!("{:?}",bag2);
}