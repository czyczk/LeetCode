pub struct SolutionCenter {}

impl SolutionCenter {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut ret = 0;

        for i in 0..2 * n - 1 {
            let mut l = i / 2;
            let mut r = i / 2 + (i & 1);
            while r < n && s[l] == s[r] {
                ret += 1;
                // l >= 0
                match l.checked_sub(1) {
                    Some(num) => l = num,
                    None => break,
                }
                r += 1;
            }
        }

        return ret;
    }
}

pub struct SolutionManacher {}

impl SolutionManacher {
    pub fn count_substrings(s: String) -> i32 {
        let mut s_new = vec![b'$', b'#'];
        for ch in s.bytes() {
            s_new.push(ch);
            s_new.push(b'#');
        }
        s_new.push(b'!');

        let n = s_new.len();
        let s = s_new;
        let mut f = vec![0; n];

        let (mut i_max, mut r_max, mut ret) = (0, 0, 0);

        for i in 1..n - 1 {
            if i <= r_max {
                f[i] = f[2 * i_max - i].min(r_max - i + 1);
            } else {
                f[i] = 1;
            }

            while s[i + f[i]] == s[i - f[i]] {
                f[i] += 1;
            }

            if i + f[i] - 1 > r_max {
                r_max = i + f[i] - 1;
                i_max = i;
            }

            // += Upper bound of (f[i] - 1) / 2
            ret += f[i] / 2;
        }

        ret as i32
    }
}
