pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = vec![];
        for ch in s.bytes() {
            match stack.last() {
                Some(&last_ch) if last_ch == ch => {
                    stack.pop();
                }
                _ => stack.push(ch),
            }
        }

        String::from_utf8(stack).unwrap()
    }
}
