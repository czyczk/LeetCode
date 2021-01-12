pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let n = nums.len();
        match n {
            0 => return vec![],
            1 => return vec![nums[0].to_string()],
            _ => {}
        }

        let mut start_num = nums[0];
        let mut end_num = nums[0];
        let mut last_num = nums[0];

        let mut result = vec![];

        for i in 1..n {
            let num = nums[i];
            match num - last_num {
                1 => end_num += 1,
                _ => {
                    if start_num == end_num {
                        result.push(start_num.to_string());
                    } else {
                        result.push(format!("{}->{}", start_num, end_num).to_owned());
                    }

                    start_num = num;
                    end_num = num;
                }
            }

            last_num = num;
        }

        if start_num == end_num {
            result.push(start_num.to_string());
        } else {
            result.push(format!("{}->{}", start_num, end_num).to_owned());
        }

        result
    }
}
