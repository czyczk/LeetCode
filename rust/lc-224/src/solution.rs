pub struct Solution {}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut op_stack = vec![1i32];
        let (mut ans, mut num, mut op) = (0i32, 0i32, 1i32);

        for ch in s.bytes() {
            match ch {
                b' ' => continue,
                ch if ch >= b'0' => {
                    num = num * 10 + (ch - b'0') as i32;
                }
                _ => {
                    ans += op * num;
                    num = 0;

                    match ch {
                        b'+' => op = *op_stack.last().unwrap(),
                        b'-' => op = -(*op_stack.last().unwrap()),
                        b'(' => op_stack.push(op),
                        b')' => {
                            op_stack.pop();
                        }
                        _ => panic!(),
                    }
                }
            }
        }

        ans + op * num
    }
}
