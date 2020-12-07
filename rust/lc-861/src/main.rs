use solution::Solution;

mod solution;

fn main() {
    let matrix = vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]];

    println!("{}", Solution::matrix_score(matrix));
}
