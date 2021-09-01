mod solution;

use solution::Solution;

fn main() {
    let (v11, v12) = ("1.01".to_owned(), "1.001".to_owned());
    let (v21, v22) = ("1.0".to_owned(), "1.0.0".to_owned());
    let (v31, v32) = ("0.1".to_owned(), "1.1".to_owned());
    let (v41, v42) = ("1.0.1".to_owned(), "1".to_owned());
    let (v51, v52) = ("7.5.2.4".to_owned(), "7.5.3".to_owned());

    assert_eq!(0, Solution::compare_version(v11, v12));
    assert_eq!(0, Solution::compare_version(v21, v22));
    assert_eq!(-1, Solution::compare_version(v31, v32));
    assert_eq!(1, Solution::compare_version(v41, v42));
    assert_eq!(-1, Solution::compare_version(v51, v52));
}
