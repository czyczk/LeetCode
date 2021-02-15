pub struct Solution {}

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let n = row.len() / 2;

        let mut uf = UnionFind::with_capacity(n);
        for i in (0..row.len()).step_by(2) {
            uf.union((row[i] / 2) as usize, (row[i + 1] / 2) as usize);
        }

        (n - uf.area_cnt) as i32
    }
}

pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    area_cnt: usize,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            parents: (0..capacity).collect(),
            ranks: vec![0; capacity],
            area_cnt: capacity,
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        let rank_x = self.ranks[root_x];
        let rank_y = self.ranks[root_y];
        if rank_x < rank_y {
            self.parents[root_x] = root_y;
            self.ranks[root_y] += rank_x;
        } else {
            self.parents[root_y] = root_x;
            self.ranks[root_x] += rank_y;
        }

        self.area_cnt -= 1;
    }

    pub fn find(&mut self, x: usize) -> usize {
        let ori_parent = self.parents[x];
        if ori_parent == x {
            return ori_parent;
        }

        let new_parent = self.find(ori_parent);
        self.parents[x] = new_parent;
        return new_parent;
    }
}