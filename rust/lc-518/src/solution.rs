pub struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let mut dp = vec![0; amount as usize + 1];
        dp[0] = 1;

        for &coin in coins.iter() {
            for j in coin as usize..=amount as usize {
                dp[j] += dp[j - coin as usize];
            }
        }

        return dp[amount as usize];
    }
}
