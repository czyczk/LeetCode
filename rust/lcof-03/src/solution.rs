pub struct Solution {}

impl Solution {
    pub fn find_repeat_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();

        for i in 0..n {
            while i as i32 != nums[i] {
                let num = nums[i];

                if nums[num as usize] == num {
                    return num;
                }

                let temp = nums[num as usize];
                nums[num as usize] = num;
                nums[i] = temp;
            }
        }

        -1
    }
}
