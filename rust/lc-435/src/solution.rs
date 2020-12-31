pub struct Solution {}

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let n = intervals.len();
        if n == 0 {
            return 0;
        }

        intervals.sort_unstable_by(cmp_vec_i32);

        let mut end = intervals[0][1];
        let mut removal_cnt = 0;

        for i in 1..n {
            match intervals[i][0].cmp(&end) {
                std::cmp::Ordering::Less => {
                    removal_cnt += 1;
                    end = end.min(intervals[i][1]);
                }
                _ => end = intervals[i][1],
            }
        }

        removal_cnt
    }
}

fn cmp_vec_i32(a: &Vec<i32>, b: &Vec<i32>) -> std::cmp::Ordering {
    return a[0].cmp(&b[0]);
}
