pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        match n {
            0 | 1 => return 0,
            _ => {}
        }

        let max_tx = 2.min(n / 2);

        let mut holding = vec![vec![0; max_tx + 1]; n];
        let mut sold = vec![vec![0; max_tx + 1]; n];

        holding[0][0] = -prices[0];
        for j in 1..max_tx + 1 {
            holding[0][j] = std::i32::MIN / 2;
            sold[0][j] = std::i32::MIN / 2;
        }

        for i in 1..n {
            holding[i][0] = holding[i - 1][0].max(sold[i - 1][0] - prices[i]);

            for j in 1..max_tx + 1 {
                holding[i][j] = holding[i - 1][j].max(sold[i - 1][j] - prices[i]);
                sold[i][j] = (holding[i - 1][j - 1] + prices[i]).max(sold[i - 1][j]);
            }
        }

        sold[n - 1].iter().fold(0, |max, val| max.max(*val))
    }
}
