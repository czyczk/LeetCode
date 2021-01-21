mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![0, 1, 1];
    let a2 = vec![1, 1, 1];
    let a3 = vec![0, 1, 1, 1, 1, 1];
    let a4 = vec![1, 1, 1, 0, 1];

    // Expecting [true, false, false]
    println!("{:?}", Solution::prefixes_div_by5(a1));
    // Expecting [false, false, false]
    println!("{:?}", Solution::prefixes_div_by5(a2));
    // Expecting [true, false, false, false, true, false]
    println!("{:?}", Solution::prefixes_div_by5(a3));
    // Expecting [false, false, false, false, false]
    println!("{:?}", Solution::prefixes_div_by5(a4));
}
