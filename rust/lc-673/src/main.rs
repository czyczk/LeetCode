mod solution;

use solution::Solution;

fn main() {
    let n1 = vec![1, 3, 5, 4, 7];
    let n2 = vec![2, 2, 2, 2, 2];
    let n3 = vec![1, 2, 4, 3, 5, 4, 7, 2];
    let n4 = vec![1, 3, 2];

    assert_eq!(2, Solution::find_number_of_lis(n1));
    assert_eq!(5, Solution::find_number_of_lis(n2));
    assert_eq!(3, Solution::find_number_of_lis(n3));
    assert_eq!(2, Solution::find_number_of_lis(n4));
}
