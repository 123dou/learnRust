use std::iter::FromIterator;

pub struct UnionFind {
    size: Vec<usize>,
    id: Vec<usize>,
}

impl UnionFind {
    pub fn new(len: usize) -> UnionFind {
        let size: Vec<usize> = (0..=len).collect::<Vec<_>>();
        let id = size.clone();
        UnionFind { size, id }
    }

    pub fn find(&self, p: usize) -> Option<usize> {
        if p > self.size.len() {
            return None;
        }
        let mut res = p;
        while self.id[res] != res {
            res = self.id[res];
        }
        Some(res)
    }

    pub fn is_conn(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    pub fn union(&mut self, p: usize, q: usize) -> bool {
        if let (Some(p_root), Some(q_root)) = (self.find(p), self.find(q)) {
            if self.size[p_root] < self.size[q_root] {
                self.id[p_root] = q_root;
                self.size[q_root] += self.size[p_root];
            } else {
                self.id[q_root] = p_root;
                self.size[p_root] += self.size[q_root];
            }
            return true;
        }
        false
    }
}
