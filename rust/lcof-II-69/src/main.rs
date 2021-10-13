mod solution;

use solution::Solution;

fn main() {
    let a1 = vec![0, 1, 0];
    let a2 = vec![1, 3, 5, 4, 2];
    let a3 = vec![0, 10, 5, 2];
    let a4 = vec![3, 4, 5, 1];
    let a5 = vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19];
    let a6 = vec![10, 0];
    let a7 = vec![0, 10];
    let a8 = vec![0];

    assert_eq!(1, Solution::peak_index_in_mountain_array(a1));
    assert_eq!(2, Solution::peak_index_in_mountain_array(a2));
    assert_eq!(1, Solution::peak_index_in_mountain_array(a3));
    assert_eq!(2, Solution::peak_index_in_mountain_array(a4));
    assert_eq!(2, Solution::peak_index_in_mountain_array(a5));
    assert_eq!(0, Solution::peak_index_in_mountain_array(a6));
    assert_eq!(1, Solution::peak_index_in_mountain_array(a7));
    assert_eq!(0, Solution::peak_index_in_mountain_array(a8));
}
