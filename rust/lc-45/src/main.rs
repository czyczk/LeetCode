mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![2, 3, 1, 1, 4];
    let n2 = vec![2, 3, 0, 1, 4];
    let n3 = vec![2, 1];
    let n4 = vec![3, 2, 1];
    let n5 = vec![0];

    assert_eq!(2, Solution::jump(n1));
    assert_eq!(2, Solution::jump(n2));
    assert_eq!(1, Solution::jump(n3));
    assert_eq!(1, Solution::jump(n4));
    assert_eq!(0, Solution::jump(n5));
}
