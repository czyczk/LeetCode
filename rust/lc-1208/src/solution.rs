pub struct Solution {}

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let n = s.len();
        let mut costs = Vec::with_capacity(n);
        let s = s.as_bytes();
        let t = t.as_bytes();

        let (mut idx_l, mut idx_r) = (0 as usize, 0 as usize);
        let mut remaining_cost = max_cost;

        while idx_r < n {
            costs.push((s[idx_r] as i32 - t[idx_r] as i32).abs());

            remaining_cost -= costs[idx_r];
            if remaining_cost < 0 {
                remaining_cost += costs[idx_l];
                idx_l += 1;
            }

            idx_r += 1;
        }

        return (idx_r - idx_l) as i32;
    }
}
