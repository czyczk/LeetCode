mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![1, 1, 0, 1, 1, 1];

    // Expecting 3
    println!("{}", Solution::find_max_consecutive_ones(arr1));
}

