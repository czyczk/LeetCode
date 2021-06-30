mod solution;

use solution::Solution;

fn main() {
    let (t1, n1) = (7, vec![2, 3, 1, 2, 4, 3]);
    let (t2, n2) = (4, vec![1, 4, 4]);
    let (t3, n3) = (11, vec![1, 1, 1, 1, 1, 1, 1, 1]);
    let (t4, n4) = (6, vec![10, 2, 3]);

    assert_eq!(2, Solution::min_sub_array_len(t1, n1));
    assert_eq!(1, Solution::min_sub_array_len(t2, n2));
    assert_eq!(0, Solution::min_sub_array_len(t3, n3));
    assert_eq!(1, Solution::min_sub_array_len(t4, n4));
}
