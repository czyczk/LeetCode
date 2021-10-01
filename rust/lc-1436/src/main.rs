mod solution;

use solution::Solution;

fn main() {
    let paths1 = vec![
        vec!["London".to_owned(), "New York".to_owned()],
        vec!["New York".to_owned(), "Lima".to_owned()],
        vec!["Lima".to_owned(), "Sao Paulo".to_owned()],
    ];

    let paths2 = vec![
        vec!["B".to_owned(), "C".to_owned()],
        vec!["D".to_owned(), "B".to_owned()],
        vec!["C".to_owned(), "A".to_owned()],
    ];

    let paths3 = vec![vec!["A".to_owned(), "Z".to_owned()]];

    assert_eq!("Sao Paulo", Solution::dest_city(paths1));
    assert_eq!("A", Solution::dest_city(paths2));
    assert_eq!("Z", Solution::dest_city(paths3));
}
