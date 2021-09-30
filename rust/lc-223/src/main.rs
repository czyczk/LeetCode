mod solution;

use solution::Solution;

fn main() {
    assert_eq!(45, Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
    assert_eq!(16, Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2));
    assert_eq!(4, Solution::compute_area(0, 0, 0, 0, -1, -1, 1, 1));
}
