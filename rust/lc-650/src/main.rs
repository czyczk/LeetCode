mod solution;

use solution::Solution;

fn main() {
    assert_eq!(0, Solution::min_steps(1));
    assert_eq!(2, Solution::min_steps(2));
    assert_eq!(3, Solution::min_steps(3));
    assert_eq!(4, Solution::min_steps(4));
    assert_eq!(6, Solution::min_steps(9));
    assert_eq!(10, Solution::min_steps(21));
    assert_eq!(9, Solution::min_steps(27));
    assert_eq!(16, Solution::min_steps(189));
}
