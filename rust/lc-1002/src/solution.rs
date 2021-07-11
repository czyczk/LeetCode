pub struct Solution {}

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let n = words.len();
        let mut patterns = vec![std::collections::HashMap::new(); n];

        for (i, word) in words.into_iter().enumerate() {
            for ch in word.bytes() {
                *patterns[i].entry(ch).or_insert(0) += 1;
            }
        }

        let mut ret = vec![];
        for i in 'a' as u8..='z' as u8 {
            let min_occurence = patterns.iter().fold(std::u32::MAX, |acc, x| {
                acc.min(x.get(&i).cloned().unwrap_or_default())
            });
            for _ in 0..min_occurence {
                ret.push(String::from_utf8(vec![i]).unwrap());
            }
        }

        ret
    }
}
