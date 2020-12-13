mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![1, 2, 3, 1];
    let arr2 = vec![1, 2, 3, 4];

    // Expecting true
    println!("{}", Solution::contains_duplicate(arr1));
    // Expecting false
    println!("{}", Solution::contains_duplicate(arr2));
}
