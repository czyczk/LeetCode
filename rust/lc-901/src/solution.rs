pub struct StockSpanner {
    prices: Vec<i32>,
    weights: Vec<i32>,
}

impl StockSpanner {
    pub fn new() -> Self {
        Self {
            prices: vec![],
            weights: vec![],
        }
    }

    pub fn next(&mut self, price: i32) -> i32 {
        let mut weight = 1;

        while !self.prices.is_empty() && self.prices.last().unwrap() <= &price {
            self.prices.pop().unwrap();
            weight += self.weights.pop().unwrap();
        }

        self.prices.push(price);
        self.weights.push(weight);

        weight
    }
}
