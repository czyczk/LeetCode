pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut stack = vec![];
        let (mut num, mut last_op) = (0i32, b'+');

        for (i, &ch) in s.iter().enumerate() {
            let mut should_op = false;
            if i == n - 1 {
                should_op = true;
            }

            match ch {
                b' ' => (),
                ch if ch >= b'0' => {
                    num = num * 10 + (ch - b'0') as i32;
                }
                _ => {
                    should_op = true;
                }
            }

            if should_op {
                match last_op {
                    b'+' => {
                        stack.push(num);
                    }
                    b'-' => {
                        stack.push(-num);
                    }
                    b'*' => {
                        *stack.last_mut().unwrap() *= num;
                    }
                    _ => {
                        *stack.last_mut().unwrap() /= num;
                    }
                }

                num = 0;
                last_op = ch;
            }
        }

        stack.iter().sum()
    }
}
