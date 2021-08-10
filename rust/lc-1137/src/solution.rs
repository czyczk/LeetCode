pub struct Solution {}

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        match n {
            0 => return 0,
            1 | 2 => return 1,
            _ => {}
        }

        let (mut tn, mut tn1, mut tn2) = (1, 1, 0);
        for _ in 3..=n {
            let new_tn = tn + tn1 + tn2;
            tn2 = tn1;
            tn1 = tn;
            tn = new_tn;
        }

        return tn;
    }
}
