mod solution;

use solution::Solution;

fn main() {
    let s1 = "3+2*2".to_owned();
    let s2 = " 3/2 ".to_owned();
    let s3 = " 3+5 / 2 ".to_owned();
    let s4 = "2048".to_owned();

    assert_eq!(7, Solution::calculate(s1));
    assert_eq!(1, Solution::calculate(s2));
    assert_eq!(5, Solution::calculate(s3));
    assert_eq!(2048, Solution::calculate(s4));
}
