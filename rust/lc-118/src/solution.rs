use std::collections::HashMap;

pub struct Solution { }

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let should_early_return = handle_early_returns(num_rows, &mut result);
        if should_early_return {
            return result;
        }

        for i in 3..=num_rows {
            let mut row = vec![];
            row.push(1);

            for j in 1..i - 1 {
                row.push(result[(i - 2) as usize][(j - 1) as usize] + result[(i - 2) as usize][j as usize]);
            }

            row.push(1);
            result.push(row);
        }
        
        return result;
    }
}

fn handle_early_returns(num_rows: i32, result: &mut Vec<Vec<i32>>) -> bool {
    if num_rows == 0 {
        return true;
    }

    let mut row = vec![];
    row.push(1);
    result.push(row);

    if num_rows == 1 {
        return true;
    }

    let mut row = vec![];
    row.push(1);
    row.push(1);
    result.push(row);
    
    if num_rows == 2 {
        return true;
    }

    return false;
}