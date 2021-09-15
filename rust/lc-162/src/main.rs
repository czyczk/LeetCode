mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![1, 2, 3, 1];
    let n2 = vec![1, 2, 1, 3, 5, 6, 4];
    let n3 = vec![1];
    let n4 = vec![-2147483648];

    check_answer(vec![2], Solution::find_peak_element(n1));
    check_answer(vec![5, 1], Solution::find_peak_element(n2));
    check_answer(vec![0], Solution::find_peak_element(n3));
    check_answer(vec![0], Solution::find_peak_element(n4));
}

fn check_answer(answers: Vec<i32>, actual: i32) {
    assert_eq!(true, answers.contains(&actual));
}
