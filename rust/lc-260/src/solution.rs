pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor_result = 0;
        for &num in nums.iter() {
            xor_result ^= num;
        }

        let mask = xor_result & -xor_result;
        let mut pile1 = 0;
        let mut pile2 = 0;
        for &num in nums.iter() {
            if num & mask == 0 {
                pile1 ^= num;
            } else {
                pile2 ^= num;
            }
        }

        return vec![pile1, pile2];
    }
}
