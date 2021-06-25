pub struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut coverage = 0;
        let mut i = 0;

        while i <= coverage {
            coverage = coverage.max(i + nums[i] as usize);
            if coverage >= nums.len() - 1 {
                return true;
            }

            i += 1;
        }

        return false;
    }
}
