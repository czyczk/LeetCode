pub struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();

        // Both inclusive
        let (mut start, mut end) = (0, 0);
        let mut window_sum = nums[0];
        let mut min_window = std::usize::MAX;

        loop {
            // Expand `end` to find the next satisfying window
            loop {
                if end == 0 && window_sum >= target {
                    break;
                }

                end += 1;
                if end == n {
                    return match min_window {
                        std::usize::MAX => 0,
                        _ => min_window as i32,
                    };
                }

                window_sum += nums[end];
                if window_sum >= target {
                    break;
                }
            }

            // Shrink `start` to minimize the window
            while start < end {
                if window_sum - nums[start] < target {
                    break;
                }

                window_sum -= nums[start];
                start += 1;
            }

            min_window = min_window.min(end - start + 1);

            if min_window == 1 {
                return 1;
            }
        }
    }
}
