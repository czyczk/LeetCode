pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut slow, mut fast) = (0, 0);
        let len = nums.len();

        while fast < len {
            let num_fast = nums[fast];
            if num_fast != 0 {
                nums[slow] = num_fast;
                slow += 1;
            }

            fast += 1;
        }

        while slow < len {
            nums[slow] = 0;
            slow += 1;
        }
    }
}
