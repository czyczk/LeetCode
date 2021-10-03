mod solution;

use solution::Solution;

fn main() {
    assert_eq!("0.5", Solution::fraction_to_decimal(1, 2));
    assert_eq!("2", Solution::fraction_to_decimal(2, 1));
    assert_eq!("0.(6)", Solution::fraction_to_decimal(2, 3));
    assert_eq!("0.(012)", Solution::fraction_to_decimal(4, 333));
    assert_eq!("0.2", Solution::fraction_to_decimal(1, 5));
    assert_eq!("-0.(012)", Solution::fraction_to_decimal(-4, 333));
    assert_eq!("-6.25", Solution::fraction_to_decimal(-50, 8));
    assert_eq!("0", Solution::fraction_to_decimal(0, -5));
    assert_eq!(
        "0.0000000004656612873077392578125",
        Solution::fraction_to_decimal(-1, -2147483648)
    );
}
