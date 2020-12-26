pub struct Solution {}

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let g_n = g.len();
        let s_n = s.len();

        let mut g_idx = 0;
        let mut s_idx = 0;

        while g_idx < g_n && s_idx < s_n {
            if s[s_idx] >= g[g_idx] {
                g_idx += 1;
            }

            s_idx += 1;
        }

        return g_idx as i32;
    }
}