pub struct Solution {}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut freq_map = std::collections::HashMap::new();
        let mut left_map = std::collections::HashMap::new();

        let mut max_freq = 0;
        let mut min_range = std::usize::MAX;

        for (i, &num) in nums.iter().enumerate() {
            let freq = freq_map.entry(num).or_insert(0);
            *freq += 1;

            left_map.entry(num).or_insert(i);

            match (*freq).cmp(&max_freq) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Greater => {
                    max_freq = *freq;
                    min_range = std::usize::MAX;
                }
                _ => {}
            }

            let left = left_map[&num];
            let candidate_range = i - left + 1;
            min_range = min_range.min(candidate_range);
        }

        min_range as i32
    }
}
