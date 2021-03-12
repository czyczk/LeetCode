mod solution;

use solution::Solution;

fn main() {
    let s1 = "9,3,4,#,#,1,#,#,2,#,6,#,#".to_owned();
    assert_eq!(true, Solution::is_valid_serialization(s1));

    let s2 = "1,#".to_owned();
    assert_eq!(false, Solution::is_valid_serialization(s2));

    let s3 = "9,#,#,1".to_owned();
    assert_eq!(false, Solution::is_valid_serialization(s3));

    let s4 = "9,#,92,#,#".to_owned();
    assert_eq!(true, Solution::is_valid_serialization(s4));

    let s5 = "2048,#,-2,#,#".to_owned();
    assert_eq!(true, Solution::is_valid_serialization(s5));
}
