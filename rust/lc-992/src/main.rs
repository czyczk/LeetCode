mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![1, 2, 1, 2, 3];
    let k1 = 2;

    let arr2 = vec![1, 2, 1, 3, 4];
    let k2 = 3;

    // Expecting 7
    println!("{}", Solution::subarrays_with_k_distinct(arr1, k1));
    // Expecting 3
    println!("{}", Solution::subarrays_with_k_distinct(arr2, k2));
}
