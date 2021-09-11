pub struct Solution {}

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        if n <= 2 {
            return n + 1;
        }

        let mut dp = vec![false; n as usize + 1];
        for i in 0..=2 {
            dp[i] = true;
        }

        let mut count = 3;
        let mut milestone = 2;

        let mut i = 3;
        while i <= n {
            let rem = i % milestone;
            if rem >= milestone / 2 {
                i += milestone / 2;
                milestone *= 2;
                continue;
            }

            let should_count = dp[rem as usize];
            if should_count {
                count += 1;
                dp[i as usize] = true;
            }

            i += 1;
        }

        return count;
    }
}
