pub struct Solution {}

impl Solution {
    pub fn sort_items(
        n: i32,
        m: i32,
        mut group: Vec<i32>,
        before_items: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut m = m as usize;
        for i in 0..group.len() {
            if group[i] == -1 {
                group[i] = m as i32;
                m += 1;
            }
        }

        let mut group_digraph = Digraph::with_capacity(m);
        for (i, &cur_group) in group.iter().enumerate() {
            for &pre in before_items[i].iter() {
                let before_group = group[pre as usize];
                if before_group == cur_group {
                    continue;
                }

                group_digraph.add_edge(before_group as usize, cur_group as usize);
            }
        }

        let mut ret = Vec::with_capacity(n as usize);
        let group_order = group_digraph.get_order();
        if group_order.is_empty() {
            return ret;
        }

        let mut item_digraph = Digraph::with_capacity(n as usize);
        for (i, pres) in before_items.iter().enumerate() {
            for &pre in pres.iter() {
                item_digraph.add_edge(pre as usize, i)
            }
        }

        let item_order = item_digraph.get_order();
        if item_order.is_empty() {
            return ret;
        }

        let mut group_items = std::collections::HashMap::with_capacity(m);
        for &item in item_order.iter() {
            (*group_items.entry(group[item]).or_insert(vec![])).push(item as i32)
        }

        for &g in group_order.iter() {
            if group_items.contains_key(&(g as i32)) {
                for &item in group_items[&(g as i32)].iter() {
                    ret.push(item);
                }
            }
        }
        ret
    }
}

pub struct Digraph {
    edges: Vec<Vec<usize>>,
    indegrees: Vec<usize>,
    capacity: usize,
    is_order_calculated: bool,
    order: Vec<usize>,
}

impl Digraph {
    pub fn with_capacity(capacity: usize) -> Digraph {
        let ret = Digraph {
            edges: vec![vec![]; capacity],
            indegrees: vec![0; capacity],
            capacity,
            is_order_calculated: false,
            order: vec![],
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
        self.indegrees[v] += 1;
    }

    pub fn get_order(&mut self) -> &[usize] {
        if self.is_order_calculated {
            return &self.order;
        }

        let mut queue = std::collections::VecDeque::new();
        for (i, &indegree) in self.indegrees.iter().enumerate() {
            if indegree == 0 {
                queue.push_back(i);
            }
        }

        while !queue.is_empty() {
            let front = queue.pop_front().unwrap();
            self.order.push(front);

            for &adj in self.edges[front].iter() {
                self.indegrees[adj] -= 1;
                if self.indegrees[adj] == 0 {
                    queue.push_back(adj);
                }
            }
        }

        if self.order.len() != self.capacity {
            self.order.clear();
        }

        self.is_order_calculated = true;
        return &self.order;
    }
}
