mod solution;

use solution::Solution;

fn main() {
    let tokens1 = to_vec_of_string(vec!["2", "1", "+", "3", "*"]);
    let tokens2 = to_vec_of_string(vec!["4", "13", "5", "/", "+"]);
    let tokens3 = to_vec_of_string(vec![
        "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
    ]);
    let tokens4 = to_vec_of_string(vec!["4", "3", "-"]);
    let tokens5 = to_vec_of_string(vec!["5"]);

    assert_eq!(9, Solution::eval_rpn(tokens1));
    assert_eq!(6, Solution::eval_rpn(tokens2));
    assert_eq!(22, Solution::eval_rpn(tokens3));
    assert_eq!(1, Solution::eval_rpn(tokens4));
    assert_eq!(5, Solution::eval_rpn(tokens5));
}

fn to_vec_of_string(v: Vec<&str>) -> Vec<String> {
    v.into_iter().map(|s| s.to_owned()).collect()
}
