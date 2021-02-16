mod solution_unionfind;

use solution_unionfind::Solution;

fn main() {
    let row1 = vec![0, 2, 1, 3];
    let row2 = vec![3, 2, 0, 1];

    // Expecting 1
    println!("{}", Solution::min_swaps_couples(row1));
    // Expecting 0
    println!("{}", Solution::min_swaps_couples(row2));
}
