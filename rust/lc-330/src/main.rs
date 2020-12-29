mod solution;

use solution::Solution;

fn main() {
    let (nums1, n1) = (vec![1, 3], 6);
    let (nums2, n2) = (vec![1, 5, 10], 20);
    let (nums3, n3) = (vec![1, 2, 2], 5);
    let (nums4, n4) = (vec![1,2,31,33], 2147483647);

    // Expecting 1
    println!("{}", Solution::min_patches(nums1, n1));
    // Expecting 2
    println!("{}", Solution::min_patches(nums2, n2));
    // Expecting 0
    println!("{}", Solution::min_patches(nums3, n3));
    // Expecting 28
    println!("{}", Solution::min_patches(nums4, n4));
}
