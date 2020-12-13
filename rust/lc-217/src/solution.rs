pub struct Solution { }

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();

        for num in nums.iter() {
            if set.contains(num) {
                return true;
            }

            set.insert(num);
        }
        
        false
    }
}