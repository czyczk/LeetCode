pub struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = std::collections::HashSet::new();
        for &num in nums.iter() {
            set.insert(num);
        }

        let mut current_streak;
        let mut longest_streak = 0;
        for &num in set.iter() {
            if set.contains(&(num - 1)) {
                current_streak = 1;
                longest_streak = longest_streak.max(current_streak);
                continue;
            }

            current_streak = 1;
            let mut k = num + 1;
            loop {
                if !set.contains(&k) {
                    break;
                }

                current_streak += 1;
                k += 1;
            }

            longest_streak = longest_streak.max(current_streak);
        }

        longest_streak
    }
}
