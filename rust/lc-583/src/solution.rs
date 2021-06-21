pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let len1 = word1.len();
        let len2 = word2.len();
        let mut f = vec![vec![0; len1 + 1]; len2 + 1];

        for j in 0..=len1 {
            f[0][j] = j;
        }

        for i in 0..=len2 {
            f[i][0] = i;
        }

        for (mut i, ch1) in word2.iter().enumerate() {
            i += 1;
            for (mut j, ch2) in word1.iter().enumerate() {
                j += 1;

                f[i][j] = (f[i - 1][j] + 1).min(f[i][j - 1] + 1);

                if ch1 == ch2 {
                    f[i][j] = f[i][j].min(f[i - 1][j - 1]);
                }
            }
        }

        return f[len2][len1] as i32;
    }
}
