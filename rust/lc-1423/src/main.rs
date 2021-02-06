mod solution;

use solution::Solution;

fn main() {
    let card_points1 = vec![1, 2, 3, 4, 5, 6, 1];
    let k1 = 3;

    let card_points2 = vec![2, 2, 2];
    let k2 = 2;

    let card_points3 = vec![9, 7, 7, 9, 7, 7, 9];
    let k3 = 7;

    let card_points4 = vec![100, 40, 17, 9, 73, 75];
    let k4 = 3;

    // Expecting 12
    println!("{}", Solution::max_score(card_points1, k1));
    // Expecting 4
    println!("{}", Solution::max_score(card_points2, k2));
    // Expecting 55
    println!("{}", Solution::max_score(card_points3, k3));
    // Expecting 248
    println!("{}", Solution::max_score(card_points4, k4));
}
