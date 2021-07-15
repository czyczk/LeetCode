pub struct Solution {}

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        let mut max_scale = 0;

        for (i, line) in matrix.iter().enumerate() {
            for (j, &ch) in line.iter().enumerate() {
                dp[i][j] = ch as i32 - '0' as i32;
                if ch == '1' {
                    max_scale = 1;
                }
            }
        }

        for (mut i, line) in matrix[1..].iter().enumerate() {
            i += 1;
            for (mut j, &ch) in line[1..].iter().enumerate() {
                j += 1;
                if ch == '1'
                    && matrix[i - 1][j] == '1'
                    && matrix[i][j - 1] == '1'
                    && matrix[i - 1][j - 1] == '1'
                {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                    max_scale = max_scale.max(dp[i][j]);
                }
            }
        }

        return max_scale * max_scale;
    }
}
