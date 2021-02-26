pub struct Solution {}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut word_freq_map = std::collections::HashMap::new();
        for word in words {
            let mut mask = 0u32;
            for ch in word.bytes() {
                mask |= 1u32 << (ch - b'a');
            }

            if mask.count_ones() <= 7 {
                *word_freq_map.entry(mask).or_insert(0) += 1;
            }

        }

        let mut ret = Vec::with_capacity(puzzles.len());
        for puzzle in puzzles {
            let puzzle = puzzle.as_bytes();
            let mut mask = 0u32;
            let mask_first_ch = 1u32 << (puzzle[0] - b'a');

            for ch in puzzle[1..].iter() {
                mask |= 1u32 << (ch - b'a');
            }

            let mut num_matching_words = 0;
            let mut subset = mask;
            loop {
                let word_pattern = subset | mask_first_ch;
                num_matching_words += word_freq_map.get(&word_pattern).unwrap_or(&0);
                subset = subset.overflowing_sub(1u32).0 & mask;
                if subset == mask {
                    break;
                }
            }

            ret.push(num_matching_words);
        }

        ret
    }
}
