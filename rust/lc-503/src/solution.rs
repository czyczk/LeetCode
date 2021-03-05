pub struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ret = vec![-1; n];
        let mut stack = vec![];

        if n == 0 {
            return ret;
        }

        for i in 0..n * 2 - 1 {
            let num = nums[i % n];
            while !stack.is_empty() && nums[*stack.last().unwrap() % n] < num {
                ret[stack.pop().unwrap() % n] = num;
            }

            stack.push(i);
        }

        ret
    }
}
