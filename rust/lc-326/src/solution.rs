pub struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        match n {
            0 => return false,
            1 => return true,
            other if other % 3 != 0 => return false,
            _ => {}
        }

        let mut num = 3;
        while num <= n {
            if n == num {
                return true;
            }

            num *= 3;
        }

        return false;
    }
}
