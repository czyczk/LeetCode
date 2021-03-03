mod solution;

use solution::Solution;
use solution::SolutionHacker;

fn main() {
    // Expecting [0, 1, 1]
    println!("{:?}", Solution::count_bits(2));

    // Expecting [0, 1, 1, 2, 1, 2]
    println!("{:?}", SolutionHacker::count_bits(5));
}
