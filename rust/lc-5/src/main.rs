mod solution;

use solution::Solution;

fn main() {
    let s1 = "babad".to_owned();
    let s2 = "cbbd".to_owned();
    let s3 = "a".to_owned();
    let s4 = "ac".to_owned();

    // Expecting "bab"
    println!("{}", Solution::longest_palindrome(s1));
    // Expecting "bb"
    println!("{}", Solution::longest_palindrome(s2));
    // Expecting "a"
    println!("{}", Solution::longest_palindrome(s3));
    // Expecting "a"
    println!("{}", Solution::longest_palindrome(s4));
}
