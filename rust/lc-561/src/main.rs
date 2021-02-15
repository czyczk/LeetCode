mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![1, 5, 10, 2, 15, 12];
    let arr2 = vec![1, 4, 3, 2];
    let arr3 = vec![6, 2, 6, 5, 1, 2];

    // Expecting 18
    println!("{}", Solution::array_pair_sum(arr1));
    // Expecting 4
    println!("{}", Solution::array_pair_sum(arr2));
    // Expecting 9
    println!("{}", Solution::array_pair_sum(arr3));
}
