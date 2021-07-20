mod solution;

use solution::fast;
use solution::window_n2;

fn main() {
    let (n1, f1, s1) = (vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2);
    let (n2, f2, s2) = (vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2);
    let (n3, f3, s3) = (vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3);

    assert_eq!(
        20,
        window_n2::Solution::max_sum_two_no_overlap(n1.clone(), f1, s1)
    );
    assert_eq!(20, fast::Solution::max_sum_two_no_overlap(n1, f1, s1));

    assert_eq!(
        29,
        window_n2::Solution::max_sum_two_no_overlap(n2.clone(), f2, s2)
    );
    assert_eq!(29, fast::Solution::max_sum_two_no_overlap(n2, f2, s2));

    assert_eq!(
        31,
        window_n2::Solution::max_sum_two_no_overlap(n3.clone(), f3, s3)
    );
    assert_eq!(31, fast::Solution::max_sum_two_no_overlap(n3, f3, s3));
}
