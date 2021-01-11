pub struct Solution {}

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let n = s.len();
        match n {
            0 | 1 => return s,
            _ => {}
        }

        let s_chars = s.as_bytes();
        let mut ret = vec![0 as u8; n];

        let mut union_find = UnionFind::with_capacity(n);

        for pair in pairs.iter() {
            union_find.union(pair[0] as usize, pair[1] as usize);
        }

        let connected_areas = union_find.get_connected_areas();
        for area in connected_areas.iter() {
            match area.len() {
                1 => {
                    let idx = area[0];
                    ret[idx] = s_chars[idx];
                }
                _ => {
                    let mut area_chars = Vec::with_capacity(area.len());
                    for &idx in area.iter() {
                        area_chars.push(s_chars[idx]);
                    }

                    area_chars.sort_unstable();
                    for (i, &idx) in area.iter().enumerate() {
                        ret[idx] = area_chars[i];
                    }
                }
            }
        }

        String::from_utf8_lossy(&ret).into()
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

    pub fn union(&mut self, x: usize, y: usize) {
        if x >= self.capacity || y >= self.capacity {
            panic!("Out of range");
        }

        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        self.parents[root_x] = root_y;
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x >= self.capacity {
            panic!("Out of range");
        }

        let parent = self.parents[x];
        if parent == x {
            return x;
        }

        self.parents[x] = self.find(parent);

        self.parents[x]
    }

    pub fn get_connected_areas(&mut self) -> Vec<Vec<usize>> {
        let mut ret = vec![];
        let mut idx_mapping = std::collections::HashMap::with_capacity(self.capacity);

        for i in 0..self.capacity {
            let root = self.find(i);
            let max_idx = idx_mapping.len();
            let idx = *idx_mapping.entry(root).or_insert(max_idx);

            if idx == max_idx {
                ret.push(vec![i]);
            } else {
                ret[idx].push(i);
            }
        }

        ret
    }
}
