mod solution;

use solution::Solution;

fn main() {
    let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    let k1 = 3;

    let mut nums2 = vec![-1, -100, 3, 99];
    let k2 = 2;

    // Expecting [5, 6, 7, 1, 2, 3, 4]
    Solution::rotate(&mut nums1, k1);
    println!("{:?}", nums1);
    // Expecting [3, 99, -1, -100]
    Solution::rotate(&mut nums2, k2);
    println!("{:?}", nums2);
}
