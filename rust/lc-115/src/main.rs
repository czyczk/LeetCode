mod solution;

use solution::Solution;

fn main() {
    let (s1, t1) = ("rabbbit".to_owned(), "rabbit".to_owned());
    let (s2, t2) = ("babgbag".to_owned(), "bag".to_owned());

    assert_eq!(3, Solution::num_distinct(s1, t1));
    assert_eq!(5, Solution::num_distinct(s2, t2));
}
