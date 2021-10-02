mod solution;

use solution::Solution;

fn main() {
    assert_eq!("1a", Solution::to_hex(26));
    assert_eq!("ffffffff", Solution::to_hex(-1));
    assert_eq!("0", Solution::to_hex(0));
}
