mod solution;

use solution::Solution;

fn main() {
    let (n1, t1) = (vec![-1, 0, 3, 5, 9, 12], 9);
    let (n2, t2) = (vec![-1, 0, 3, 5, 9, 12], 2);

    assert_eq!(4, Solution::search(n1, t1));
    assert_eq!(-1, Solution::search(n2, t2));
}
