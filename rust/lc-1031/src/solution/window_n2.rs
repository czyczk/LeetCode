pub struct Solution {}

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let n = nums.len();
        let (first_len, second_len) = (first_len as usize, second_len as usize);

        let first_range = n - first_len + 1;
        let mut first_sums = vec![0; first_range];
        first_sums[0] = nums[0..first_len].iter().sum();
        let second_range = n - second_len + 1;
        let mut second_sums = vec![0; second_range];
        second_sums[0] = nums[0..second_len].iter().sum();

        for i in 1..n {
            let first_end_idx = i + first_len - 1; // inclusive
            let second_end_idx = i + second_len - 1; // inclusive
            let is_first_in_range = first_end_idx < n;
            let is_second_in_range = second_end_idx < n;

            if !is_first_in_range && !is_second_in_range {
                break;
            }

            if is_first_in_range {
                first_sums[i] = first_sums[i - 1] - nums[i - 1] + nums[first_end_idx];
            }
            if is_second_in_range {
                second_sums[i] = second_sums[i - 1] - nums[i - 1] + nums[second_end_idx];
            }
        }

        let mut max_sum = 0;
        let shorter_len = first_len.min(second_len);
        for i in shorter_len..n + 1 - shorter_len {
            // first: [0..i], second: [i..n]
            {
                let (first_end_idx, is_first_overflow) = i.overflowing_sub(first_len);
                let is_second_overflow = i >= second_range;
                if !is_first_overflow && !is_second_overflow {
                    for j in 0..=first_end_idx {
                        for k in i..second_range {
                            max_sum = max_sum.max(first_sums[j] + second_sums[k]);
                        }
                    }
                }
            }

            // first: [i..n], second: [0..i]
            {
                let is_first_overflow = i >= first_range;
                let (second_end_idx, is_second_overflow) = i.overflowing_sub(second_len);
                if !is_first_overflow && !is_second_overflow {
                    for j in i..first_range {
                        for k in 0..=second_end_idx {
                            max_sum = max_sum.max(first_sums[j] + second_sums[k]);
                        }
                    }
                }
            }
        }

        return max_sum;
    }
}
