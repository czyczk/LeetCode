pub struct Solution {}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut freq_map = std::collections::HashMap::new();
        for &num in nums.iter() {
            *freq_map.entry(num).or_insert(0) += 1;
        }

        let mut min_heap = std::collections::BinaryHeap::new();
        for (&key, &val) in freq_map.iter() {
            min_heap.push(Pair { k: key, v: val });

            if min_heap.len() > k {
                min_heap.pop();
            }
        }

        min_heap.iter().map(|pair| pair.k).collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pair {
    k: i32,
    v: i32,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.v.cmp(&self.v)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
