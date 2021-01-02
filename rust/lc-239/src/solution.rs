pub struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();

        let mut idx_list = std::collections::VecDeque::new();
        let mut result = vec![0; n - k + 1];

        for (i, &num) in nums.iter().enumerate() {
            /*
             * 1. Push back the current index if 1.1 || 1.2.
             *   1.1 idx_list is empty.
             *   1.2 the number indicated by the last in idx_list is smaller than the current one.
             * 2. Pop all the numbers <= the current one and add the index.
             */
            while !idx_list.is_empty() && num >= nums[*idx_list.back().unwrap()] {
                idx_list.pop_back();
            }
            idx_list.push_back(i);

            // Just move ahead if not enough numbers have been iterated, i.e. a window is not formed.
            if i < k - 1 {
                continue;
            }

            // Check if the head in idx_list is in the window range. Pop the head if it's out of range.
            let head = idx_list.front().unwrap();
            // head < l
            if *head as i32 <= i as i32 - k as i32 {
                idx_list.pop_front();
            }

            // Now the head indicates the index of the max number within the window.
            // result[l]
            result[i + 1 - k] = nums[*idx_list.front().unwrap()];
        }

        result
    }
}
