mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![1, 4, 2, 5, 3];
    let a2 = vec![1, 2];
    let a3 = vec![10, 11, 12];

    assert_eq!(58, Solution::sum_odd_length_subarrays(a1));
    assert_eq!(3, Solution::sum_odd_length_subarrays(a2));
    assert_eq!(66, Solution::sum_odd_length_subarrays(a3));
}
