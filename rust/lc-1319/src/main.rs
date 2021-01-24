mod solution;

use solution::Solution;

fn main() {
    let n1 = 4;
    let c1 = vec![vec![0, 1], vec![0, 2], vec![1, 2]];
    let n2 = 6;
    let c2 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]];
    let n3 = 6;
    let c3 = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]];
    let n4 = 5;
    let c4 = vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]];

    // Expecting 1
    println!("{}", Solution::make_connected(n1, c1));
    // Expecting 2
    println!("{}", Solution::make_connected(n2, c2));
    // Expecting -1
    println!("{}", Solution::make_connected(n3, c3));
    // Expecting 0
    println!("{}", Solution::make_connected(n4, c4));
}
