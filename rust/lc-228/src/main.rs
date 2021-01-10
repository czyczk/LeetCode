mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![0, 1, 2, 4, 5, 7];
    let nums2 = vec![0, 2, 3, 4, 6, 8, 9];
    let nums3 = vec![];

    // Expecting ["0->2", "4->5", "7"]
    println!("{:?}", Solution::summary_ranges(nums1));
    // Expecting ["0", "2->4", "6", "8->9"]
    println!("{:?}", Solution::summary_ranges(nums2));
    // Expecting []
    println!("{:?}", Solution::summary_ranges(nums3));
}
