mod solution;

use solution::Solution;

fn main() {
    assert_eq!(1, Solution::fib(2));
    assert_eq!(5, Solution::fib(5));
}
