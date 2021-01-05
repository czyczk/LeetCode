mod solution;

use solution::Solution;

fn main() {
    let nums1 = vec![2, 3, 1, 0, 2, 5, 3];

    // Expecting 2 or 3
    println!("{}", Solution::find_repeat_number(nums1));
}
