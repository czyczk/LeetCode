mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![1, 2, 1, 3, 2, 5];
    let n2 = vec![-1, 0];
    let n3 = vec![0, 1];

    check_ans(vec![3, 5], Solution::single_number(n1));
    check_ans(vec![-1, 0], Solution::single_number(n2));
    check_ans(vec![1, 0], Solution::single_number(n3));
}

fn check_ans(expected: Vec<i32>, actual: Vec<i32>) {
    let mut set_expected = std::collections::HashSet::new();
    let mut set_actual = std::collections::HashSet::new();

    for num in expected.into_iter() {
        set_expected.insert(num);
    }

    for num in actual.into_iter() {
        set_actual.insert(num);
    }

    assert_eq!(set_expected, set_actual);
}
