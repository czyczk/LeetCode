pub struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ret = Vec::with_capacity(n as usize);
        for i in 1..=n {
            let mod3 = i % 3;
            let mod5 = i % 5;
            if mod3 == 0 && mod5 == 0 {
                ret.push("FizzBuzz".to_owned());
            } else if mod3 == 0 {
                ret.push("Fizz".to_owned());
            } else if mod5 == 0 {
                ret.push("Buzz".to_owned());
            } else {
                ret.push(i.to_string());
            }
        }

        ret
    }
}
