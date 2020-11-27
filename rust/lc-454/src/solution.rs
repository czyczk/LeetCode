use std::collections::HashMap;

pub struct Solution { }

impl Solution {
    pub fn four_sum_count(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>, d: Vec<i32>) -> i32 {
        let mut num_solutions = 0;
        let mut a_b_sum_count_map = HashMap::new();

        for &num_from_a in a.iter() {
            for &num_from_b in b.iter() {
                let sum = num_from_a + num_from_b;

                let entry = a_b_sum_count_map.entry(sum).or_insert(0);
                *entry += 1;
            }
        }

        for &num_from_c in c.iter() {
            for &num_from_d in d.iter() {
                let sum_neg = -(num_from_c + num_from_d);

                if let Some(count) = a_b_sum_count_map.get(&sum_neg) {
                    num_solutions += count;
                }
            }
        }

        return num_solutions;
    }
}
