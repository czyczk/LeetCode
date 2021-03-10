mod solution;

use solution::Solution;

fn main() {
    let s1 = "1 + 1".to_owned();
    let s2 = "2-1 + 2".to_owned();
    let s3 = "(1+(4+5+2)-3)+(6+8)".to_owned();

    // Expecting 2
    assert_eq!(2, Solution::calculate(s1));
    // Expecting 3
    assert_eq!(3, Solution::calculate(s2));
    // Expecting 23
    assert_eq!(23, Solution::calculate(s3));
}
