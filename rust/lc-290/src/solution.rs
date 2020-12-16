pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut word_map = std::collections::HashMap::new();

        let mut words = s.split(' ');

        for pattern_ch in pattern.bytes() {
            let word = format!("{} ", words.next().unwrap());

            match word_map.get(&pattern_ch.to_string()) {
                None => {
                    if word_map.contains_key(&word) {
                        return false;
                    }

                    word_map.insert(pattern_ch.to_string(), word.clone());
                    word_map.insert(word, pattern_ch.to_string());
                }
                Some(word_in_pattern) => match word_map.get(word_in_pattern) {
                    None => return false,
                    Some(look_back) => {
                        if look_back != &pattern_ch.to_string() {
                            return false;
                        }
                    }
                },
            }
        }

        return true;
    }
}
