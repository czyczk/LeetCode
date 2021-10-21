mod solution;

use solution::Solution;

fn main() {
    let d1 = vec![1, 2, 3];
    let d2 = vec![4, 3, 2, 1];
    let d3 = vec![0];
    let d4 = vec![9];

    check_ans(vec![1, 2, 4], Solution::plus_one(d1));
    check_ans(vec![4, 3, 2, 2], Solution::plus_one(d2));
    check_ans(vec![1], Solution::plus_one(d3));
    check_ans(vec![1, 0], Solution::plus_one(d4));
}

fn check_ans(expected: Vec<i32>, actual: Vec<i32>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
