mod solution;

use solution::Solution;

fn main() {
    let is_connected1 = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
    let is_connected2 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];

    // Expecting 2
    println!("{}", Solution::find_circle_num(is_connected1));
    // Expecting 3
    println!("{}", Solution::find_circle_num(is_connected2));
}
