pub struct Solution {}

impl Solution {
    pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }

        if m > n {
            std::mem::swap(&mut m, &mut n);
        }

        let mut ans: i64 = 1;
        let (mut i, mut j) = (n as i64, 1);
        while j < m as i64 {
            ans = ans * i / j;
            i += 1;
            j += 1;
        }

        return ans as i32;
    }
}
