mod solution;

use solution::Solution;

fn main() {
    let n1 = 4;
    let n2 = 25;

    assert_eq!(4, Solution::tribonacci(n1));
    assert_eq!(1389537, Solution::tribonacci(n2));
}
