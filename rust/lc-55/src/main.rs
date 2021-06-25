mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![2, 3, 1, 1, 4];
    let n2 = vec![3, 2, 1, 0, 4];

    assert_eq!(true, Solution::can_jump(n1));
    assert_eq!(false, Solution::can_jump(n2));
}
