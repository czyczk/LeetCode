mod solution;

use solution::Solution;

fn main() {
    let d1 = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];

    // Expecting 1
    println!("{}", Solution::num_equiv_domino_pairs(d1));
}
