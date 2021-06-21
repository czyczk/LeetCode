mod solution;

use solution::Solution;

fn main() {
    let (s11, s12) = ("sea".to_owned(), "eat".to_owned());
    let (s21, s22) = ("leetcode".to_owned(), "etco".to_owned());

    assert_eq!(2, Solution::min_distance(s11, s12));
    assert_eq!(4, Solution::min_distance(s21, s22));
}
