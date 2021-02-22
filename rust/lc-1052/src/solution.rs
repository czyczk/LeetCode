pub struct Solution {}

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let x = x as usize;
        let n = customers.len();
        let mut num_satisfied = 0;

        let mut max_customers_kept = 0;
        for i in 0..x {
            max_customers_kept += customers[i] * grumpy[i];
            num_satisfied += customers[i] * (1 - grumpy[i]);
        }
        let mut customers_kept = max_customers_kept;

        for i in x..n {
            num_satisfied += customers[i] * (1 - grumpy[i]);
            let candidate_ult_start = i + 1 - x;
            customers_kept += customers[i] * grumpy[i] - customers[candidate_ult_start - 1] * grumpy[candidate_ult_start - 1];
            max_customers_kept = max_customers_kept.max(customers_kept);
        }

        num_satisfied + max_customers_kept
    }
}
