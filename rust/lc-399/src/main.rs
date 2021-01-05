mod solution;

use solution::Solution;

fn main() {
    let (eq1, val1, q1) = (
        vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "c".to_owned()],
        ],
        vec![2.0, 3.0],
        vec![
            vec!["a".to_owned(), "c".to_owned()],
            vec!["b".to_owned(), "a".to_owned()],
            vec!["a".to_owned(), "e".to_owned()],
            vec!["a".to_owned(), "a".to_owned()],
            vec!["x".to_owned(), "x".to_owned()],
        ],
    );

    let (eq2, val2, q2) = (
        vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "c".to_owned()],
            vec!["bc".to_owned(), "cd".to_owned()],
        ],
        vec![1.5, 2.5, 5.0],
        vec![
            vec!["a".to_owned(), "c".to_owned()],
            vec!["c".to_owned(), "b".to_owned()],
            vec!["bc".to_owned(), "cd".to_owned()],
            vec!["cd".to_owned(), "bc".to_owned()],
        ],
    );

    let (eq3, val3, q3) = (
        vec![vec!["a".to_owned(), "b".to_owned()]],
        vec![0.5],
        vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "a".to_owned()],
            vec!["a".to_owned(), "c".to_owned()],
            vec!["x".to_owned(), "y".to_owned()],
        ],
    );

    // Expecting [6.0, 0.5, -1.0, 1.0, -1.0]
    println!("{:?}", Solution::calc_equation(eq1, val1, q1));
    // Expecting [3.75, 0.4, 5.0, 0.2]
    println!("{:?}", Solution::calc_equation(eq2, val2, q2));
    // Expecting [0.5, 2.0, -1.0, -1.0]
    println!("{:?}", Solution::calc_equation(eq3, val3, q3));
}
