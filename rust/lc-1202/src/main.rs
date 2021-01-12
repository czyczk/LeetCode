mod solution;

use solution::Solution;

fn main() {
    let s1 = "dcab".to_owned();
    let pairs1 = vec![vec![0, 3], vec![1, 2]];

    let s2 = "dcab".to_owned();
    let pairs2 = vec![vec![0, 3], vec![1, 2], vec![0, 2]];

    let s3 = "cba".to_owned();
    let pairs3 = vec![vec![0, 1], vec![1, 2]];

    // Expecting "bacd"
    println!("{}", Solution::smallest_string_with_swaps(s1, pairs1));
    // Expecting "abcd"
    println!("{}", Solution::smallest_string_with_swaps(s2, pairs2));
    // Expecting "abc"
    println!("{}", Solution::smallest_string_with_swaps(s3, pairs3));
}
