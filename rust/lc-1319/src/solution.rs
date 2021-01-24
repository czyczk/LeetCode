pub struct Solution {}

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::with_capacity(n as usize);
        let mut remaining = 0;

        for c in connections.iter() {
            let is_accepted = uf.union(c[0] as usize, c[1] as usize);
            if !is_accepted {
                remaining += 1;
            }
        }

        return match (remaining - uf.area_cnt as i32 + 1).cmp(&0) {
            std::cmp::Ordering::Less => -1,
            _ => uf.area_cnt as i32 - 1,
        }
    }
}

pub struct UnionFind {
    parents: Vec<usize>,
    ranks: Vec<usize>,
    area_cnt: usize,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> UnionFind {
        UnionFind {
            parents: (0..capacity).collect(),
            ranks: vec![1; capacity],
            area_cnt: capacity,
        }
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return false;
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

        return true;
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
