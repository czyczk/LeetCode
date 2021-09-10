pub struct Solution {}

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let consumption_turn = chalk.iter().map(|&num| num as i64).sum::<i64>();
        let mut k = k as i64 % consumption_turn;

        for (i, &c) in chalk.iter().enumerate() {
            if k >= c as i64 {
                k -= c as i64
            } else {
                return i as i32;
            }
        }

        0
    }
}
