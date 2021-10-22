mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![3, 2, 3];
    let n2 = vec![1];
    let n3 = vec![1, 2];

    check_ans(vec![3], Solution::majority_element(n1));
    check_ans(vec![1], Solution::majority_element(n2));
    check_ans(vec![1, 2], Solution::majority_element(n3));
}

fn check_ans(expected: Vec<i32>, actual: Vec<i32>) {
    assert_eq!(expected.len(), actual.len());
    let mut expected_set = std::collections::HashSet::with_capacity(expected.len());
    for &num in expected.iter() {
        expected_set.insert(num);
    }
    let mut actual_set = std::collections::HashSet::with_capacity(actual.len());
    for &num in actual.iter() {
        actual_set.insert(num);
    }
    assert_eq!(expected_set, actual_set);
}
