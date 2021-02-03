pub struct Solution {}

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut sum = 0;
        let k = k as usize;

        let mut max_average = std::f64::MIN;

        for (i, &num) in nums.iter().enumerate() {
            sum += num;

            if i < k - 1 {
                continue;
            }

            max_average = max_average.max(sum as f64 / k as f64);
            sum -= nums[i + 1 - k];
        }

        max_average
    }
}