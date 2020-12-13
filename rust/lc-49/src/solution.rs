pub struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();

        for word in strs.iter() {
            let mut buckets = [0u8; 26];
            word.bytes()
                .for_each(|ch| buckets[(ch - b'a') as usize] += 1);
            map.entry(buckets).or_insert(vec![]).push(word.clone());
        }

        map.into_iter().map(|(_, value)| value).collect()
    }
}
