pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }

        let (mut sold, mut holding) = (0, -prices[0]);
        for i in 1..n {
            sold = sold.max(holding + prices[i] - fee);
            holding = holding.max(sold - prices[i]);
        }

        sold
    }
}
