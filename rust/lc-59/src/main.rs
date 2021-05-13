mod solution;

use solution::Solution;

fn main() {
    // Expecting [[1, 2, 3], [8, 9, 4], [7, 6, 5]]
    println!("{:?}", Solution::generate_matrix(3));
}
