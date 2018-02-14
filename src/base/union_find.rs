#[derive(Debug)]
struct UF {
    id: Vec<i32>,
    count: usize,
}

impl UF {
    pub fn new(n: usize) -> UF {
        let mut  id = Vec::with_capacity(n);
        for v in 0..n as i32 {
            id.push(v);
        }

        UF {
            id,
            count: n,
        }

    }

    pub fn union(&mut self, p: usize, q: usize) {
        let p_id = self.find(p);
        let q_id = self.find(q);
        if p_id == q_id { return; }
        for i in 0..self.id.len() {
            if self.id[i] == p_id {
                self.id[i] = q_id;
            }
        }
        self.count -= 1;

    }

    pub fn find(&self, p: usize) -> i32 {
        self.id[p]
    }

    pub fn connected(&self, p: usize, q: usize) -> bool {
        self.id[p] == self.id[q]
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_union_find() {
        let mut union1 = UF::new(16);
        union1.union(1, 2);
        union1.union(2, 3);
        union1.union(5, 6);
        union1.union(7, 8);

        assert_eq!(3, union1.find(1));
        assert_eq!(6, union1.find(5));
        assert_eq!(8, union1.find(7));

        assert!(union1.connected(1,2));
        assert_eq!(12, union1.count());
    }
}