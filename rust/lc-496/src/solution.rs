pub struct Solution {}

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater_map = std::collections::HashMap::with_capacity(nums2.len());
        let mut key_queue = std::collections::VecDeque::new();
        for &num in nums2.iter() {
            while !key_queue.is_empty() && num > *key_queue.back().unwrap() {
                next_greater_map.insert(key_queue.pop_back().unwrap(), num);
            }

            key_queue.push_back(num);
        }

        while !key_queue.is_empty() {
            next_greater_map.insert(key_queue.pop_front().unwrap(), -1);
        }

        nums1
            .into_iter()
            .map(|num| *next_greater_map.get(&num).unwrap())
            .collect()
    }
}
