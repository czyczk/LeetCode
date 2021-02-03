mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![1, 12, -5, -6, 50, 3];
    let k1 = 4;

    // Expecting 12.75
    println!("{}", Solution::find_max_average(nums1, k1));
}
