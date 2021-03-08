pub struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // Construct a new string that looks like "$#o#r#i#!"
        let s = s.as_bytes();
        let mut t = vec![b'$', b'#'];
        for &ch in s.iter() {
            t.push(ch);
            t.push(b'#');
        }
        t.push(b'!');
        let n = t.len();

        // Iterate through the new string using Manacher's algorithm
        let mut f = vec![0; n];
        // r_max is the radius that will contain i;
        // i_max is the center of the radius
        let (mut i_max, mut r_max) = (0, 0);
        // f_ans is the max f value in the string;
        // i_ans is the center of f_ans
        let (mut i_ans, mut f_ans) = (0, 0);

        for i in 1..n - 1 {
            // Reuse the mirror point if possible
            if i <= r_max {
                f[i] = f[2 * i_max - i].min(r_max - i + 1);
            } else {
                f[i] = 1;
            }

            // Based on the reused radius, expand from the center
            while t[i + f[i]] == t[i - f[i]] {
                f[i] += 1;
            }

            // Update the max radius and center
            if f[i] > f_ans {
                i_ans = i;
                f_ans = f[i];
            }

            // Update the max radius that contains the current i
            if i + f[i] - 1 > r_max {
                i_max = i;
                r_max = i + f[i] - 1;
            }
        }
        
        // i_ans will always be even (falling on a meaningful character).
        // i_ans + f_ans - 1 is the maximum span to the right. -1 again to exclude the useless '#' char.
        String::from_utf8((i_ans + 2 - f_ans..=i_ans + f_ans - 2).step_by(2).map(|i| t[i]).collect()).unwrap()
    }
}
