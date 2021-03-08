// https://leetcode-cn.com/problems/palindrome-partitioning-ii/solution/xiang-jie-liang-bian-dong-tai-gui-hua-ji-s5xr/
pub struct Solution {}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();

        // f: Whether s[i, j] is a {p}alindrome
        let mut fp = vec![vec![true; n]; n];

        for i in (0..n).rev() {
            for j in i + 1..n {
                fp[i][j] = fp[i + 1][j - 1] && s[i] == s[j];
            }
        }

        // f: Count of {m}in {c}ut of [0, j]
        let mut fmc = vec![0; n];
        for j in 1..n {
            if fp[0][j] {
                fmc[j] = 0;
            } else {
                fmc[j] = fmc[j - 1] + 1;

                for i in 1..j {
                    if fp[i][j] {
                        fmc[j] = fmc[j].min(fmc[i - 1] + 1);
                    }
                }
            }
        }

        fmc[n - 1] as i32
    }
}
