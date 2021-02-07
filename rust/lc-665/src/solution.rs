pub struct Solution {}

impl Solution {
    pub fn check_possibility(mut nums: Vec<i32>) -> bool {
        let mut is_drop_detected = false;
        let n = nums.len();

        for i in 0..n - 1 {
            let (x, y) = (nums[i], nums[i + 1]);

            if x > y {
                if is_drop_detected {
                    return false;
                }

                is_drop_detected = true;

                if i > 0 && y < nums[i - 1] {
                    nums[i + 1] = x;
                }
            }
        }

        true
    }
}
