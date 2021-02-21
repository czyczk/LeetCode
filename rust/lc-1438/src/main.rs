mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![8, 2, 4, 7];
    let limit1 = 4;

    let nums2 = vec![10, 1, 2, 4, 7, 2];
    let limit2 = 5;

    let nums3 = vec![4, 2, 2, 2, 4, 4, 2, 2];
    let limit3 = 0;

    let nums4 = vec![1, 5, 6, 7, 8, 10, 6, 5, 6];
    let limit4 = 4;
    
    let nums5 = vec![38, 73, 69, 15, 59, 36, 14, 6, 38, 2, 79, 86, 2, 12, 53, 15, 6, 25, 31, 76, 54, 21, 15, 58, 22, 88, 31, 21, 96, 14, 56, 49, 70, 38, 71, 33, 92, 62, 41, 13, 27, 84, 41, 6, 4, 2, 38, 93, 77, 41, 58, 51, 41, 52, 9, 9, 41, 77, 59, 15, 33, 28, 80, 100, 70, 89, 61];
    let limit5 = 73;

    // Expecting 2
    println!("{}", Solution::longest_subarray(nums1, limit1));
    // Expecting 4
    println!("{}", Solution::longest_subarray(nums2, limit2));
    // Expecting 3
    println!("{}", Solution::longest_subarray(nums3, limit3));
    // Expecting 5
    println!("{}", Solution::longest_subarray(nums4, limit4));
    // Expecting 15
    println!("{}", Solution::longest_subarray(nums5, limit5));
}
