mod solution;

use solution::Solution;

fn main() {
    let m1 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let m2 = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    // Expecting [[1, 4, 7], [2, 5, 8], [3, 6, 9]]
    println!("{:?}", Solution::transpose(m1));
    // Expecting [[1, 4], [2, 5], [3, 6]]
    println!("{:?}", Solution::transpose(m2));
}
