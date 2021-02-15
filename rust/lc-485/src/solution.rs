pub struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut cur_len = 0;
        let mut max_len = 0;

        for num in nums {
            match num {
                1 => cur_len += 1,
                _ => {
                    max_len = max_len.max(cur_len);
                    cur_len = 0;
                }
            }
        }

        max_len.max(cur_len)
    }
}
