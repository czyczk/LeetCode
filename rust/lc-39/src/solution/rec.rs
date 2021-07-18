pub struct Solution {}

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        let mut queue = vec![];
        for &c in candidates.iter() {
            if c > target {
                break;
            }
            queue.push(c);
        }

        let mut rec_pack = RecPack {
            ans: vec![],
            trace: vec![],
        };
        rec_pack = backtrace_rec(&queue, target, 0, 0, rec_pack);

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
    mut rec_pack: RecPack,
) -> RecPack {
    if sum == target {
        rec_pack.ans.push(rec_pack.trace.clone());
        return rec_pack;
    }

    if sum > target {
        return rec_pack;
    }

    for (i, &c) in candidates[start_idx..].iter().enumerate() {
        sum += c;
        if sum > target {
            break;
        }

        rec_pack.trace.push(c);
        rec_pack = backtrace_rec(candidates, target, sum, i + start_idx, rec_pack);
        rec_pack.trace.pop();
        sum -= c;
    }

    return rec_pack;
}
