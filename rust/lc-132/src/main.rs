mod solution;

use solution::Solution;

fn main() {
    let s1 = "aab".to_owned();
    let s2 = "a".to_owned();
    let s3 = "ab".to_owned();
    let s4 = "cdd".to_owned();

    // Expecting 1
    assert_eq!(1, Solution::min_cut(s1));
    // Expecting 0
    assert_eq!(0, Solution::min_cut(s2));
    // Expecting 1
    assert_eq!(1, Solution::min_cut(s3));
    // Expectin 1
    assert_eq!(1, Solution::min_cut(s4));
}
