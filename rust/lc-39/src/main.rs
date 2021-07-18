mod solution;

use solution::lp;
use solution::rec;

fn main() {
    let (c1, t1) = (vec![2, 3, 6, 7], 7);
    let (c2, t2) = (vec![2, 3, 5], 8);
    let (c3, t3) = (vec![2], 1);
    let (c4, t4) = (vec![1], 1);
    let (c5, t5) = (vec![1], 2);
    let (c6, t6) = (vec![2, 7, 6, 3, 5, 1], 9);

    check_ans_eq(
        vec![vec![2, 2, 3], vec![7]],
        lp::Solution::combination_sum(c1.clone(), t1),
    );
    check_ans_eq(
        vec![vec![2, 2, 3], vec![7]],
        rec::Solution::combination_sum(c1, t1),
    );

    check_ans_eq(
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        lp::Solution::combination_sum(c2.clone(), t2),
    );
    check_ans_eq(
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
        rec::Solution::combination_sum(c2, t2),
    );

    check_ans_eq(vec![], lp::Solution::combination_sum(c3.clone(), t3));
    check_ans_eq(vec![], rec::Solution::combination_sum(c3, t3));

    check_ans_eq(vec![vec![1]], lp::Solution::combination_sum(c4.clone(), t4));
    check_ans_eq(vec![vec![1]], rec::Solution::combination_sum(c4, t4));

    check_ans_eq(
        vec![vec![1, 1]],
        lp::Solution::combination_sum(c5.clone(), t5),
    );
    check_ans_eq(vec![vec![1, 1]], rec::Solution::combination_sum(c5, t5));

    // 21 results in total
    assert_eq!(21, lp::Solution::combination_sum(c6.clone(), t6).len());
    assert_eq!(21, rec::Solution::combination_sum(c6, t6).len());
}

fn check_ans_eq(expected: Vec<Vec<i32>>, actual: Vec<Vec<i32>>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
