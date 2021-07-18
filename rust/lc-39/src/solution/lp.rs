pub struct Solution {}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let mut ret = vec![];
        let mut stack = vec![];
        {
            let mut q = std::collections::VecDeque::new();
            for &c in candidates.iter() {
                if c <= target {
                    q.push_back(c);
                }
            }
            stack.push(StackStuff {
                queue: q,
                start_idx: std::usize::MAX,
            });
        }

        let mut trace = vec![];
        let mut sum = 0;

        while !stack.is_empty() {
            if stack.last().unwrap().queue.is_empty() {
                stack.pop();
                if trace.is_empty() {
                    break;
                }
                sum -= trace.last().unwrap();
                trace.pop();
                continue;
            }

            let ss = stack.last_mut().unwrap();
            let c = ss.queue.pop_front().unwrap();
            sum += c;
            trace.push(c);
            ss.start_idx = ss.start_idx.overflowing_add(1).0;

            if sum == target {
                ret.push(trace.clone());
                sum -= trace.last().unwrap();
                trace.pop();
            } else {
                let mut new_q = std::collections::VecDeque::new();
                for &c in candidates[ss.start_idx..].iter() {
                    if c > target - sum {
                        break;
                    }
                    new_q.push_back(c);
                }
                let new_start_idx = ss.start_idx.overflowing_sub(1).0;
                stack.push(StackStuff {
                    queue: new_q,
                    start_idx: new_start_idx,
                });
            }
        }

        ret
    }
}

struct StackStuff {
    queue: std::collections::VecDeque<i32>,
    start_idx: usize,
}
