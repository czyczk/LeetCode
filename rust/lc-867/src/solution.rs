pub struct Solution {}

impl Solution {
    pub fn transpose(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();

        if m == n {
            for i in 0..n {
                for j in i + 1..n {
                    unsafe {
                        std::ptr::swap(&mut matrix[i][j] as *mut i32, &mut matrix[j][i] as *mut i32);
                    }
                }
            }

            return matrix;
        }

        let mut ret = vec![Vec::with_capacity(m); n];

        for line in matrix.iter() {
            for (j, &num) in line.into_iter().enumerate()  {
                ret[j].push(num);
            }
        }

        ret
    }
}
