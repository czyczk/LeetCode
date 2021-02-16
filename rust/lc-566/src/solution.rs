pub struct Solution {}

impl Solution {
    pub fn matrix_reshape(nums: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = nums.len();
        let n = nums[0].len();
        let (r, c) = (r as usize, c as usize);

        if m * n != r * c {
            return nums;
        }

        let mut ret = vec![vec![0; c]; r];

        for (i, row) in nums.iter().enumerate() {
            for (j, &num) in row.iter().enumerate() {
                let (new_i, new_j) = to_new_idx(i, j, n, c);
                ret[new_i][new_j] = num;
            }
        }

        ret
    }
}

fn to_new_idx(i: usize, j: usize, n: usize, c: usize) -> (usize, usize) {
    let idx = i * n + j;
    (idx / c, idx % c)
}
