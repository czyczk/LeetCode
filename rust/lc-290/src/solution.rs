pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut word_map = std::collections::HashMap::new();

        let pattern_n = pattern.len();
        let mut words_n = 0;
        for ch in s.as_bytes() {
            if ch == &b' ' {
                words_n += 1;
            }
        }
        words_n += 1;
        if pattern_n != words_n {
            return false;
        }

        let mut words = s.split(' ');

        for pattern_ch in pattern.bytes() {
            let word = format!("{} ", words.next().unwrap());

            match word_map.get(&(pattern_ch as char).to_string()) {
                None => {
                    if word_map.contains_key(&word) {
                        return false;
                    }

                    word_map.insert((pattern_ch as char).to_string(), word.clone());
                    word_map.insert(word, (pattern_ch as char).to_string());
                }
                Some(word_in_pattern) => {
                    if &word != word_in_pattern {
                        return false;
                    }

                    match word_map.get(word_in_pattern) {
                        None => {
                            return false;
                        }
                        Some(look_back) => {
                            if look_back != &(pattern_ch as char).to_string() {
                                return false;
                            }
                        }
                    }
                }
            }
        }

        return true;
    }
}
