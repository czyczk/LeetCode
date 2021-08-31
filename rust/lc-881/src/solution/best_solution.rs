// Time consumption: O(nlog(n))
pub struct Solution {}

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let mut num_boats = 0;
        let (mut left, mut right) = (0, people.len() - 1);
        while left <= right {
            let weight_left = limit - people[left];

            while left < right && people[right] > weight_left {
                num_boats += 1;
                right -= 1;
            }

            if left < right {
                right -= 1;
            }

            num_boats += 1;
            left += 1;
        }

        num_boats
    }
}
