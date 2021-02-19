mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
    let k1 = 2;

    let arr2 = vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1];
    let k2 = 3;

    // Expecting 6
    println!("{}", Solution::longest_ones(arr1, k1));
    // Expecting 10
    println!("{}", Solution::longest_ones(arr2, k2));
}
