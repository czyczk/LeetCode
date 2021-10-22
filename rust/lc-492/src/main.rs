mod solution;

use solution::Solution;

fn main() {
    assert_eq!(vec![2, 2], Solution::construct_rectangle(4));
    assert_eq!(vec![37, 1], Solution::construct_rectangle(37));
    assert_eq!(vec![427, 286], Solution::construct_rectangle(122122));
    assert_eq!(vec![1, 1], Solution::construct_rectangle(1));
}
