pub struct Solution {}

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_streaks = vec![1; n];
        let mut dp_ways = vec![1; n];
        let mut max_streak = 1;

        for i in 0..n {
            for j in 0..i {
                if nums[i] <= nums[j] {
                    continue;
                }

                let streak_candidate = dp_streaks[j] + 1;
                match streak_candidate.cmp(&dp_streaks[i]) {
                    std::cmp::Ordering::Greater => {
                        dp_streaks[i] = streak_candidate;
                        dp_ways[i] = dp_ways[j];
                        max_streak = max_streak.max(streak_candidate);
                    }
                    std::cmp::Ordering::Equal => {
                        dp_streaks[i] = streak_candidate;
                        dp_ways[i] += dp_ways[j];
                        max_streak = max_streak.max(streak_candidate);
                    }
                    _ => {}
                }
            }
        }

        dp_streaks
            .iter()
            .enumerate()
            .filter(|(_, streak)| **streak == max_streak)
            .map(|(i, _)| dp_ways[i])
            .sum()
    }
}
