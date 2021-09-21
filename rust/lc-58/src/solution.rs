pub struct Solution {}

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut is_leading_space = true;
        let mut ret = 0;

        for ch in s.bytes().rev() {
            match ch {
                b' ' => {
                    if is_leading_space {
                        continue;
                    }

                    return ret;
                }
                _ => {
                    if is_leading_space {
                        is_leading_space = false;
                    }
                    ret += 1;
                }
            }
        }

        ret
    }
}
