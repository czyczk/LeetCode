mod solution;

use solution::Solution;

fn main() {
    let mut n1 = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut n1);

    let mut n2 = vec![0];
    Solution::move_zeroes(&mut n2);

    assert_eq!("[1, 3, 12, 0, 0]", format!("{:?}", n1));
    assert_eq!("[0]", format!("{:?}", n2));
}
