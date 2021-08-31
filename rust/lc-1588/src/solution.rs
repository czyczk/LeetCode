pub struct Solution {}

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let mut sum = 0;
        let n = arr.len();

        for (i, &num) in arr.iter().enumerate() {
            let left_count = i as i32;
            let right_count = (n - i - 1) as i32;
            let left_odd = (left_count + 1) / 2;
            let right_odd = (right_count + 1) / 2;
            let left_even = left_count / 2 + 1;
            let right_even = right_count / 2 + 1;

            sum += num * (left_odd * right_odd + left_even * right_even);
        }

        sum
    }
}
