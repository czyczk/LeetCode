pub struct Solution {}

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut num_patches = 0;
        let len = nums.len();

        let mut x = 1 as i64;
        let mut idx = 0;

        while x <= n as i64 {
            if idx < len && nums[idx] as i64 <= x {
                x += nums[idx] as i64;
                idx += 1;
            } else {
                x *= 2;
                num_patches += 1;
            }
        }

        return num_patches;
    }
}