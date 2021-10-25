pub struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let (mut i, mut j) = (0, n - 1);
        while i < m {
            let num = matrix[i][j];
            match target.cmp(&num) {
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Less => {
                    // j >= 0
                    if j == 0 {
                        return false;
                    }

                    j -= 1;
                }
                std::cmp::Ordering::Greater => {
                    i += 1;
                }
            }
        }

        false
    }
}
