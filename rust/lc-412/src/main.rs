mod solution;

use solution::Solution;

fn main() {
    check_eq(
        to_vec_of_string(vec!["1", "2", "Fizz"]),
        Solution::fizz_buzz(3),
    );
    check_eq(
        to_vec_of_string(vec!["1", "2", "Fizz", "4", "Buzz"]),
        Solution::fizz_buzz(5),
    );
    check_eq(
        to_vec_of_string(vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ]),
        Solution::fizz_buzz(15),
    );
}

fn check_eq(expected: Vec<String>, actual: Vec<String>) {
    assert_eq!(format!("{:?}", expected), format!("{:?}", actual));
}

fn to_vec_of_string(vec: Vec<&str>) -> Vec<String> {
    vec.into_iter().map(|str| str.to_owned()).collect()
}
