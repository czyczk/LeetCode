pub struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len() as i32;
        // Both inclusive
        let (mut start, mut end) = (0, len - 1);
        loop {
            if start > end || start < 0 || end >= len {
                break;
            }

            let i = (start + end) / 2;
            match nums[i as usize].cmp(&target) {
                std::cmp::Ordering::Equal => {
                    return i;
                }
                std::cmp::Ordering::Less => {
                    start = i + 1;
                }
                std::cmp::Ordering::Greater => {
                    end = i - 1;
                }
            }
        }

        return -1;
    }
}
