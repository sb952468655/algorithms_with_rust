pub mod bag;
pub mod queue;
pub mod stack;
pub mod stack_list;

pub fn euclidean_algorithm(p: i32, q: i32) -> i32 {
    match q {
        0 => p,
        _ => {
            let r = p % q;
            euclidean_algorithm(q, r)
        },
    }
}

pub fn binary_search(key: i32, a: [i32;8]) -> i32 {

    let mut b: usize = 0;
    let mut e: usize = a.len()-1;

    while e >= b {
        let mid = b + (e - b) / 2;

        match a[mid] {
            v if v < key => b = mid + 1,
            v if v > key => e = mid - 1,
            _ => return key
        }
    }

    return -1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_euclidean_algorithm() {
        assert_eq!(6, euclidean_algorithm(12, 30));
    }
    #[test]
    fn test_binary_search() {
        assert_eq!(3, binary_search(3,[1,3,5,7,9,35,54,99]));
    }    
}