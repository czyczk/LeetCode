pub struct Solution {}

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        if k == 1 {
            return s;
        }

        let mut s = s.as_bytes().to_owned();
        let k = k as usize;
        let len = s.len();

        let mut should_reverse = true;
        let (mut start, mut end) = (0, k);
        while start < len {
            if should_reverse {
                let mut should_break = false;

                if end > len {
                    end = len;
                    should_break = true;
                }

                Solution::reverse(&mut s[start..end]);

                if should_break {
                    break;
                }
            }

            start = end;
            end += k;
            should_reverse = !should_reverse;
        }

        return String::from_utf8(s.to_owned()).unwrap();
    }

    fn reverse(s: &mut [u8]) {
        let len = s.len();
        if len == 0 {
            return;
        }

        let (mut head, mut tail) = (0, len - 1);
        while head < tail {
            let temp = s[head];
            s[head] = s[tail];
            s[tail] = temp;
            head += 1;
            tail -= 1;
        }
    }
}
