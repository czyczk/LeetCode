pub struct Solution {}

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut last = nums[0];
        let mut max_length = 0;
        let mut length = 1;

        for i in 1..nums.len() {
            let num = nums[i];
            match num.cmp(&last) {
                std::cmp::Ordering::Greater => length += 1,
                _ => {
                    max_length = max_length.max(length);
                    length = 1;
                }
            }

            last = num;
        }

        return max_length.max(length);
    }
}
