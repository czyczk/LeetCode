mod solution;

use solution::Solution;

fn main() {
    let s1 = "aab".to_owned();
    let s2 = "a".to_owned();
    let s3 = "aaba".to_owned();

    // Expecting [["a", "a", "b"], ["aa", "b"]]
    println!("{:?}", Solution::partition(s1));
    // Expecting [["a"]]
    println!("{:?}", Solution::partition(s2));
    // Expecting [["a", "a", "b", "a"], ["a", "aba"], ["aa", "b", "a"]]
    println!("{:?}", Solution::partition(s3));
}
