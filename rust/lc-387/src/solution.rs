pub struct Solution { }

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        if s.len() == 0 {
            return -1;
        }

        let mut occurences = [0; 26];
        let mut positions = [std::usize::MAX; 26];

        for (i, ch) in s.bytes().enumerate() {
            let idx = (ch - b'a') as usize;
            occurences[idx] += 1;

            if i < positions[idx] {
                positions[idx] = i;
            }
        }

        // Using the `positions` array, more space is used, but the following process takes only O(1) of time.
        // Without the array, we have to iterate the string again.
        let mut position = std::usize::MAX;
        for (i, occurence) in occurences.iter().enumerate() {
            if occurence == &1 && positions[i] < position {
                position = positions[i];
            }
        }

        return position as i32;
    }
}