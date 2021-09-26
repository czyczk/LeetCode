mod solution;

use solution::Solution;

fn main() {
    assert_eq!(3, Solution::get_sum(1, 2));
    assert_eq!(5, Solution::get_sum(2, 3));
}
