pub struct Solution {}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut f = vec![vec![1; len]; len];

        for j in 0..len {
            for i in (0..=j).rev() {
                if j - i < 2 {
                    if s[i] == s[j] {
                        f[i][j] = j - i + 1;
                    } else {
                        // 1 is instantiated by default
                    }
                } else {
                    if s[i] == s[j] {
                        f[i][j] = f[i + 1][j - 1] + 2;
                    } else {
                        f[i][j] = f[i + 1][j].max(f[i][j - 1]);
                    }
                }
            }
        }

        return f[0][len - 1] as i32;
    }
}
