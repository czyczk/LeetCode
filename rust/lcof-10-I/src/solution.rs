pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 | 1 => return n,
            _ => {}
        }

        let (mut f0, mut f1) = (0, 1);
        for _ in 2..=n {
            let temp = f1;
            f1 = (f1 + f0) % 1000000007;
            f0 = temp;
        }

        return f1;
    }
}
