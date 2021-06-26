pub struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = Vec::with_capacity(nums.len());

        for num in nums.into_iter() {
            if num < 0 {
                stack.push(num);
                continue;
            }

            loop {
                match stack.last() {
                    None => break,
                    Some(&stack_top) => {
                        if -stack_top > num {
                            break;
                        } else {
                            stack.pop();
                            ret.push(stack_top * stack_top);
                        }
                    }
                }
            }

            ret.push(num * num);
        }

        while let Some(&rem) = stack.last() {
            stack.pop();
            ret.push(rem * rem);
        }

        return ret;
    }
}
