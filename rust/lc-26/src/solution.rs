pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len == 0 {
            return 0;
        }

        let (mut slow, mut fast) = (1, 1);

        let mut last_visited = nums[0];
        while fast < len {
            if nums[fast] != last_visited {
                let num = nums[fast];
                nums[slow] = num;
                last_visited = num;
                slow += 1;
            }

            fast += 1;
        }

        return slow as i32;
    }
}
