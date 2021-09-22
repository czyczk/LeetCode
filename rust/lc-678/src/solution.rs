pub struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut max_offset = 0;
        let mut min_offset = 0;

        for ch in s.bytes() {
            match ch {
                b'(' => {
                    max_offset += 1;
                    min_offset += 1;
                }
                b')' => {
                    if max_offset == 0 {
                        return false;
                    }

                    max_offset -= 1;
                    min_offset = (min_offset - 1).max(0);
                }
                _ => {
                    // *
                    max_offset += 1;
                    min_offset = (min_offset - 1).max(0);
                }
            }
        }

        min_offset == 0
    }
}
