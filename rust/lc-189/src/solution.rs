pub struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        if n == 0 {
            return;
        }

        let k = k as usize % n;
        let slice_to_be_moved = nums[n - k..n].to_owned();

        for i in (k..n).rev() {
            nums[i] = nums[i - k];
        }

        for i in 0..k {
            nums[i] = slice_to_be_moved[i];
        }
    }
}
