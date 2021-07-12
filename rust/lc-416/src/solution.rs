pub struct Solution2DArray {}

impl Solution2DArray {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = (nums.iter().sum::<i32>()) as usize;
        if sum % 2 != 0 {
            return false;
        }

        let mut dp = vec![vec![0; (sum / 2) + 1]; nums.len()];

        for j in nums[0] as usize..=sum / 2 {
            dp[0][j] = nums[0];
        }

        for i in 1..nums.len() {
            for j in 1..(nums[i] as usize).min(sum / 2) {
                dp[i][j] = dp[i - 1][j];
            }
            for j in nums[i] as usize..=sum / 2 {
                dp[i][j] = dp[i - 1][j].max(dp[i - 1][j - nums[i] as usize] + nums[i]);

                if j == sum / 2 && dp[i][j] == (sum / 2) as i32 {
                    return true;
                }
            }
        }

        return false;
    }
}

pub struct Solution {}

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }

        let half = sum / 2;
        let mut dp = vec![0; half as usize + 1];
        for j in nums[0] as usize..=half as usize {
            dp[j] = nums[0];
        }

        for i in 1..nums.len() {
            for j in (nums[i] as usize..=half as usize).rev() {
                dp[j] = dp[j].max(dp[j - (nums[i] as usize)] + nums[i]);
            }

            if dp[half as usize] == half {
                return true;
            }
        }

        false
    }
}
