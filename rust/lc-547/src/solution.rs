pub struct Solution {}

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        if n == 0 {
            return 0;
        }

        let mut union_find = UnionFind::with_capacity(n);
        for (x, vertex_connections) in is_connected.iter().enumerate() {
            for (y, is_connected) in vertex_connections.iter().enumerate() {
                if x == y {
                    continue;
                }

                match is_connected {
                    1 => union_find.union(x, y),
                    _ => continue,
                }
            }
        }

        let mut result = 0;
        for x in 0..n {
            if x == union_find.find(x) {
                result += 1;
            }
        }

        result
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

    pub fn find(&mut self, x: usize) -> usize {
        if x >= self.capacity {
            panic!("{} not found.", x);
        }

        if x == self.parents[x] {
            return x;
        }

        let ori_parent = self.parents[x];
        self.parents[x] = self.find(ori_parent);

        return self.parents[x];
    }

    pub fn union(&mut self, x: usize, y: usize) {
        if x >= self.capacity || y >= self.capacity {
            panic!("Out of capacity.");
        }

        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }

        self.parents[root_x] = root_y;
    }
}
