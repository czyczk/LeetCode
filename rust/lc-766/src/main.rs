mod solution;

use solution::Solution;

fn main() {
    let matrix1 = vec![
        vec![1, 2, 3, 4],
        vec![5, 1, 2, 3],
        vec![9, 5, 1, 2],
    ];

    let matrix2 = vec![
        vec![1, 2],
        vec![2, 2],
    ];

    // Expecting true
    println!("{}", Solution::is_toeplitz_matrix(matrix1));
    // Expecting false
    println!("{}", Solution::is_toeplitz_matrix(matrix2));
}
