pub struct Solution {}

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let n = machines.len();
        if n == 1 {
            return 0;
        }

        let sum = machines.iter().sum::<i32>();
        if sum as usize % n != 0 {
            return -1;
        }

        let avg = sum / n as i32;
        let max_over_avg = machines.iter().map(|&num| num - avg).max().unwrap();
        let mut left_sum = machines[0] - avg;

        let mut ans = 0;

        for i in 1..n - 1 {
            left_sum += machines[i] - avg;
            ans = ans.max(left_sum.abs());
        }

        ans.max(max_over_avg)
    }
}
