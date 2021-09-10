pub mod solution;

use solution::Solution;

fn main() {
    let (c1, k1) = (vec![5, 1, 5], 22);
    let (c2, k2) = (vec![3, 4, 1, 2], 25);
    let (c3, k3) = (vec![1], 1000000000);
    let (c4, k4) = (vec![1, 2, 3, 5], 1000000000);

    assert_eq!(0, Solution::chalk_replacer(c1, k1));
    assert_eq!(1, Solution::chalk_replacer(c2, k2));
    assert_eq!(0, Solution::chalk_replacer(c3, k3));
    assert_eq!(3, Solution::chalk_replacer(c4, k4));
}
