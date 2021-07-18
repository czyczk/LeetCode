pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        candidates.sort_unstable();
        let mut stack = vec![];
        {
            let mut end_idx = 0;
            for &c in candidates.iter() {
                if c > target {
                    break;
                }
                end_idx += 1;
            }

            stack.push(StackStuff {
                start_idx: 0,
                end_idx,
                visited: std::collections::HashSet::new(),
            })
        }

        let mut trace = vec![];
        let mut sum = 0;

        while !stack.is_empty() {
            if stack.last().unwrap().start_idx >= stack.last().unwrap().end_idx {
                stack.pop();
                if trace.is_empty() {
                    break;
                }

                sum -= trace.pop().unwrap();
                continue;
            }

            let ss = stack.last_mut().unwrap();

            let c = candidates[ss.start_idx];
            ss.start_idx += 1;
            if ss.visited.contains(&c) {
                continue;
            }

            trace.push(c);
            sum += c;
            ss.visited.insert(c);

            if sum == target {
                ret.push(trace.clone());
                sum -= trace.pop().unwrap();
            } else {
                let mut end_idx = ss.start_idx;
                while end_idx < candidates.len() {
                    if candidates[end_idx] > target - sum {
                        break;
                    }

                    end_idx += 1;
                }

                let start_idx = ss.start_idx;
                stack.push(StackStuff {
                    start_idx,
                    end_idx,
                    visited: std::collections::HashSet::new(),
                });
            }
        }

        ret
    }
}

struct StackStuff {
    start_idx: usize,
    end_idx: usize,
    visited: std::collections::HashSet<i32>,
}
