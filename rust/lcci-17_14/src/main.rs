mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![1, 3, 5, 7, 2, 4, 6, 8];
    let k1 = 4;
    let a2 = vec![];
    let k2 = 0;

    check_eq(vec![1, 2, 3, 4], Solution::smallest_k(a1, k1));
    check_eq(vec![], Solution::smallest_k(a2, k2));
}

fn check_eq(expected: Vec<i32>, actual: Vec<i32>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
