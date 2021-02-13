pub struct Solution {}

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        for i in 0..n {
            let num = nums[i];
            let idx = (num - 1) as usize % n;
            nums[idx] += n as i32;
        }

        let mut ret = vec![];
        for (i, &num) in nums.iter().enumerate() {
            if num <= n as i32 {
                ret.push((i + 1) as i32);
            }
        }

        ret
    }
}