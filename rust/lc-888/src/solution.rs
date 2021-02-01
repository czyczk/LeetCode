pub struct Solution {}

impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_sum = 0;
        let mut b_sum = 0;
        let mut a_map = std::collections::HashSet::new();
        let mut b_map = std::collections::HashSet::new();

        for &bar in a.iter() {
            a_sum += bar;
            a_map.insert(bar);
        }

        for &bar in b.iter() {
            b_sum += bar;
            b_map.insert(bar);
        }

        // Mean
        let diff = a_sum - (a_sum + b_sum) / 2;
        let mut a_bar = 1;
        let mut b_bar = 1;
        match diff.cmp(&0) {
            std::cmp::Ordering::Greater => a_bar += diff,
            _ => b_bar -= diff,
        }

        while !a_map.contains(&a_bar) || !b_map.contains(&b_bar) {
            a_bar += 1;
            b_bar += 1;
        }

        return vec![a_bar, b_bar]
    }
}