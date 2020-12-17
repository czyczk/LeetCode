mod solution;

use solution::Solution;

fn main() {
    let prices1 = vec![1, 3, 2, 8, 4, 9];
    let prices2 = vec![1, 3, 7, 5, 10, 3];

    // Expecting 8
    println!("{}", Solution::max_profit(prices1, 2));
    // Expecting 6
    println!("{}", Solution::max_profit(prices2, 3));
}
