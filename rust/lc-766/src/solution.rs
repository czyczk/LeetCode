pub struct Solution {}

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut i = m - 1;
        let mut j = 0;
        while j < n {
            let num = matrix[i][j];
            let mut x = i + 1;
            let mut y = j + 1;
            while x < m && y < n {
                if matrix[x][y] != num {
                    return false;
                }
                x += 1;
                y += 1;
            }

            if i == 0 {
                j += 1;
            } else {
                i -= 1;
            }
        }

        true
    }
}
