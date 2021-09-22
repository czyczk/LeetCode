mod solution;

use solution::Solution;

fn main() {
    assert_eq!(5, Solution::find_integers(5));
    assert_eq!(2, Solution::find_integers(1));
    assert_eq!(3, Solution::find_integers(2));
    assert_eq!(2178309, Solution::find_integers(1000000000));
}
