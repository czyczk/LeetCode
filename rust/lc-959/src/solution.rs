pub struct Solution {}

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        }

        let mut uf = UnionFind::with_capacity(n * n * 4);

        for (i, line) in grid.iter().enumerate() {
            for (j, ch) in line.bytes().enumerate() {
                match ch {
                    b'/' => {
                        uf.union(to_uf_idx(n, i, j, 0), to_uf_idx(n, i, j, 3));
                        uf.union(to_uf_idx(n, i, j, 1), to_uf_idx(n, i, j, 2));
                    }
                    b'\\' => {
                        uf.union(to_uf_idx(n, i, j, 0), to_uf_idx(n, i, j, 1));
                        uf.union(to_uf_idx(n, i, j, 2), to_uf_idx(n, i, j, 3));
                    }
                    _ => {
                        uf.union(to_uf_idx(n, i, j, 0), to_uf_idx(n, i, j, 1));
                        uf.union(to_uf_idx(n, i, j, 1), to_uf_idx(n, i, j, 2));
                        uf.union(to_uf_idx(n, i, j, 2), to_uf_idx(n, i, j, 3));
                    }
                }

                // Union right
                if j < n - 1 {
                    uf.union(to_uf_idx(n, i, j, 1), to_uf_idx(n, i, j + 1, 3))
                }

                // Union down
                if i < n - 1 {
                    uf.union(to_uf_idx(n, i, j, 2), to_uf_idx(n, i + 1, j, 0))
                }
            }
        }

        uf.area_cnt as i32
    }
}

pub fn to_uf_idx(n: usize, i: usize, j: usize, sub_idx: usize) -> usize {
    ((i * n) + j) * 4 + sub_idx
}

pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    area_cnt: usize,
    capacity: usize,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> UnionFind {
        UnionFind {
            parents: (0..capacity).collect(),
            ranks: vec![1; capacity],
            area_cnt: capacity,
            capacity,
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return 
        }

        let rank_x = self.ranks[root_x];
        let rank_y = self.ranks[root_y];

        match rank_x.cmp(&rank_y) {
            std::cmp::Ordering::Less => {
                self.parents[root_x] = root_y;
                self.ranks[root_y] += rank_x;
            }
            _ => {
                self.parents[root_y] = root_x;
                self.ranks[root_x] += rank_y;
            }
        }

        self.area_cnt -= 1;
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x >= self.capacity {
            panic!("Out of capacity.");
        }

        let ori_parent = self.parents[x];
        if ori_parent == x {
            return x
        }

        let new_parent = self.find(ori_parent);
        self.parents[x] = new_parent;
        return new_parent;
    }
}