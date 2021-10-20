mod solution;

use solution::Solution;

fn main() {
    assert_eq!(3, Solution::min_moves(vec![1, 2, 3]));
    assert_eq!(0, Solution::min_moves(vec![1, 1, 1]));
    assert_eq!(0, Solution::min_moves(vec![1]));
}
