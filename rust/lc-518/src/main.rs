mod solution;

use solution::Solution;

fn main() {
    let (a1, c1) = (5, vec![1, 2, 5]);
    let (a2, c2) = (3, vec![2]);
    let (a3, c3) = (10, vec![10]);

    assert_eq!(4, Solution::change(a1, c1));
    assert_eq!(0, Solution::change(a2, c2));
    assert_eq!(1, Solution::change(a3, c3));
}
