mod solution;
use solution::Solution;
use solution::SolutionOneLiner;

fn main() {
    let s1 = "abcd".to_owned();
    let t1 = "abcde".to_owned();

    let s2 = "".to_owned();
    let t2 = "y".to_owned();

    let s3 = "a".to_owned();
    let t3 = "aa".to_owned();

    let s4 = "ae".to_owned();
    let t4 = "aea".to_owned();

    // Expecting 'e'
    println!("{}", Solution::find_the_difference(s1.clone(), t1.clone()));
    println!("{}", SolutionOneLiner::find_the_difference(s1, t1));
    // Expecting 'y'
    println!("{}", Solution::find_the_difference(s2.clone(), t2.clone()));
    println!("{}", SolutionOneLiner::find_the_difference(s2, t2));
    // Expecting 'a'
    println!("{}", Solution::find_the_difference(s3.clone(), t3.clone()));
    println!("{}", SolutionOneLiner::find_the_difference(s3, t3));
    // Expecting 'a'
    println!("{}", Solution::find_the_difference(s4.clone(), t4.clone()));
    println!("{}", SolutionOneLiner::find_the_difference(s4, t4));
}
