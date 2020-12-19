pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n - 1 {
            for j in i..n - i - 1 {
               let (i2, j2) = (j, n - i - 1);
               let (i3, j3) = (n - i - 1, n - j - 1);
               let (i4, j4) = (n - j - 1, i);

               let (val1, val2, val3, val4) = (matrix[i][j], matrix[i2][j2], matrix[i3][j3], matrix[i4][j4]);

               matrix[i][j] = val4;
               matrix[i2][j2] = val1;
               matrix[i3][j3] = val2;
               matrix[i4][j4] = val3;
            }
        }
    } 
}