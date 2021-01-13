mod solution;

use solution::Solution;

fn main() {
    let edges1 = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let edges2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]];

    // Expecting [2, 3]
    println!("{:?}", Solution::find_redundant_connection(edges1));
    // Expecting [1, 4]
    println!("{:?}", Solution::find_redundant_connection(edges2));
}
