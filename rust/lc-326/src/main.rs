mod solution;

use solution::Solution;

fn main() {
    assert_eq!(true, Solution::is_power_of_three(27));
    assert_eq!(false, Solution::is_power_of_three(0));
    assert_eq!(true, Solution::is_power_of_three(9));
    assert_eq!(false, Solution::is_power_of_three(45));
    assert_eq!(true, Solution::is_power_of_three(1));
}
