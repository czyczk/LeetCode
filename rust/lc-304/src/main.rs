mod solution;

use solution::NumMatrix;

fn main() {
    let matrix1 = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5],
    ];

    let nm = NumMatrix::new(matrix1);
    // Expecting 8
    println!("{}", nm.sum_region(2, 1, 4, 3));
    // Expecting 11
    println!("{}", nm.sum_region(1, 1, 2, 2));
    // Expecting 12
    println!("{}", nm.sum_region(1, 2, 2, 4));
}
