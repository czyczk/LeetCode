mod solution;

use solution::Solution;

fn main() {
    let s1 = "bcabc".to_owned();
    let s2 = "cbacdcbc".to_owned();
    let s3 = "abacb".to_owned();

    // Expecting "abc"
    println!("{}", Solution::remove_duplicate_letters(s1));
    // Expecting "cbacdcbc"
    println!("{}", Solution::remove_duplicate_letters(s2));
    // Expecting "abc"
    println!("{}", Solution::remove_duplicate_letters(s3));
}
