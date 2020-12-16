mod solution;

use solution::Solution;

fn main() {
    let pattern1 = "abba".to_owned();
    let s1 = "dog cat cat dog".to_owned();

    let pattern2 = "abba".to_owned();
    let s2 = "dog cat cat fish".to_owned();

    let pattern3 = "aaaa".to_owned();
    let s3 = "dog cat cat dog".to_owned();

    let pattern4 = "abba".to_owned();
    let s4 = "dog dog dog dog".to_owned();

    // Expecting true
    println!("{}", Solution::word_pattern(pattern1, s1));
    // Expecting false
    println!("{}", Solution::word_pattern(pattern2, s2));
    // Expecting false
    println!("{}", Solution::word_pattern(pattern3, s3));
    // Expecting false
    println!("{}", Solution::word_pattern(pattern4, s4));
}
