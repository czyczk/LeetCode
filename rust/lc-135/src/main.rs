mod solution;

use solution::Solution;

fn main() {
    let rating1 = vec![1, 0, 2];
    let rating2 = vec![1, 2, 2];
    let rating3 = vec![];
    let rating4 = vec![1, 3, 4, 5, 2];

    // Expecting [2, 1, 2] => 5
    println!("{}", Solution::candy(rating1));
    // Expecting [1, 2, 1] => 4
    println!("{}", Solution::candy(rating2));
    // Expecting [] => 0
    println!("{}", Solution::candy(rating3));
    // Expecting 11
    println!("{}", Solution::candy(rating4));
}
