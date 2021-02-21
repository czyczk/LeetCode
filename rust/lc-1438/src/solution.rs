pub struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut left = 0;
        let (mut min_queue, mut max_queue) = (std::collections::VecDeque::new(), std::collections::VecDeque::new());

        let mut max_window_size = 1;
        for (right, &num) in nums.iter().enumerate() {
            loop {
                match min_queue.back() {
                    Some(min) if *min > num => {
                        min_queue.pop_back();
                    }
                    _ => break,
                }
            }
            min_queue.push_back(num);
            
            loop {
                match max_queue.back() {
                    Some(max) if *max < num => {
                        max_queue.pop_back();
                    }
                    _ => break,
                }
            }
            max_queue.push_back(num);

            while max_queue.front().unwrap() - min_queue.front().unwrap() > limit {
                let left_num = nums[left];
                if max_queue[0] == left_num {
                    max_queue.pop_front();
                }
                if min_queue[0] == left_num {
                    min_queue.pop_front();
                }
                left += 1;
            }

            max_window_size = max_window_size.max(right - left + 1);
        }

        max_window_size as i32
    }
}
