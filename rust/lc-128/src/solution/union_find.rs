pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut el_idx_map = std::collections::HashMap::new();
        let mut uf_capacity = 0;
        for &num in nums.iter() {
            if !el_idx_map.contains_key(&num) {
                el_idx_map.insert(num, uf_capacity);
                uf_capacity += 1;
            }

            if !el_idx_map.contains_key(&(num - 1)) {
                el_idx_map.insert(num - 1, uf_capacity);
                uf_capacity += 1;
            }
        }

        let mut uf = UnionFind::with_capacity(uf_capacity);
        for &num in nums.iter() {
            uf.union(
                *el_idx_map.get(&num).unwrap(),
                *el_idx_map.get(&(num - 1)).unwrap(),
            );
        }

        (*uf.sizes.iter().max().unwrap() - 1) as i32
    }
}

struct UnionFind {
    capacity: usize,
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            parents: (0..capacity).collect(),
            sizes: vec![1; capacity],
        }
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        if x >= self.capacity || y >= self.capacity {
            panic!();
        }

        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            return false;
        }

        self.parents[x] = parent_y;
        self.sizes[parent_y] += self.sizes[x];
        true
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x >= self.capacity {
            panic!();
        }

        let old_parent = self.parents[x];
        if x == old_parent {
            return x;
        }

        self.parents[x] = self.find(old_parent);
        self.parents[x]
    }
}
