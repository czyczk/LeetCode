pub struct Solution {}

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut max_result: i32 = card_points[0..k].iter().sum();
        let mut sum = max_result;

        let n = card_points.len();
        for i in (0..k).rev() {
            sum = sum - card_points[i] + card_points[n + i - k];
            max_result = max_result.max(sum);
        }

        max_result
    }
}
