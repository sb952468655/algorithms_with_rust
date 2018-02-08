extern crate algorithms_with;

use algorithms_with::base;

fn main() {
    let v = base::euclidean_algorithm(12, 30);
    println!("gcd = {}",v);
}