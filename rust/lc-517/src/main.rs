mod solution;

use solution::Solution;

fn main() {
    let m1 = vec![1, 0, 5];
    let m2 = vec![0, 3, 0];
    let m3 = vec![0, 2, 0];
    let m4 = vec![4, 0, 0, 4];
    let m5 = vec![4, 0, 7, 4, 0];
    let m6 = vec![1];

    assert_eq!(3, Solution::find_min_moves(m1));
    assert_eq!(2, Solution::find_min_moves(m2));
    assert_eq!(-1, Solution::find_min_moves(m3));
    assert_eq!(2, Solution::find_min_moves(m4));
    assert_eq!(4, Solution::find_min_moves(m5));
    assert_eq!(0, Solution::find_min_moves(m6));
}
