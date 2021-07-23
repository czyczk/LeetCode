pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut rec_pack = RecPack {
            ans: vec![],
            trace: vec![],
            used: vec![false; nums.len()],
        };

        rec_pack = backtrace_rec(&nums, rec_pack);

        return rec_pack.ans;
    }
}

struct RecPack {
    ans: Vec<Vec<i32>>,
    trace: Vec<i32>,
    used: Vec<bool>,
}

fn backtrace_rec(nums: &[i32], mut rec_pack: RecPack) -> RecPack {
    if rec_pack.trace.len() == nums.len() {
        return rec_pack;
    }

    for (i, &num) in nums.iter().enumerate() {
        if rec_pack.used[i] {
            continue;
        }

        rec_pack.trace.push(num);
        rec_pack.used[i] = true;
        if rec_pack.trace.len() == nums.len() {
            rec_pack.ans.push(rec_pack.trace.clone());
        }
        rec_pack = backtrace_rec(nums, rec_pack);
        rec_pack.trace.pop();
        rec_pack.used[i] = false;
    }

    return rec_pack;
}
