
pub fn euclidean_algorithm(p: i32, q: i32) -> i32 {
    match q {
        0 => p,
        _ => {
            let r = p % q;
            println!("p = {}, q = {}, p % q = {}", p, q, r);
            euclidean_algorithm(q, r)
        },
    }
}