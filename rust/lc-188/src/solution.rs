pub struct SolutionOriginal {}

impl SolutionOriginal {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0;
        }

        let k = (k as usize).min(n / 2);

        let mut buy = vec![vec![0; k + 1]; n];
        let mut sell = vec![vec![0; k + 1]; n];

        // Initial conditions
        buy[0][0] = -prices[0];
        sell[0][0] = 0;

        for j in 1..k + 1 {
            buy[0][j] = std::i32::MIN / 2;
            sell[0][j] = std::i32::MIN / 2;
        }

        for i in 1..n {
            buy[i][0] = buy[i - 1][0].max(sell[i - 1][0] - prices[i]);

            for j in 1..k + 1 {
                buy[i][j] = buy[i - 1][j].max(sell[i - 1][j] - prices[i]);
                sell[i][j] = (buy[i - 1][j - 1] + prices[i]).max(sell[i - 1][j])
            }
        }

        return sell[n - 1].iter().fold(0, |max, val| max.max(*val));
    }
}
