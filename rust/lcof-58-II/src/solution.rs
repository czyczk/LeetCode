pub struct Solution {}

impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        let n = n as usize;
        let mut s = s.as_bytes().to_owned();
        let len = s.len();

        Solution::reverse(&mut s[..]);
        Solution::reverse(&mut s[0..len - n]);
        Solution::reverse(&mut s[len - n..len]);

        return String::from_utf8(s).unwrap();
    }

    fn reverse(s: &mut [u8]) {
        let (mut head, mut tail) = (0, s.len() - 1);
        while head < tail {
            let temp = s[head];
            s[head] = s[tail];
            s[tail] = temp;

            head += 1;
            tail -= 1;
        }
    }
}
