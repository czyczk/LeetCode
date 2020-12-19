mod solution;

use solution::Solution;

fn main() {
    let mut matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let mut matrix2 = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];

    Solution::rotate(&mut matrix1);
    Solution::rotate(&mut matrix2);

    print_matrix(&matrix1);
    print_matrix(&matrix2);
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..n {
        for j in 0..n {
            print!("{} ", matrix[i][j]);
        }
        println!()
    }
}
