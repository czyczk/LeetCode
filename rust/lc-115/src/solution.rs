pub struct Solution {}

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let m = s.len();

        let t = t.as_bytes();
        let n = t.len();

        if m < n {
            return 0;
        }

        let mut dp = vec![vec![0; m + 1]; n + 1];
        for j in 0..=m {
            dp[0][j] = 1;
        }

        for i in 1..=n {
            for j in 1..=m {
                if t[i - 1] == s[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + dp[i][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }

        return dp[n][m];
    }
}
