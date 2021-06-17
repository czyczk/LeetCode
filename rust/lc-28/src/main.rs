mod solution;

use solution::Solution;

fn main() {
    let (h1, n1) = ("hello".to_string(), "ll".to_string());
    let (h2, n2) = ("aaaaa".to_string(), "bba".to_string());
    let (h3, n3) = ("".to_string(), "".to_string());

    assert_eq!(2, Solution::str_str(h1, n1));
    assert_eq!(-1, Solution::str_str(h2, n2));
    assert_eq!(0, Solution::str_str(h3, n3));
}
