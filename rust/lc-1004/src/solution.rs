pub struct Solution {}

impl Solution {
    pub fn longest_ones(arr: Vec<i32>, k: i32) -> i32 {
        let (mut ret, mut l_sum, mut r_sum, mut left) = (0, 0, 0, 0);
        
        for (right, &num) in arr.iter().enumerate() {
            r_sum += 1 ^ num;

            while r_sum - l_sum > k {
                l_sum += 1 ^ arr[left];
                left += 1;
            }

            ret = ret.max(right - left + 1);
        }

        ret as i32
    }
}
