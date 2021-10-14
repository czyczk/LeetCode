mod solution;

use solution::Solution;

fn main() {
    assert_eq!("1", Solution::count_and_say(1));
    assert_eq!("1211", Solution::count_and_say(4));
}
