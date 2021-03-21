mod solution;

use solution::Solution;

fn main() {
    let mut m1 = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    let mut m2 = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];

    Solution::set_zeroes(&mut m1);
    assert_eq!(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1],], m1);

    Solution::set_zeroes(&mut m2);
    assert_eq!(
        vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
        m2
    );
}
