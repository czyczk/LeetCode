pub struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut freq = [0; 26];
        let mut cur_freq = [0; 26];
        let mut check_list = std::collections::HashSet::with_capacity(26);

        for ch in s1.bytes() {
            freq[to_alphabet_idx(ch)] += 1;
        }

        let s2 = s2.into_bytes();
        let n = s2.len();

        let window_len = s1.len();
        if window_len > n {
            return false;
        }

        let mut right = window_len - 1;
        for i in 0..=right {
            let ch = s2[i];
            cur_freq[to_alphabet_idx(ch)] += 1;
        }

        for j in 0..26 {
            if freq[j] != cur_freq[j] {
                check_list.insert(j as u8 + b'a');
            }
        }

        if check_list.is_empty() {
            return true;
        }

        right += 1;

        while right < n {
            let left = right - window_len;
            let ch = s2[left];
            let idx = to_alphabet_idx(ch);
            cur_freq[idx] -= 1;
            if cur_freq[idx] == freq[idx] {
                check_list.remove(&ch);
            }

            let ch = s2[right];
            let idx = to_alphabet_idx(ch);
            cur_freq[idx] += 1;
            match cur_freq[idx] == freq[idx] {
                true => {
                    check_list.remove(&ch);
                }
                false => {
                    check_list.insert(ch);
                }
            }

            if check_list.is_empty() {
                return true;
            }

            right += 1;
        }

        return false;
    }
}

fn to_alphabet_idx(ch: u8) -> usize {
    (ch - b'a') as usize
}
