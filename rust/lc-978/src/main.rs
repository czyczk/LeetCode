mod solution;

use solution::Solution;

fn main() {
    let arr1 = vec![9, 4, 2, 10, 7, 8, 8, 1, 9];
    let arr2 = vec![4, 8, 12, 16];
    let arr3 = vec![100];
    let arr4 = vec![9, 4, 2, 10, 7, 8, 9, 1, 9, 2, 3, 1, 2, 1, 2];
    let arr5 = vec![9, 9];

    // Expecting 5
    println!("{}", Solution::max_turbulence_size(arr1));
    // Expecting 2
    println!("{}", Solution::max_turbulence_size(arr2));
    // Expecting 1
    println!("{}", Solution::max_turbulence_size(arr3));
    // Expecting 10
    println!("{}", Solution::max_turbulence_size(arr4));
    // Expecting 1
    println!("{}", Solution::max_turbulence_size(arr5));
}
