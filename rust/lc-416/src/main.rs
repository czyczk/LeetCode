mod solution;

use solution::Solution;
use solution::Solution2DArray;

fn main() {
    let n1 = vec![1, 5, 11, 5];
    let n2 = vec![1, 2, 3, 5];
    let n3 = vec![1, 5, 10, 6];
    let n4 = vec![2, 13, 1];
    let n5 = vec![14, 9, 8, 4, 3, 2];

    assert_eq!(true, Solution2DArray::can_partition(n1.clone()));
    assert_eq!(false, Solution2DArray::can_partition(n2.clone()));
    assert_eq!(true, Solution2DArray::can_partition(n3.clone()));
    assert_eq!(false, Solution2DArray::can_partition(n4.clone()));
    assert_eq!(true, Solution2DArray::can_partition(n5.clone()));

    assert_eq!(true, Solution::can_partition(n1));
    assert_eq!(false, Solution::can_partition(n2));
    assert_eq!(true, Solution::can_partition(n3));
    assert_eq!(false, Solution::can_partition(n4));
    assert_eq!(true, Solution::can_partition(n5));
}
