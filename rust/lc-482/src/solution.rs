pub struct Solution {}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut ret_bytes = std::collections::VecDeque::new();
        let n = s.len();
        let s = s.as_bytes();
        let mut count = 0;
        let mut is_first = true;

        for i in (0..n).rev() {
            if s[i] == b'-' {
                continue;
            }

            if !is_first && count == 0 {
                ret_bytes.push_front(b'-');
            }

            ret_bytes.push_front(s[i].to_ascii_uppercase());
            count = (count + 1) % k;

            if is_first {
                is_first = false;
            }
        }

        String::from_utf8(Vec::from(ret_bytes)).unwrap()
    }
}
