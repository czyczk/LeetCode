mod solution;

fn main() {
    let nums1 = vec![3, 6, 9, 1];
    let nums2 = vec![10];
    let nums3 = vec![3, 7, 13, 4];
    let nums4 = vec![1, 100000000];

    // Expecting 3
    println!("{}", solution::Solution::maximum_gap(nums1));
    // Expecting 0
    println!("{}", solution::Solution::maximum_gap(nums2));
    // Expecting 6
    println!("{}", solution::Solution::maximum_gap(nums3));
    // Expecting 99999999
    println!("{}", solution::Solution::maximum_gap(nums4));
}
