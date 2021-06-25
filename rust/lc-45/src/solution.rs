pub struct Solution {}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 0;
        }

        // Prevent index overflow
        let mut coverage = (nums[0] as usize).min(n - 1);
        let mut cur = 0usize;
        let mut num_steps = 0;

        loop {
            if coverage == n - 1 {
                return num_steps + 1;
            }

            let mut max_coverage = 0;
            let mut best_candidate = cur + 1;
            for i in cur + 1..=coverage {
                let new_coverage = (nums[i] as usize) + i;
                if new_coverage > max_coverage {
                    max_coverage = new_coverage;
                    best_candidate = i;
                }
            }

            coverage = max_coverage.min(n - 1);
            cur = best_candidate;

            num_steps += 1;
        }
    }
}
