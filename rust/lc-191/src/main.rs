mod solution;

use solution::Solution;

fn main() {
    assert_eq!(3, Solution::hammingWeight(11));
    assert_eq!(1, Solution::hammingWeight(128));
    assert_eq!(31, Solution::hammingWeight(4294967293));
}
