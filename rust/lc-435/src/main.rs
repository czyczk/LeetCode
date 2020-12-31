mod solution;

use solution::Solution;

fn main() {
    let i1 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    let i2 = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
    let i3 = vec![vec![1, 2], vec![2, 3]];

    // Expecting 1
    println!("{}", Solution::erase_overlap_intervals(i1));
    // Expecting 2
    println!("{}", Solution::erase_overlap_intervals(i2));
    // Expecting 0
    println!("{}", Solution::erase_overlap_intervals(i3));
}
