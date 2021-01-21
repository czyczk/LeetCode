pub struct Solution {}

impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges.len();
        let mut union_find = UnionFind::with_capacity(n);
        let mut ret = vec![0, 0];

        for edge in edges.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let is_accepted = union_find.union(u - 1, v - 1);
            if !is_accepted {
                ret[0] = u as i32;
                ret[1] = v as i32;
            }
        }

        ret
    }
}

pub struct UnionFind {
    parents: Vec<usize>,
    capacity: usize,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> UnionFind {
        let mut ret = UnionFind {
            parents: Vec::with_capacity(capacity),
            capacity,
        };

        for i in 0..capacity {
            ret.parents.push(i);
        }

        ret
    }

    pub fn union(&mut self, u: usize, v: usize) -> bool {
        if u >= self.capacity || v >= self.capacity {
            panic!("Out of capacity.");
        }

        let root_u = self.find(u);
        let root_v = self.find(v);
        if root_u == root_v {
            return false;
        }

        self.parents[root_u] = root_v;
        return true;
    }

    pub fn find(&mut self, u: usize) -> usize {
        if u >= self.capacity {
            panic!("Out of capacity.");
        }

        let ori_parent = self.parents[u];
        if ori_parent == u {
            return u;
        }

        self.parents[u] = self.find(ori_parent);
        return self.parents[u];
    }
}
