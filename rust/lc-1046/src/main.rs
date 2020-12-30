mod solution;

use solution::Solution;

fn main() {
    let stones1 = vec![2, 7, 4, 1, 8, 1];

    // Expecting 1
    println!("{}", Solution::last_stone_weight(stones1));
}
