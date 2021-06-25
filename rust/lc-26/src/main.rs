mod solution;

use solution::Solution;

fn main() {
    let mut n1 = vec![1, 1, 2];
    let mut n2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

    assert_eq!(2, Solution::remove_duplicates(&mut n1));
    assert_eq!(5, Solution::remove_duplicates(&mut n2));
}
