use std::cmp::Ordering;

pub struct Solution { }

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 2 {
            return n as i32;
        }

        let (mut up, mut down) = (1, 1);

        for i in 1..n {
            match nums[i - 1].cmp(&nums[i]) {
                Ordering::Less => up = down + 1,
                Ordering::Greater => down = up + 1,
                _ => {},
            }
        }

        return up.max(down);
    }
}