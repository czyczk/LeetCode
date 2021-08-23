pub struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as usize, dst as usize, k as usize);

        let mut dp = vec![vec![std::i32::MAX; n]; k + 2];
        for t in 0..=k + 1 {
            dp[t][src] = 0;
        }

        for t in 1..=k + 1 {
            for flight in flights.iter() {
                let f_src = flight[0] as usize;
                let f_dest = flight[1] as usize;
                dp[t][f_dest] = dp[t][f_dest].min(dp[t - 1][f_src].saturating_add(flight[2]));
            }
        }

        let mut ret = std::i32::MAX;
        for i in 0..=k + 1 {
            ret = ret.min(dp[i][dst]);
        }

        return match ret {
            std::i32::MAX => -1,
            _ => ret,
        };
    }
}
