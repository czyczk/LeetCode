pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        // 0: right, 1: down, 2: left, 3: up
        let mut cur_direction = 0usize;
        let mut total_steps = vec![n, n - 1, n - 1, n - 2];
        let mut remaining_steps = total_steps.clone();
        let mut ret = vec![vec![0; n]; n];

        remaining_steps[0] -= 1;
        ret[0][0] = 1;
        let mut cnt = 1;
        let (mut i, mut j) = (0, 0);

        while cnt < n * n {
            if remaining_steps[cur_direction] > 0 {
                remaining_steps[cur_direction] -= 1;
                // Update the next coordinate (position) in the current direction
                match cur_direction {
                    0 => j += 1,
                    1 => i += 1,
                    2 => j -= 1,
                    3 => i -= 1,
                    _ => panic!(),
                }
                // Set the matrix in the position with the desired number and update the count
                cnt += 1;
                ret[i][j] = cnt as i32;
            } else {
                // Each time the remaining steps of a direction are exausted, sub
                // total_steps of the current direction by 2.
                total_steps[cur_direction] = total_steps[cur_direction].checked_sub(2).unwrap_or(0);
                // Reset the remaining_steps of the current direction.
                remaining_steps[cur_direction] = total_steps[cur_direction];
                // Change the direction clockwise.
                cur_direction = (cur_direction + 1) % 4;
            }
        }

        ret
    }
}
