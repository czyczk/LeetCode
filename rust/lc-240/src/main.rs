mod solution;

use solution::Solution;

fn main() {
    let m1 = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![18, 21, 23, 26, 30],
    ];

    let m2 = m1.clone();

    assert_eq!(true, Solution::search_matrix(m1, 5));
    assert_eq!(false, Solution::search_matrix(m2, 20));
}
