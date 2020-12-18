pub struct Solution {}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut result = vec![];
        let mut left = [0 as u16; 26];
        let mut used = [false; 26];

        for ch in s.bytes() {
            let ch_idx = (ch - b'a') as usize;
            left[ch_idx] += 1;
        }

        for ch in s.bytes() {
            let ch_idx = (ch - b'a') as usize;
            if used[ch_idx] {
                left[ch_idx] -= 1;
                continue;
            }

            loop {
                match result.last() {
                    None => break,
                    Some(last_ch) if last_ch < &ch => break,
                    Some(last_ch) => {
                        let last_ch_idx = (last_ch - b'a') as usize;
                        if left[last_ch_idx] > 0 {
                            result.remove(result.len() - 1);
                            used[last_ch_idx] = false;
                        } else {
                            break;
                        }
                    }
                }
            }

            left[ch_idx] -= 1;

            if !used[ch_idx] {
                result.push(ch);
                used[ch_idx] = true;
            }
        }

        result.into_iter().map(|ch_u8| ch_u8 as char).collect()
    }
}
