mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![2, 3, 4];
    let k1 = 1;
    let nums2 = vec![1, 3, -1, -3, 5, 3, 6, 7];
    let k2 = 3;
    let nums3 = vec![2, 3, 4];
    let k3 = 2;
    let nums4 = vec![2147483647, 2147483647];
    let k4 = 2;
    let nums5 = vec![9, 7, 0, 3, 9, 8, 6, 5, 7, 6];
    let k5 = 2;

    // Expecting [2, 3, 4]
    println!("{:?}", Solution::median_sliding_window(nums1, k1));
    // Expecting [1, -1, -1, 3, 5, 6]
    println!("{:?}", Solution::median_sliding_window(nums2, k2));
    // Expecting [2.5, 3.5]
    println!("{:?}", Solution::median_sliding_window(nums3, k3));
    // Expecting [2147483647]
    println!("{:?}", Solution::median_sliding_window(nums4, k4));
    // Expecting [8, 3.5, 1.5, 6, 8.5, 7, 5.5, 6, 6.5]
    println!("{:?}", Solution::median_sliding_window(nums5, k5));
}
