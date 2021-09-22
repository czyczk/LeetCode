mod solution;

use solution::hash_set;
use solution::union_find;

fn main() {
    let n1 = vec![100, 4, 200, 1, 3, 2];
    let n2 = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    let n3 = vec![];
    let n4 = vec![1, 1, 1];

    assert_eq!(4, union_find::Solution::longest_consecutive(n1.clone()));
    assert_eq!(9, union_find::Solution::longest_consecutive(n2.clone()));
    assert_eq!(0, union_find::Solution::longest_consecutive(n3.clone()));
    assert_eq!(1, union_find::Solution::longest_consecutive(n4.clone()));

    assert_eq!(4, hash_set::Solution::longest_consecutive(n1.clone()));
    assert_eq!(9, hash_set::Solution::longest_consecutive(n2.clone()));
    assert_eq!(0, hash_set::Solution::longest_consecutive(n3.clone()));
    assert_eq!(1, hash_set::Solution::longest_consecutive(n4.clone()));
}
