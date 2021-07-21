pub struct Solution {}

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let n = nums.len();
        let (first_len, second_len) = (first_len as usize, second_len as usize);
        let (
            mut first_left_sums,
            mut second_left_sums,
            mut first_right_sums,
            mut second_right_sums,
        ) = (vec![0; n], vec![0; n], vec![0; n], vec![0; n]);

        let (mut first_left_sum, mut second_left_sum) = (0, 0);
        let (mut first_right_sum, mut second_right_sum) = (
            nums[0..first_len - 1].iter().sum(),
            nums[0..second_len - 1].iter().sum(),
        );

        for (i, &num) in nums.iter().enumerate() {
            let (first_left_start_idx, is_first_left_overflow) = (i + 1).overflowing_sub(first_len);
            first_left_sum += num;
            if !is_first_left_overflow {
                if first_left_start_idx > 0 {
                    first_left_sum -= nums[first_left_start_idx - 1];
                }
                first_left_sums[i] = first_left_sum;
            }

            let (second_left_start_idx, is_second_left_overflow) =
                (i + 1).overflowing_sub(second_len);
            second_left_sum += num;
            if !is_second_left_overflow {
                if second_left_start_idx > 0 {
                    second_left_sum -= nums[second_left_start_idx - 1];
                }
                second_left_sums[i] = second_left_sum;
            }

            let first_right_end_idx = i + first_len - 1; // inclusive
            if first_right_end_idx < n {
                first_right_sum += nums[first_right_end_idx];
                if i > 0 {
                    first_right_sum -= nums[i - 1];
                }
                first_right_sums[i] = first_right_sum;
            }

            let second_right_end_idx = i + second_len - 1; // inclusive
            if second_right_end_idx < n {
                second_right_sum += nums[second_right_end_idx];
                if i > 0 {
                    second_right_sum -= nums[i - 1];
                }
                second_right_sums[i] = second_right_sum;
            }
        }

        for i in 1..n {
            first_left_sums[i] = first_left_sums[i].max(first_left_sums[i - 1]);
            second_left_sums[i] = second_left_sums[i].max(second_left_sums[i - 1]);
        }

        for i in (n - 2..=0).rev() {
            first_right_sums[i] = first_right_sums[i].max(first_right_sums[i + 1]);
            second_right_sums[i] = second_right_sums[i].max(second_right_sums[i + 1]);
        }

        let mut ret = 0;
        for i in 1..n {
            ret = ret
                .max(first_left_sums[i - 1] + second_right_sums[i])
                .max(second_left_sums[i - 1] + first_right_sums[i]);
        }

        ret
    }
}
