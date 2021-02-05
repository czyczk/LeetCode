mod solution;

use solution::Solution;

fn main() {
    let s1 = "abcd".to_owned();
    let t1 = "bcdf".to_owned();
    let max_cost1 = 3;

    let s2 = "abcd".to_owned();
    let t2 = "cdef".to_owned();
    let max_cost2 = 3;

    let s3 = "abcd".to_owned();
    let t3 = "acde".to_owned();
    let max_cost3 = 0;

    // Expecting 3
    println!("{}", Solution::equal_substring(s1, t1, max_cost1));
    // Expecting 1
    println!("{}", Solution::equal_substring(s2, t2, max_cost2));
    // Expecting 1
    println!("{}", Solution::equal_substring(s3, t3, max_cost3));
}
