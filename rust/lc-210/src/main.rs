mod solution;

use solution::Solution;

fn main() {
    let (n1, pre1) = (2, vec![vec![1, 0]]);
    let (n2, pre2) = (4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);

    // Expecting [0, 1]
    println!("{:?}", Solution::find_order(n1, pre1));
    // Expecting [0, 1, 2, 3] or [0, 2, 1, 3]
    println!("{:?}", Solution::find_order(n2, pre2));
}
