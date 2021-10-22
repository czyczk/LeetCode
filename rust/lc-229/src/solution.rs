pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut set = std::collections::HashMap::new();
        let mut ret_set = std::collections::HashSet::with_capacity(2);
        for &num in nums.iter() {
            let times = set.entry(num).or_insert(0_u32);
            *times += 1;
            if *times as usize > n / 3 {
                ret_set.insert(num);
            }
        }

        ret_set.into_iter().collect()
    }
}
