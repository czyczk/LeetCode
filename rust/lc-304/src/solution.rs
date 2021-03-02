pub struct NumMatrix {
    sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        if m == 0 {
            return Self {
                sum: vec![vec![0]],
            }
        }

        let n = matrix[0].len();
        let mut sum = vec![vec![0; n]; m];

        for (i, line) in matrix.iter().enumerate() {
            for (j, &num) in line.iter().enumerate() {
                if j == 0 {
                    if i == 0 {
                        sum[i][j] = num;
                    } else {
                        sum[i][j] = sum[i - 1][j] + num;
                    }
                } else {
                    if i == 0 {
                        sum[i][j] = sum[i][j - 1] + num;
                    } else {
                        sum[i][j] = sum[i][j - 1] + sum[i - 1][j] - sum[i - 1][j - 1] + num;
                    }
                }
            }
        }

        Self {
            sum,
        }
    }
    
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        let mut ret = self.sum[row2][col2];

        if row1 > 0 {
            ret -= self.sum[row1 - 1][col2];
        }

        if col1 > 0 {
            ret -= self.sum[row2][col1 - 1];
        }

        if row1 > 0 && col1 > 0 {
            ret += self.sum[row1 - 1][col1 - 1];
        }

        ret
    }
}
