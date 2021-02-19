mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![1, 2, 2, 3, 1];
    let nums2 = vec![1, 2, 2, 3, 1, 4, 2];

    // Expecting 2
    println!("{}", Solution::find_shortest_sub_array(nums1));
    // Expecting 6
    println!("{}", Solution::find_shortest_sub_array(nums2));
}
