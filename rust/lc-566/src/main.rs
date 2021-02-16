mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![vec![1, 2], vec![3, 4]];
    let (r1, c1) = (1, 4);
    let nums2 = vec![vec![1, 2], vec![3, 4]];
    let (r2, c2) = (2, 4);

    // Expecting [[1, 2, 3, 4]]
    println!("{:?}", Solution::matrix_reshape(nums1, r1, c1));
    // Expecting [[1, 2], [3, 4]] (Cannot reshape. Keep original.)
    println!("{:?}", Solution::matrix_reshape(nums2, r2, c2));
}
