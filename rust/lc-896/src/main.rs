mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![1, 2, 2, 3];
    let a2 = vec![6, 5, 4, 4];
    let a3 = vec![1, 3, 2];
    let a4 = vec![1, 2, 4, 5];
    let a5 = vec![1, 1, 1];
    let a6 = vec![9];

    // Expecting true
    println!("{}", Solution::is_monotonic(a1));
    // Expecting true
    println!("{}", Solution::is_monotonic(a2));
    // Expecting false
    println!("{}", Solution::is_monotonic(a3));
    // Expecting true
    println!("{}", Solution::is_monotonic(a4));
    // Expecting true
    println!("{}", Solution::is_monotonic(a5));
    // Expecting true
    println!("{}", Solution::is_monotonic(a6));
}
