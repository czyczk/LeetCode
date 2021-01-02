mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k1 = 3;
    let nums2 = vec![7, 2, 4];
    let k2 = 2;

    // Expecting [3, 3, 5, 5, 6, 7]
    println!("{:?}", Solution::max_sliding_window(nums1, k1));
    // Expecting [7, 4]
    println!("{:?}", Solution::max_sliding_window(nums2, k2));
}
