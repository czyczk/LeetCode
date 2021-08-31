mod solution;

use solution::Solution;

fn main() {
    let s1 = "hello".to_owned();
    let s2 = "leetcode".to_owned();
    let s3 = "a.".to_owned();
    let s4 = "aA".to_owned();

    assert_eq!("holle", Solution::reverse_vowels(s1));
    assert_eq!("leotcede", Solution::reverse_vowels(s2));
    assert_eq!("a.", Solution::reverse_vowels(s3));
    assert_eq!("Aa", Solution::reverse_vowels(s4));
}
