pub struct Solution {}

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut stack_is_left = vec![false];
        let preorder = preorder.as_bytes();
        let n = preorder.len();
        let mut num = 0;

        for (i, &ch) in preorder.iter().enumerate() {
            if stack_is_left.is_empty() {
                return false;
            }

            let mut should_op = i == n - 1;

            match ch {
                b',' => should_op = preorder[i - 1] != b'#',
                b'#' => {
                    should_op = false;
                    if *stack_is_left.last().unwrap() {
                        *stack_is_left.last_mut().unwrap() = false;
                    } else {
                        stack_is_left.pop();
                    }
                }
                _ => num = num * 10 + (ch - b'0') as i32,
            }

            if should_op {
                if *stack_is_left.last().unwrap() {
                    *stack_is_left.last_mut().unwrap() = false;
                    stack_is_left.push(true);
                } else {
                    *stack_is_left.last_mut().unwrap() = true;
                }

                num = 0;
            }
        }

        if !stack_is_left.is_empty() {
            return false;
        }

        true
    }
}
