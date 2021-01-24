mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![1, 3, 5, 4, 7];
    let nums2 = vec![2, 2, 2, 2, 2];
    let nums3 = vec![];

    // Expecting 3
    println!("{}", Solution::find_length_of_lcis(nums1));
    // Expecting 1
    println!("{}", Solution::find_length_of_lcis(nums2));
    // Expecting 0
    println!("{}", Solution::find_length_of_lcis(nums3));
}
