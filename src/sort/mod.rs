pub fn selection_sort(a: &mut [i32]) {
    for i in 0..a.len()-1 {
        let mut min = i;
        for j in (i+1)..a.len() {
            if a[j] < a[min] {
                min = j;
            }
        }
        exchange(a, i, min);
    }
}

pub fn insertion_sort(a: &mut [i32]) {
    for i in 1..a.len() {
        for j in (1..i+1).rev() {
            if a[j] < a[j-1] {
                exchange(a, j, j-1);
            }
        }
    }
}

pub fn shell_sort(a: &mut [i32]) {
    let n = a.len();
    let mut h = 1;
    while h < n/3 { h = 3*h + 1; }
    while h >= 1 {
        for i in h..n {
            for j in (h..i+1).filter(|s| {
                *s >= h && (i - *s)% h == 0 
            }).rev() {
                if a[j] < a[j - h] { exchange(a, j, j-h); }
            }
        }
        h = h/3;
    }
}

fn exchange(a: &mut [i32], i: usize, j: usize) {
    let temp = a[i];
    a[i] = a[j];
    a[j] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_selection_sort() {
        let mut a = [3,5,8,2,7,9,23,11,16,78];
        selection_sort(&mut a);
        assert_eq!([2,3,5,7,8,9,11,16,23,78], a);
    } 

    #[test]
    fn test_insertion_sort() {
        let mut a = [3,5,8,2,7,9,23,11,16,78];
        insertion_sort(&mut a);
        assert_eq!([2,3,5,7,8,9,11,16,23,78], a);                                                                                              
    }

    #[test]
    fn test_shell_sort() {
        let mut a = [3,5,8,2,7,9,23,11,16,78];
        shell_sort(&mut a);
        assert_eq!([2,3,5,7,8,9,11,16,23,78], a);                                                                                              
    }      
}