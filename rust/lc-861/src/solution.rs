pub struct Solution {}

impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let num_rows = a.len();
        let num_cols = a[0].len();

        let mut rows_inverted = vec![false; num_rows];

        // Ensure that the first column is all 1
        for i in 0..num_rows {
            if a[i][0] == 0 {
                rows_inverted[i] = true;
            }
        }

        let mut score = num_rows * 1 << (num_cols - 1);

        if num_cols < 2 {
            return score as i32;
        }

        // Scan from the second column to ensure the max score
        for j in 1..num_cols {
            let mut num_ones = 0;

            for i in 0..num_rows {
                if (a[i][j] == 1) ^ rows_inverted[i] {
                    num_ones += 1;
                }
            }

            if num_ones < (num_rows + 1) / 2 {
                num_ones = num_rows - num_ones;
            }

            score += num_ones * 1 << (num_cols - j - 1);
        }

        return score as i32;
    }
}
