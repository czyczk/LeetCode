mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![1, 1];
    let b1 = vec![2, 2];
    let a2 = vec![1, 2];
    let b2 = vec![2, 3];
    let a3 = vec![2];
    let b3 = vec![1, 3];
    let a4 = vec![1, 2, 5];
    let b4 = vec![2, 4];

    // Expecting [1, 2]
    println!("{:?}", Solution::fair_candy_swap(a1, b1));
    // Expecting [1, 2]
    println!("{:?}", Solution::fair_candy_swap(a2, b2));
    // Expecting [2, 3]
    println!("{:?}", Solution::fair_candy_swap(a3, b3));
    // Expecting [5, 4]
    println!("{:?}", Solution::fair_candy_swap(a4, b4));
}
