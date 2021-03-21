pub struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut first_row_has_0 = false;
        let mut first_col_has_0 = false;

        for num in matrix[0].iter() {
            if num == &0 {
                first_row_has_0 = true;
                break;
            }
        }

        for i in 0..n {
            if matrix[i][0] == 0 {
                first_col_has_0 = true;
                break;
            }
        }

        for i in 1..n {
            for j in 1..m {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..n {
            for j in 1..m {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if first_row_has_0 {
            for j in 0..m {
                matrix[0][j] = 0;
            }
        }

        if first_col_has_0 {
            for i in 0..n {
                matrix[i][0] = 0;
            }
        }
    }
}
