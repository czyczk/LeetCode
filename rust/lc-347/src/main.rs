mod solution;

use solution::Solution;

fn main() {
    let (n1, k1) = (vec![1, 1, 1, 2, 2, 3], 2);
    let (n2, k2) = (vec![1], 1);

    check_vec_elements_equal(vec![1, 2], Solution::top_k_frequent(n1, k1));
    check_vec_elements_equal(vec![1], Solution::top_k_frequent(n2, k2));
}

fn check_vec_elements_equal(mut expected: Vec<i32>, mut actual: Vec<i32>) {
    expected.sort_unstable();
    actual.sort_unstable();
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
