mod solution;

use solution::Solution;

fn main() {
    let num1 = 143;
    let num2 = 432;
    let num3 = 10;
    let num4 = 1234;
    let num5 = 332;

    // Expecting 139
    println!("{}", Solution::monotone_increasing_digits(num1));
    // Expecting 399
    println!("{}", Solution::monotone_increasing_digits(num2));
    // Expecting 9
    println!("{}", Solution::monotone_increasing_digits(num3));
    // Expecting 1234
    println!("{}", Solution::monotone_increasing_digits(num4));
    // Expecting 299
    println!("{}", Solution::monotone_increasing_digits(num5));
}
