mod solution;

use solution::Solution;

fn main() {
    assert_eq!(false, Solution::can_win_nim(4));
    assert_eq!(true, Solution::can_win_nim(1));
    assert_eq!(true, Solution::can_win_nim(2));
}
