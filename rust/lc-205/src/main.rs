mod solution;

use solution::Solution;

fn main() {
    let (s1, t1) = ("egg".to_owned(), "add".to_owned());
    let (s2, t2) = ("foo".to_owned(), "bar".to_owned());
    let (s3, t3) = ("paper".to_owned(), "title".to_owned());
    let (s4, t4) = ("13".to_owned(), "42".to_owned());

    // Expecting true
    println!("{}", Solution::is_isomorphic(s1, t1));
    // Expecting false
    println!("{}", Solution::is_isomorphic(s2, t2));
    // Expecting true
    println!("{}", Solution::is_isomorphic(s3, t3));
    // Expecting true
    println!("{}", Solution::is_isomorphic(s4, t4));
}
