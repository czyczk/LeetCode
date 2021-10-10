mod solution;

use solution::Solution;

fn main() {
    assert_eq!(2, Solution::arrange_coins(5));
    assert_eq!(3, Solution::arrange_coins(8));
    assert_eq!(0, Solution::arrange_coins(0));
    assert_eq!(60070, Solution::arrange_coins(1804289383));
}
