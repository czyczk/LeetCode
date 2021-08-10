mod solution;

use solution::Solution;

fn main() {
    let (n1, t1) = (vec![1, 0, -1, 0, -2, 2], 0);
    let (n2, t2) = (vec![2, 2, 2, 2, 2], 8);
    let (n3, t3) = (vec![0], 0);
    let (n4, t4) = (vec![1, -2, -5, -4, -3, 3, 3, 5], -11);

    check_vec_eq(
        vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]],
        Solution::four_sum(n1, t1),
    );

    check_vec_eq(vec![vec![2, 2, 2, 2]], Solution::four_sum(n2, t2));

    check_vec_eq(vec![], Solution::four_sum(n3, t3));

    check_vec_eq(vec![vec![-5, -4, -3, 1]], Solution::four_sum(n4, t4));
}

fn check_vec_eq(vec1: Vec<Vec<i32>>, vec2: Vec<Vec<i32>>) {
    assert_eq!(format!("{:?}", vec1), format!("{:?}", vec2));
}
