mod solution;

use solution::lp;
use solution::rec;

fn main() {
    let n1 = vec![1, 2, 3];
    let n2 = vec![0, 1];
    let n3 = vec![1];

    let exp1 = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    check_vec_equal(exp1.clone(), lp::Solution::permute(n1.clone()));
    check_vec_equal(exp1, rec::Solution::permute(n1));

    let exp2 = vec![vec![0, 1], vec![1, 0]];
    check_vec_equal(exp2.clone(), lp::Solution::permute(n2.clone()));
    check_vec_equal(exp2, rec::Solution::permute(n2));

    let exp3 = vec![vec![1]];
    check_vec_equal(exp3.clone(), lp::Solution::permute(n3.clone()));
    check_vec_equal(exp3, rec::Solution::permute(n3));
}

fn check_vec_equal(expected: Vec<Vec<i32>>, actual: Vec<Vec<i32>>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
