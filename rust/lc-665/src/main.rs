mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![4, 2, 3];
    let nums2 = vec![4, 2, 1];
    let nums3 = vec![3, 4, 2, 3];

    // Expecting true
    println!("{}", Solution::check_possibility(nums1));
    // Expecting false
    println!("{}", Solution::check_possibility(nums2));
    // Expecting false
    println!("{}", Solution::check_possibility(nums3));
}
