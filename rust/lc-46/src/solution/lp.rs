pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut ans = vec![];
        let mut trace = vec![];
        let mut used = vec![false; n];

        let mut stack = vec![StackStuff { i: 0 }];
        while !stack.is_empty() {
            if stack.last().unwrap().i == n {
                stack.pop().unwrap();
                if trace.is_empty() {
                    break;
                }
                used[trace.pop().unwrap()] = false;
                continue;
            }

            let ss = stack.last_mut().unwrap();

            if used[ss.i] {
                ss.i += 1;
                continue;
            }

            trace.push(ss.i);
            used[ss.i] = true;
            ss.i += 1;

            if trace.len() == n {
                ans.push(trace.iter().map(|&i| nums[i]).collect());
                used[trace.pop().unwrap()] = false;
            } else {
                stack.push(StackStuff { i: 0 })
            }
        }

        return ans;
    }
}

struct StackStuff {
    i: usize,
}
