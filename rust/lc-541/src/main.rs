mod solution;

use solution::Solution;

fn main() {
    let (s1, k1) = ("abcdefg".to_owned(), 2);
    let (s2, k2) = ("abcd".to_owned(), 2);
    let (s3, k3) = ("abababab".to_owned(), 1);

    assert_eq!("bacdfeg", Solution::reverse_str(s1, k1));
    assert_eq!("bacd", Solution::reverse_str(s2, k2));
    assert_eq!("abababab", Solution::reverse_str(s3, k3));
}
