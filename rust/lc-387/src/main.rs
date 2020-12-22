mod solution;

use solution::Solution;

fn main() {
    let s1 = "leetcode".to_owned();
    let s2 = "loveleetcode".to_owned();
    let s3 = "".to_owned();

    // Expecting 0
    println!("{}", Solution::first_uniq_char(s1));
    // Expecting 2
    println!("{}", Solution::first_uniq_char(s2));
    // Expecting -1
    println!("{}", Solution::first_uniq_char(s3));
}
