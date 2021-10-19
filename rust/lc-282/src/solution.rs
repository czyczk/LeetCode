use std::cell::Cell;

pub struct Solution {}

thread_local! {static ANS: Cell<Vec<String>> = Cell::new(vec![]);}
thread_local! {static TRACE: Cell<String> = Cell::new(String::new());}

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        Self::backtrace_rec(&num.as_bytes(), target as i64, Operation::Plus);
        ANS.with(|ans_cell| ans_cell.take())
    }

    fn backtrace_rec(chars: &[u8], target: i64, op: Operation) {
        if chars.is_empty() {
            return;
        }

        let mut num = 0i64;
        for (ch_idx, &ch) in chars.iter().enumerate() {
            if ch_idx > 0 && num == 0 {
                // Exclude numbers with leading zeroes.
                break;
            }

            num *= 10;
            num += to_num(ch);
            TRACE.with(|trace_cell| {
                let mut trace = trace_cell.take();
                let original_trace = trace.clone();

                match op {
                    Operation::Plus => {
                        trace.push_str(&format!("+{}", num));
                    }
                    Operation::Minus => {
                        trace.push_str(&format!("-{}", num));
                    }
                    Operation::Multiply => {
                        trace.push_str(&format!("*{}", num));
                    }
                }

                if ch_idx == chars.len() - 1 {
                    let sum = Self::evaluate(&trace.as_bytes()[1..]);
                    if sum == target {
                        ANS.with(|ans_cell| {
                            let mut ans = ans_cell.take();
                            ans.push(trace[1..].to_owned());
                            ans_cell.set(ans);
                        })
                    }
                }

                trace_cell.set(trace);
                Self::backtrace_rec(&chars[ch_idx + 1..], target, Operation::Plus);
                Self::backtrace_rec(&chars[ch_idx + 1..], target, Operation::Minus);
                Self::backtrace_rec(&chars[ch_idx + 1..], target, Operation::Multiply);

                trace_cell.set(original_trace);
            });
        }
    }

    pub fn evaluate(exp: &[u8]) -> i64 {
        let mut op_stack = vec![];
        let mut multiply_stack = vec![];
        let mut plus_stack = vec![0];
        let mut num = 0i64;
        for &ch in exp.iter() {
            match ch {
                b'*' => {
                    if op_stack.is_empty()
                        || !op_stack.is_empty() && *op_stack.last().unwrap() != Operation::Multiply
                    {
                        if op_stack.last() == Some(&Operation::Minus) {
                            op_stack.pop();
                            op_stack.push(Operation::Plus);
                            multiply_stack.push(-num);
                        } else {
                            multiply_stack.push(num);
                        }
                        op_stack.push(Operation::Multiply);
                    } else {
                        let last_num = multiply_stack.pop().unwrap();
                        multiply_stack.push(last_num * num);
                    }

                    num = 0;
                }
                b'+' | b'-' => {
                    if !op_stack.is_empty() {
                        if *op_stack.last().unwrap() == Operation::Multiply {
                            op_stack.pop().unwrap();
                            let last_num = multiply_stack.pop().unwrap();
                            plus_stack.push(last_num * num);
                        } else {
                            let last_num = plus_stack.pop().unwrap();
                            if op_stack.pop().unwrap() == Operation::Plus {
                                plus_stack.push(last_num + num);
                            } else {
                                plus_stack.push(last_num - num);
                            }
                        }
                    } else {
                        let last_num = plus_stack.pop().unwrap();
                        plus_stack.push(last_num + num);
                    }

                    op_stack.push(if ch == b'+' {
                        Operation::Plus
                    } else {
                        Operation::Minus
                    });
                    num = 0;
                }
                digit => {
                    num *= 10;
                    num += to_num(digit);
                }
            }
        }

        while !op_stack.is_empty() {
            match op_stack.pop().unwrap() {
                Operation::Multiply => {
                    let last_num = multiply_stack.pop().unwrap();
                    num *= last_num;
                }
                Operation::Plus => {
                    let last_num = plus_stack.pop().unwrap();
                    num += last_num;
                }
                Operation::Minus => {
                    let last_num = plus_stack.pop().unwrap();
                    num = last_num - num;
                }
            }
        }

        num
    }
}

fn to_num(ch: u8) -> i64 {
    (ch - b'0') as i64
}

#[derive(PartialEq, Eq)]
enum Operation {
    Plus,
    Minus,
    Multiply,
}
