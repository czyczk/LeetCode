pub struct Solution {}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut digraph = Digraph::with_capacity(num_courses as usize);

        for vertices in prerequisites.iter() {
            digraph.add_edge(vertices[1] as usize, vertices[0] as usize);
        }

        digraph
            .get_order()
            .into_iter()
            .map(|num| *num as i32)
            .collect()
    }
}

pub struct Digraph {
    edges: Vec<Vec<usize>>,
    visited: Vec<VertexStatus>,
    capacity: usize,
    is_order_calculated: bool,
    is_valid: bool,
    order: Vec<usize>,
}

impl Digraph {
    pub fn with_capacity(num_vertices: usize) -> Digraph {
        let ret = Digraph {
            edges: vec![vec![]; num_vertices],
            visited: vec![VertexStatus::Pending; num_vertices],
            capacity: num_vertices,
            is_order_calculated: false,
            is_valid: true,
            order: vec![0; num_vertices],
        };

        ret
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u >= self.capacity || v >= self.capacity {
            panic!("Out of capacity.");
        }

        if self.is_order_calculated {
            panic!("Order calculated.");
        }

        self.edges[u].push(v);
    }

    fn dfs(&mut self, u: usize, order_idx: &mut usize) {
        self.visited[u] = VertexStatus::Visiting;

        let adjacents = self.edges[u].clone();
        for &v in adjacents.iter() {
            match self.visited[v] {
                VertexStatus::Pending => {
                    self.dfs(v, order_idx);
                    if !self.is_valid {
                        return;
                    }
                }
                VertexStatus::Visiting => {
                    self.is_valid = false;
                    return;
                }
                _ => {}
            }
        }

        self.visited[u] = VertexStatus::Visited;
        self.order[*order_idx] = u;
        *order_idx = order_idx.saturating_sub(1);
    }

    pub fn get_order(&mut self) -> &[usize] {
        if self.is_order_calculated {
            return &self.order;
        }

        let mut order_idx = self.capacity - 1;
        for i in 0..self.capacity {
            if !self.is_valid {
                self.order.clear();
                break;
            }

            if self.visited[i] == VertexStatus::Pending {
                self.dfs(i, &mut order_idx);
            }
        }

        self.is_order_calculated = true;
        return &self.order;
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum VertexStatus {
    Pending,
    Visiting,
    Visited,
}
