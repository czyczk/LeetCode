mod solution;

use solution::lp;
use solution::rec;

fn main() {
    let (c1, t1) = (vec![10, 1, 2, 7, 6, 1, 5], 8);
    let (c2, t2) = (vec![2, 5, 2, 1, 2], 5);

    check_vec_eq(
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        lp::Solution::combination_sum2(c1.clone(), t1),
    );
    check_vec_eq(
        vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
        rec::Solution::combination_sum2(c1, t1),
    );

    check_vec_eq(
        vec![vec![1, 2, 2], vec![5]],
        lp::Solution::combination_sum2(c2.clone(), t2),
    );
    check_vec_eq(
        vec![vec![1, 2, 2], vec![5]],
        rec::Solution::combination_sum2(c2, t2),
    );
}

fn check_vec_eq(expected: Vec<Vec<i32>>, actual: Vec<Vec<i32>>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
