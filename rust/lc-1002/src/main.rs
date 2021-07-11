mod solution;

use solution::Solution;

fn main() {
    let w1 = vec_str_to_owned(vec!["bella", "label", "roller"]);
    let w2 = vec_str_to_owned(vec!["cool", "lock", "cook"]);

    check_eq(vec!["e", "l", "l"], Solution::common_chars(w1));
    check_eq(vec!["c", "o"], Solution::common_chars(w2));
}

fn vec_str_to_owned(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|s| s.to_owned()).collect()
}

fn check_eq(expected: Vec<&str>, actual: Vec<String>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}
