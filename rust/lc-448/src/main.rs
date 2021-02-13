mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![4, 3, 2, 7, 8, 2, 3, 1];

    // Expecting [5, 6]
    println!("{:?}", Solution::find_disappeared_numbers(arr1));
}
