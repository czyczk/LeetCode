mod solution;

use solution::Solution;

fn main() {
    let (nums11, nums12) = (vec![4, 1, 2], vec![1, 3, 4, 2]);
    let (nums21, nums22) = (vec![2, 4], vec![1, 2, 3, 4]);

    assert_eq!(
        vec![-1, 3, -1],
        Solution::next_greater_element(nums11, nums12)
    );
    assert_eq!(vec![3, -1], Solution::next_greater_element(nums21, nums22));
}
