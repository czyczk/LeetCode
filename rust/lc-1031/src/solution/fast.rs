pub struct Solution {}

impl Solution {
    pub fn max_sum_two_no_overlap(mut nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let n = nums.len();
        let (first_len, second_len) = (first_len as usize, second_len as usize);

        for i in 1..n {
            nums[i] += nums[i - 1];
        }

        let mut ret = nums[first_len + second_len - 1];
        let mut first_max = nums[first_len - 1];
        let mut second_max = nums[second_len - 1];

        for i in first_len + second_len..n {
            first_max = first_max.max(nums[i - second_len] - nums[i - first_len - second_len]);
            second_max = second_max.max(nums[i - first_len] - nums[i - first_len - second_len]);
            ret = ret.max(
                (first_max + nums[i] - nums[i - second_len])
                    .max(second_max + nums[i] - nums[i - first_len]),
            );
        }

        ret
    }
}
