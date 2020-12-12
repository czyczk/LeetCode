mod solution;

use solution::Solution;

fn main() {
    let seq1 = vec![1, 7, 4, 9, 2, 5];
    let seq2 = vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8];
    let seq3 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let seq4 = vec![1];

    // Expecting 6
    println!("{}", Solution::wiggle_max_length(seq1));
    // Expecting 7
    println!("{}", Solution::wiggle_max_length(seq2));
    // Expecting 2
    println!("{}", Solution::wiggle_max_length(seq3));
    // Expecting 1
    println!("{}", Solution::wiggle_max_length(seq4));
}
