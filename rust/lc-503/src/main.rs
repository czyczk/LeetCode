mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![1, 2, 1];
    let nums2 = vec![2, 3, 1];
    let nums3 = vec![6, 5, 4, 3, 8];

    // Expecting [2, -1, 2]
    println!("{:?}", Solution::next_greater_elements(nums1));
    // Expecting [3, -1, 2]
    println!("{:?}", Solution::next_greater_elements(nums2));
    // Expecting [8, 8, 8, 8, -1]
    println!("{:?}", Solution::next_greater_elements(nums3));
}
