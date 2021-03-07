pub struct Solution {}

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.as_bytes();
        let n = s.len();
        let mut f = vec![vec![true; n]; n];
        
        for i in (0..n).rev() {
            for j in i + 1..n {
                f[i][j] = s[i] == s[j] && f[i + 1][j - 1];
            }
        }

        let (ret, _) = dfs(s, 0, n, &f, vec![], vec![]);

        ret
    }
}

fn dfs(s: &[u8], i: usize, n: usize, f: &Vec<Vec<bool>>, mut ret: Vec<Vec<String>>, mut ans: Vec<String>) -> (Vec<Vec<String>>, Vec<String>) {
    if i == n {
        ret.push(ans.clone());
        return (ret, ans);
    }

    for j in i..n {
        if f[i][j] {
            ans.push(String::from_utf8(s[i..j + 1].to_owned()).unwrap());
            let r_ret = dfs(s, j + 1, n, f, ret, ans);
            ret = r_ret.0;
            ans = r_ret.1;
            ans.pop();
        }
    }

    return (ret, ans);
}
