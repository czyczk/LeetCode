pub struct Solution {}

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.retain(|&c| c <= target);
        candidates.sort();
        let mut rec_pack = RecPack {
            ans: vec![],
            trace: vec![],
        };

        rec_pack = backtrace_rec(
            &candidates,
            target,
            0,
            0,
            candidates.len(),
            std::collections::HashSet::new(),
            rec_pack,
        );

        rec_pack.ans
    }
}

struct RecPack {
    ans: Vec<Vec<i32>>,
    trace: Vec<i32>,
}

fn backtrace_rec(
    candidates: &[i32],
    target: i32,
    mut sum: i32,
    start_idx: usize,
    end_idx: usize,
    mut visited: std::collections::HashSet<i32>,
    mut rec_pack: RecPack,
) -> RecPack {
    if sum > target {
        return rec_pack;
    }

    if sum == target {
        rec_pack.ans.push(rec_pack.trace.clone());
        return rec_pack;
    }

    if start_idx >= end_idx {
        return rec_pack;
    }

    for (i, &c) in candidates[start_idx..end_idx].iter().enumerate() {
        if visited.contains(&c) {
            continue;
        }

        sum += c;
        rec_pack.trace.push(c);
        visited.insert(c);

        let new_start_idx = start_idx + i + 1;
        let mut new_end_idx = new_start_idx;
        while new_end_idx < candidates.len() {
            if candidates[new_end_idx] > target - sum {
                break;
            }

            new_end_idx += 1;
        }

        rec_pack = backtrace_rec(
            candidates,
            target,
            sum,
            new_start_idx,
            new_end_idx,
            std::collections::HashSet::new(),
            rec_pack,
        );

        sum -= rec_pack.trace.pop().unwrap();
    }

    return rec_pack;
}
