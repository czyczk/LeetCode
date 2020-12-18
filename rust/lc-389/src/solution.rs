pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut buckets = [0; 26];

        for ch in s.as_bytes() {
            buckets[(ch - b'a') as usize] += 1;
        }

        let mut result = ' ';

        for ch in t.as_bytes() {
            let idx = (ch - b'a') as usize;
            if buckets[idx] == 0 {
                result = *ch as char;
                break;
            }

            buckets[idx] -= 1;
        }

        result
    }
}

pub struct SolutionOneLiner {}

impl SolutionOneLiner {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.chars()
            .chain(t.chars())
            .fold(0, |xor, c| xor ^ c as u8)
            .into()
    }
}
