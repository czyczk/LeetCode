pub struct Solution {}

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![-1; amount + 1];
        dp[0] = 0;

        for coin in coins.iter() {
            let coin = *coin as usize;
            for j in coin..=amount {
                if dp[j - coin] == -1 {
                    continue;
                }

                if dp[j] == -1 {
                    dp[j] = dp[j - coin] + 1;
                } else {
                    dp[j] = dp[j].min(dp[j - coin] + 1);
                }
            }
        }

        return dp[amount];
    }
}
