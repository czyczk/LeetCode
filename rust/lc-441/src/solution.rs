pub struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((-1.0 + (8.0 * n as f64 + 1.0).sqrt()) / 2.0).floor() as i32
    }
}
