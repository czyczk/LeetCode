pub struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let queries_n = queries.len();
        let mut ret = Vec::with_capacity(queries_n);

        if queries_n == 0 {
            return ret;
        }

        let equations_n = equations.len();
        let mut id = 0 as usize;
        let mut id_map = std::collections::HashMap::with_capacity(2 * equations_n);
        let mut union_find = UnionFind::with_capacity(2 * equations_n);

        for i in 0..equations_n {
            let x_str = &equations[i][0][..];
            let y_str = &equations[i][1][..];

            let x_id = *id_map.entry(x_str).or_insert(id);
            id += 1;
            let y_id = *id_map.entry(y_str).or_insert(id);
            id += 1;

            union_find.union(x_id, y_id, values[i]);
        }

        for i in 0..queries_n {
            let x_str = &queries[i][0][..];
            let y_str = &queries[i][1][..];

            let x_id = id_map.get(x_str);
            if x_id.is_none() {
                ret.push(-1.0);
                continue;
            }
            let x_id = *x_id.unwrap();

            let y_id = id_map.get(y_str);
            if y_id.is_none() {
                ret.push(-1.0);
                continue;
            }
            let y_id = *y_id.unwrap();

            ret.push(union_find.weight(x_id, y_id));
        }

        ret
    }
}

pub struct UnionFind {
    parents: Vec<usize>,
    weights: Vec<f64>,
    capacity: usize,
}

impl UnionFind {
    pub fn with_capacity(capacity: usize) -> UnionFind {
        let mut ret = UnionFind {
            parents: Vec::with_capacity(capacity),
            weights: vec![1.0; capacity],
            capacity,
        };

        for i in 0..capacity {
            ret.parents.push(i);
        }

        ret
    }

    pub fn union(&mut self, x: usize, y: usize, weight: f64) {
        if x >= self.capacity || y >= self.capacity {
            panic!("Out of capacity.");
        }

        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }

        self.parents[root_x] = root_y;
        self.weights[root_x] = weight * self.weights[y] / self.weights[x];
    }

    pub fn find(&mut self, x: usize) -> usize {
        if x >= self.parents.len() {
            panic!("Target {} not found.", x);
        }

        if x == self.parents[x] {
            return x;
        }

        let mut stack = vec![];
        let mut cursor = x;
        while cursor != self.parents[cursor] {
            stack.push(cursor);
            cursor = self.parents[cursor];
        }

        loop {
            match stack.pop() {
                None => break,
                Some(cursor) => {
                    let cursor_parent = self.parents[cursor];
                    self.parents[cursor] = self.parents[cursor_parent];
                    self.weights[cursor] *= self.weights[cursor_parent];
                }
            }
        }

        self.parents[x]
    }

    pub fn is_connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn weight(&mut self, x: usize, y: usize) -> f64 {
        if self.is_connected(x, y) {
            self.weights[x] / self.weights[y]
        } else {
            -1.0
        }
    }
}
