mod solution;

use solution::Solution;

fn main() {
    let cost1 = vec![10, 15, 20];
    let cost2 = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
    let cost3 = vec![0, 0, 0, 0];

    // Expecting 15
    println!("{}", Solution::min_cost_climbing_stairs(cost1));
    // Expecting 6
    println!("{}", Solution::min_cost_climbing_stairs(cost2));
    // Expecting 0
    println!("{}", Solution::min_cost_climbing_stairs(cost3));
}
