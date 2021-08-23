mod solution;

use solution::Solution;

fn main() {
    assert_eq!(0, Solution::get_maximum_generated(0));
    assert_eq!(1, Solution::get_maximum_generated(1));
    assert_eq!(1, Solution::get_maximum_generated(2));
    assert_eq!(2, Solution::get_maximum_generated(3));
    assert_eq!(2, Solution::get_maximum_generated(4));
    assert_eq!(3, Solution::get_maximum_generated(7));
    assert_eq!(4, Solution::get_maximum_generated(9));
    assert_eq!(5, Solution::get_maximum_generated(15));
}
