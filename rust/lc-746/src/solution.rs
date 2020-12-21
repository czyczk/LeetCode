pub struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut cost_prev = cost[0];
        let mut cost_cur = cost[1];

        for i in 2..n {
            let new_cost_cur = cost_prev.min(cost_cur) + cost[i];
            cost_prev = cost_cur;
            cost_cur = new_cost_cur;
        }

        return cost_prev.min(cost_cur);
    }
}

