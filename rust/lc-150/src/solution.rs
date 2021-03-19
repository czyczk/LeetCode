pub struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];
        for s in tokens.iter() {
            let s_as_bytes = s.as_bytes();
            if s_as_bytes[0] >= b'0'
                || (s_as_bytes.len() > 1 && s_as_bytes[0] == b'-' && s_as_bytes[1] >= b'0')
            {
                stack.push(s.parse::<i32>().unwrap());
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match s_as_bytes[0] {
                    b'+' => stack.push(a + b),
                    b'-' => stack.push(a - b),
                    b'*' => stack.push(a * b),
                    b'/' => stack.push(a / b),
                    _ => panic!(),
                }
            }
        }

        *stack.last().unwrap()
    }
}
