pub struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        check_rows(&board) && check_cols(&board) && check_blocks(&board)
    }
}

fn check_rows(board: &Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        let mut is_used = [false; 9];
        for j in 0..9 {
            match board[i][j] {
                '.' => continue,
                num => {
                    let idx = to_idx(num);
                    if is_used[idx] {
                        return false;
                    }
                    is_used[idx] = true;
                }
            }
        }
    }

    true
}

fn check_cols(board: &Vec<Vec<char>>) -> bool {
    for j in 0..9 {
        let mut is_used = [false; 9];
        for i in 0..9 {
            match board[i][j] {
                '.' => continue,
                num => {
                    let idx = to_idx(num);
                    if is_used[idx] {
                        return false;
                    }
                    is_used[idx] = true;
                }
            }
        }
    }

    true
}

fn check_blocks(board: &Vec<Vec<char>>) -> bool {
    for start_i in (0..9).step_by(3) {
        for start_j in (0..9).step_by(3) {
            let mut is_used = [false; 9];

            for i in start_i..start_i + 3 {
                for j in start_j..start_j + 3 {
                    match board[i][j] {
                        '.' => continue,
                        num => {
                            let idx = to_idx(num);
                            if is_used[idx] {
                                return false;
                            }
                            is_used[idx] = true;
                        }
                    }
                }
            }
        }
    }

    true
}

fn to_idx(ch: char) -> usize {
    ch as usize - '1' as usize
}
