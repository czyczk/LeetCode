mod solution;

use solution::Solution;

fn main() {
    let s1 = "()".to_owned();
    let s2 = "(*)".to_owned();
    let s3 = "(*))".to_owned();
    let s4 = "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".to_owned();

    assert_eq!(true, Solution::check_valid_string(s1));
    assert_eq!(true, Solution::check_valid_string(s2));
    assert_eq!(true, Solution::check_valid_string(s3));
    assert_eq!(false, Solution::check_valid_string(s4));
}
