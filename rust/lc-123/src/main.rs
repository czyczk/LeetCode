mod solution;

use solution::Solution;

fn main() {
    let prices1 = vec![3, 3, 5, 0, 0, 3, 1, 4];
    let prices2 = vec![1, 2, 3, 4, 5];
    let prices3 = vec![7, 6, 4, 3, 1];
    let prices4 = vec![1];

    // Expecting 6
    println!("{}", Solution::max_profit(prices1));
    // Expecting 4
    println!("{}", Solution::max_profit(prices2));
    // Expecting 0
    println!("{}", Solution::max_profit(prices3));
    // Expecting 0
    println!("{}", Solution::max_profit(prices4));
}
